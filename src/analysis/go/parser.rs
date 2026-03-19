use std::collections::BTreeSet;
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use tree_sitter::{Node, Parser};

use crate::analysis::go::fingerprint::build_function_fingerprint;
use crate::analysis::{
    CallSite, ContextFactoryCall, DbQueryCall, DeclaredSymbol, FormattedErrorCall, ImportSpec,
    ParsedFile, ParsedFunction,
};
use crate::model::SymbolKind;

pub(super) fn parse_file(path: &Path, source: &str) -> Result<ParsedFile> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_go::LANGUAGE.into())
        .map_err(|error| anyhow!(error.to_string()))
        .context("failed to configure Go parser")?;

    let tree = parser
        .parse(source, None)
        .ok_or_else(|| anyhow!("tree-sitter returned no parse tree"))?;

    let root = tree.root_node();
    let package_name = find_package_name(root, source);
    let imports = collect_imports(root, source);
    let symbols = collect_symbols(root, source);
    let functions = collect_functions(root, source, &imports);

    Ok(ParsedFile {
        path: path.to_path_buf(),
        package_name,
        syntax_error: root.has_error(),
        byte_size: source.len(),
        functions,
        imports,
        symbols,
    })
}

fn collect_functions(root: Node<'_>, source: &str, imports: &[ImportSpec]) -> Vec<ParsedFunction> {
    let mut functions = Vec::new();
    visit_for_functions(root, source, imports, &mut functions);
    functions.sort_by(|left, right| {
        left.fingerprint
            .start_line
            .cmp(&right.fingerprint.start_line)
            .then(left.fingerprint.name.cmp(&right.fingerprint.name))
    });
    functions
}

fn visit_for_functions(
    node: Node<'_>,
    source: &str,
    imports: &[ImportSpec],
    functions: &mut Vec<ParsedFunction>,
) {
    if matches!(node.kind(), "function_declaration" | "method_declaration") {
        if let Some(parsed_function) = parse_function_node(node, source, imports) {
            functions.push(parsed_function);
        }
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_functions(child, source, imports, functions);
    }
}

fn parse_function_node(
    node: Node<'_>,
    source: &str,
    imports: &[ImportSpec],
) -> Option<ParsedFunction> {
    let body_node = node.child_by_field_name("body")?;
    let calls = collect_calls(body_node, source);
    let type_assertion_count = count_descendants(body_node, "type_assertion_expression");
    let has_context_parameter = function_has_context_parameter(node, source, imports);
    let doc_comment = extract_doc_comment(source, node.start_position().row);
    let dropped_error_lines = collect_dropped_error_lines(body_node, source);
    let panic_on_error_lines = collect_panic_on_error_lines(body_node, source);
    let errorf_calls = collect_errorf_calls(body_node, source);
    let context_factory_calls = collect_context_factory_calls(body_node, source, imports);
    let goroutine_launch_lines = collect_goroutine_launch_lines(body_node);
    let goroutine_without_shutdown_lines = collect_goroutine_without_shutdown_lines(body_node, source);
    let sleep_in_loop_lines = collect_sleep_in_loop_lines(body_node, source, imports);
    let busy_wait_lines = collect_busy_wait_lines(body_node, source);
    let mutex_lock_in_loop_lines = collect_mutex_lock_in_loop_lines(body_node, source);
    let allocation_in_loop_lines = collect_allocation_in_loop_lines(body_node, source, imports);
    let fmt_in_loop_lines = collect_fmt_in_loop_lines(body_node, source, imports);
    let reflection_in_loop_lines = collect_reflection_in_loop_lines(body_node, source, imports);
    let string_concat_in_loop_lines = collect_string_concat_in_loop_lines(body_node, source);
    let json_marshal_in_loop_lines = collect_json_marshal_in_loop_lines(body_node, source, imports);
    let db_query_calls = collect_db_query_calls(body_node, source);
    let receiver_type = node
        .child_by_field_name("receiver")
        .and_then(|receiver| extract_receiver_type(receiver, source));
    let fingerprint = build_function_fingerprint(
        node,
        source,
        receiver_type,
        type_assertion_count,
        calls.len(),
    )?;

    Some(ParsedFunction {
        fingerprint,
        calls,
        has_context_parameter,
        doc_comment,
        dropped_error_lines,
        panic_on_error_lines,
        errorf_calls,
        context_factory_calls,
        goroutine_launch_lines,
        goroutine_without_shutdown_lines,
        sleep_in_loop_lines,
        busy_wait_lines,
        mutex_lock_in_loop_lines,
        allocation_in_loop_lines,
        fmt_in_loop_lines,
        reflection_in_loop_lines,
        string_concat_in_loop_lines,
        json_marshal_in_loop_lines,
        db_query_calls,
    })
}

fn collect_calls(body_node: Node<'_>, source: &str) -> Vec<CallSite> {
    let mut calls = Vec::new();
    visit_for_calls(body_node, source, &mut calls);
    calls
}

fn visit_for_calls(node: Node<'_>, source: &str, calls: &mut Vec<CallSite>) {
    if node.kind() == "call_expression" {
        if let Some(function_node) = node.child_by_field_name("function") {
            if let Some((receiver, name)) = extract_call_target(function_node, source) {
                calls.push(CallSite {
                    receiver,
                    name,
                    line: node.start_position().row + 1,
                });
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_calls(child, source, calls);
    }
}

fn extract_call_target(function_node: Node<'_>, source: &str) -> Option<(Option<String>, String)> {
    let text = source.get(function_node.byte_range())?.trim();
    if text.is_empty() {
        return None;
    }

    if let Some((receiver, name)) = text.rsplit_once('.') {
        return Some((Some(receiver.trim().to_string()), name.trim().to_string()));
    }

    Some((None, text.to_string()))
}

fn collect_imports(root: Node<'_>, source: &str) -> Vec<ImportSpec> {
    let mut imports = Vec::new();
    visit_for_imports(root, source, &mut imports);
    imports.sort_by(|left, right| {
        left.alias
            .cmp(&right.alias)
            .then(left.path.cmp(&right.path))
    });
    imports
}

fn visit_for_imports(node: Node<'_>, source: &str, imports: &mut Vec<ImportSpec>) {
    if node.kind() == "import_spec" {
        if let Some(import_spec) = parse_import_spec(node, source) {
            imports.push(import_spec);
        }
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_imports(child, source, imports);
    }
}

fn parse_import_spec(node: Node<'_>, source: &str) -> Option<ImportSpec> {
    let text = source.get(node.byte_range())?.trim();
    let mut parts = text.split_whitespace().collect::<Vec<_>>();
    let path_literal = parts.pop()?;
    let path = path_literal.trim_matches('"').to_string();
    let alias = parts
        .first()
        .map(|alias| alias.to_string())
        .unwrap_or_else(|| package_alias_from_import_path(&path));

    Some(ImportSpec { alias, path })
}

fn collect_symbols(root: Node<'_>, source: &str) -> Vec<DeclaredSymbol> {
    let mut symbols = Vec::new();
    visit_for_symbols(root, source, &mut symbols);
    symbols.sort_by(|left, right| left.line.cmp(&right.line).then(left.name.cmp(&right.name)));
    symbols
}

fn visit_for_symbols(node: Node<'_>, source: &str, symbols: &mut Vec<DeclaredSymbol>) {
    match node.kind() {
        "function_declaration" => {
            if let Some(symbol) = parse_function_symbol(node, source) {
                symbols.push(symbol);
            }
        }
        "method_declaration" => {
            if let Some(symbol) = parse_method_symbol(node, source) {
                symbols.push(symbol);
            }
        }
        "type_spec" => {
            if let Some(symbol) = parse_type_symbol(node, source) {
                symbols.push(symbol);
            }
        }
        "var_spec" => {
            symbols.extend(parse_package_var_symbols(node, source));
        }
        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_symbols(child, source, symbols);
    }
}

fn parse_package_var_symbols(node: Node<'_>, source: &str) -> Vec<DeclaredSymbol> {
    if !is_package_scope(node) {
        return Vec::new();
    }

    let Some(name_node) = find_var_name_node(node) else {
        return Vec::new();
    };
    let names = collect_identifiers(name_node, source);
    if names.is_empty() {
        return Vec::new();
    }

    let is_function_typed = node
        .child_by_field_name("type")
        .is_some_and(|type_node| type_node.kind() == "function_type");

    if is_function_typed {
        return names
            .into_iter()
            .map(|(name, line)| DeclaredSymbol {
                name,
                kind: SymbolKind::Function,
                receiver_type: None,
                line,
            })
            .collect();
    }

    let Some(value_node) = find_var_value_node(node) else {
        return Vec::new();
    };
    let values = collect_expression_nodes(value_node);

    names
        .into_iter()
        .enumerate()
        .filter_map(|(index, (name, line))| {
            let value = values.get(index)?;
            is_callable_var_value(*value).then_some(DeclaredSymbol {
                name,
                kind: SymbolKind::Function,
                receiver_type: None,
                line,
            })
        })
        .collect()
}

fn is_package_scope(node: Node<'_>) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        match parent.kind() {
            "function_declaration" | "method_declaration" | "func_literal" => return false,
            "source_file" => return true,
            _ => current = parent.parent(),
        }
    }

    false
}

fn find_var_name_node(node: Node<'_>) -> Option<Node<'_>> {
    node.child_by_field_name("name")
        .or_else(|| first_named_child_of_kind(node, "identifier_list"))
        .or_else(|| first_named_child_of_kind(node, "identifier"))
}

fn find_var_value_node(node: Node<'_>) -> Option<Node<'_>> {
    node.child_by_field_name("value")
        .or_else(|| first_named_child_of_kind(node, "expression_list"))
        .or_else(|| {
            let mut cursor = node.walk();
            node.named_children(&mut cursor)
                .find(|child| is_expression_node_kind(child.kind()))
        })
}

fn first_named_child_of_kind<'tree>(node: Node<'tree>, kind: &str) -> Option<Node<'tree>> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .find(|child| child.kind() == kind)
}

fn collect_identifiers(node: Node<'_>, source: &str) -> Vec<(String, usize)> {
    if node.kind() == "identifier" {
        return source
            .get(node.byte_range())
            .map(|name| vec![(name.to_string(), node.start_position().row + 1)])
            .unwrap_or_default();
    }

    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .filter(|child| child.kind() == "identifier")
        .filter_map(|child| {
            source
                .get(child.byte_range())
                .map(|name| (name.to_string(), child.start_position().row + 1))
        })
        .collect()
}

fn collect_expression_nodes(node: Node<'_>) -> Vec<Node<'_>> {
    if node.kind() != "expression_list" {
        return vec![node];
    }

    let mut cursor = node.walk();
    node.named_children(&mut cursor).collect()
}

fn is_callable_var_value(node: Node<'_>) -> bool {
    matches!(
        node.kind(),
        "identifier"
            | "selector_expression"
            | "func_literal"
            | "parenthesized_expression"
            | "index_expression"
            | "slice_expression"
    )
}

fn is_expression_node_kind(kind: &str) -> bool {
    matches!(
        kind,
        "identifier"
            | "selector_expression"
            | "func_literal"
            | "parenthesized_expression"
            | "call_expression"
            | "unary_expression"
            | "binary_expression"
            | "index_expression"
            | "slice_expression"
            | "type_assertion_expression"
            | "composite_literal"
            | "literal_value"
            | "int_literal"
            | "float_literal"
            | "imaginary_literal"
            | "rune_literal"
            | "raw_string_literal"
            | "interpreted_string_literal"
    )
}

fn parse_function_symbol(node: Node<'_>, source: &str) -> Option<DeclaredSymbol> {
    let name_node = node.child_by_field_name("name")?;
    Some(DeclaredSymbol {
        name: source.get(name_node.byte_range())?.to_string(),
        kind: SymbolKind::Function,
        receiver_type: None,
        line: node.start_position().row + 1,
    })
}

fn parse_method_symbol(node: Node<'_>, source: &str) -> Option<DeclaredSymbol> {
    let name_node = node.child_by_field_name("name")?;
    let receiver_type = node
        .child_by_field_name("receiver")
        .and_then(|receiver| extract_receiver_type(receiver, source));
    Some(DeclaredSymbol {
        name: source.get(name_node.byte_range())?.to_string(),
        kind: SymbolKind::Method,
        receiver_type,
        line: node.start_position().row + 1,
    })
}

fn parse_type_symbol(node: Node<'_>, source: &str) -> Option<DeclaredSymbol> {
    let name_node = node.child_by_field_name("name")?;
    let type_node = node.child_by_field_name("type")?;
    let kind = match type_node.kind() {
        "struct_type" => SymbolKind::Struct,
        "interface_type" => SymbolKind::Interface,
        _ => SymbolKind::Type,
    };

    Some(DeclaredSymbol {
        name: source.get(name_node.byte_range())?.to_string(),
        kind,
        receiver_type: None,
        line: node.start_position().row + 1,
    })
}

fn find_package_name(root: Node<'_>, source: &str) -> Option<String> {
    let mut cursor = root.walk();
    for child in root.named_children(&mut cursor) {
        if child.kind() != "package_clause" {
            continue;
        }

        let mut package_cursor = child.walk();
        for package_child in child.named_children(&mut package_cursor) {
            if package_child.kind() == "package_identifier" || package_child.kind() == "identifier"
            {
                return source
                    .get(package_child.byte_range())
                    .map(ToOwned::to_owned);
            }
        }
    }

    None
}

fn extract_receiver_type(receiver_node: Node<'_>, source: &str) -> Option<String> {
    let text = source.get(receiver_node.byte_range())?;
    let sanitized = text
        .chars()
        .filter(|character| !matches!(character, '(' | ')' | '*' | ','))
        .collect::<String>();
    sanitized
        .split_whitespace()
        .last()
        .map(|receiver| receiver.to_string())
}

fn package_alias_from_import_path(path: &str) -> String {
    path.rsplit('/').next().unwrap_or(path).to_string()
}

fn count_descendants(node: Node<'_>, kind: &str) -> usize {
    let mut total = usize::from(node.kind() == kind);
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        total += count_descendants(child, kind);
    }
    total
}

fn function_has_context_parameter(node: Node<'_>, source: &str, imports: &[ImportSpec]) -> bool {
    let Some(parameters_node) = node.child_by_field_name("parameters") else {
        return false;
    };
    let Some(parameters_text) = source.get(parameters_node.byte_range()) else {
        return false;
    };

    imports
        .iter()
        .filter(|import| import.path == "context")
        .any(|import| parameters_text.contains(&format!("{}.Context", import.alias)))
}

fn collect_sleep_in_loop_lines(
    body_node: Node<'_>,
    source: &str,
    imports: &[ImportSpec],
) -> Vec<usize> {
    let mut lines = Vec::new();
    visit_for_sleep_in_loop(body_node, source, imports, false, &mut lines);
    lines
}

fn visit_for_sleep_in_loop(
    node: Node<'_>,
    source: &str,
    imports: &[ImportSpec],
    inside_loop: bool,
    lines: &mut Vec<usize>,
) {
    let next_inside_loop = inside_loop || node.kind() == "for_statement";

    if next_inside_loop && node.kind() == "call_expression" {
        let function_node = node.child_by_field_name("function");
        if let Some(function_node) = function_node {
            let target = source.get(function_node.byte_range()).unwrap_or("").trim();
            if is_time_sleep_call(target, imports) {
                lines.push(node.start_position().row + 1);
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_sleep_in_loop(child, source, imports, next_inside_loop, lines);
    }
}

fn is_time_sleep_call(target: &str, imports: &[ImportSpec]) -> bool {
    imports
        .iter()
        .filter(|import| import.path == "time")
        .any(|import| target == format!("{}.Sleep", import.alias))
}

fn collect_busy_wait_lines(body_node: Node<'_>, source: &str) -> Vec<usize> {
    let mut lines = Vec::new();
    visit_for_busy_wait(body_node, source, false, &mut lines);
    lines
}

fn visit_for_busy_wait(node: Node<'_>, source: &str, inside_loop: bool, lines: &mut Vec<usize>) {
    let next_inside_loop = inside_loop || node.kind() == "for_statement";

    if next_inside_loop
        && node.kind() == "select_statement"
        && source
            .get(node.byte_range())
            .is_some_and(|text| text.contains("default:"))
    {
        lines.push(node.start_position().row + 1);
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_busy_wait(child, source, next_inside_loop, lines);
    }
}

fn collect_context_factory_calls(
    body_node: Node<'_>,
    source: &str,
    imports: &[ImportSpec],
) -> Vec<ContextFactoryCall> {
    let mut calls = Vec::new();
    visit_for_context_factory_calls(body_node, source, imports, &mut calls);
    calls
}

fn visit_for_context_factory_calls(
    node: Node<'_>,
    source: &str,
    imports: &[ImportSpec],
    calls: &mut Vec<ContextFactoryCall>,
) {
    if matches!(node.kind(), "assignment_statement" | "short_var_declaration" | "var_spec") {
        if let Some(call) = parse_context_factory_call(node, source, imports) {
            calls.push(call);
        }
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_context_factory_calls(child, source, imports, calls);
    }
}

fn parse_context_factory_call(
    node: Node<'_>,
    source: &str,
    imports: &[ImportSpec],
) -> Option<ContextFactoryCall> {
    let text = source.get(node.byte_range())?;
    let (left, right) = split_assignment(text)?;
    let factory_name = context_factory_name(right, imports)?;
    let cancel_name = extract_cancel_name(left)?;

    Some(ContextFactoryCall {
        line: node.start_position().row + 1,
        cancel_name,
        factory_name,
    })
}

fn context_factory_name(text: &str, imports: &[ImportSpec]) -> Option<String> {
    let compact = text.split_whitespace().collect::<String>();

    for import in imports.iter().filter(|import| import.path == "context") {
        for factory_name in ["WithCancel", "WithTimeout", "WithDeadline"] {
            let prefix = format!("{}.{}(", import.alias, factory_name);
            if compact.starts_with(&prefix) {
                return Some(factory_name.to_string());
            }
        }
    }

    None
}

fn extract_cancel_name(left: &str) -> Option<String> {
    let candidate = left.rsplit(',').next()?.trim();
    let cancel_name = candidate.split_whitespace().last()?;
    if cancel_name == "_" || !is_identifier_name(cancel_name) {
        return None;
    }

    Some(cancel_name.to_string())
}

fn collect_goroutine_launch_lines(body_node: Node<'_>) -> Vec<usize> {
    let mut lines = Vec::new();
    visit_for_goroutine_launches(body_node, &mut lines);
    lines
}

fn collect_goroutine_without_shutdown_lines(body_node: Node<'_>, source: &str) -> Vec<usize> {
    let mut lines = Vec::new();
    visit_for_goroutines_without_shutdown(body_node, source, &mut lines);
    lines
}

fn visit_for_goroutines_without_shutdown(node: Node<'_>, source: &str, lines: &mut Vec<usize>) {
    if node.kind() == "go_statement"
        && source.get(node.byte_range()).is_some_and(|text| {
            let compact = text.split_whitespace().collect::<String>();
            let has_func_literal = compact.contains("gofunc(") || compact.contains("gofunc()");
            let has_loop = count_descendants(node, "for_statement") > 0;
            let has_shutdown_signal = compact.contains("ctx.Done()")
                || compact.contains("<-done")
                || compact.contains("<-shutdown")
                || compact.contains("case<-done")
                || compact.contains("case<-shutdown")
                || compact.contains("case<-ctx.Done()");
            has_func_literal && has_loop && !has_shutdown_signal
        })
    {
        lines.push(node.start_position().row + 1);
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_goroutines_without_shutdown(child, source, lines);
    }
}

fn visit_for_goroutine_launches(node: Node<'_>, lines: &mut Vec<usize>) {
    if node.kind() == "go_statement" {
        lines.push(node.start_position().row + 1);
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_goroutine_launches(child, lines);
    }
}

fn collect_string_concat_in_loop_lines(body_node: Node<'_>, source: &str) -> Vec<usize> {
    let string_variables = collect_explicit_string_variables(body_node, source);
    let mut lines = Vec::new();
    visit_for_string_concat_in_loop(body_node, source, &string_variables, false, &mut lines);
    lines
}

fn collect_mutex_lock_in_loop_lines(body_node: Node<'_>, source: &str) -> Vec<usize> {
    let mut lines = Vec::new();
    visit_for_mutex_lock_in_loop(body_node, source, false, &mut lines);
    lines
}

fn visit_for_mutex_lock_in_loop(
    node: Node<'_>,
    source: &str,
    inside_loop: bool,
    lines: &mut Vec<usize>,
) {
    let next_inside_loop = inside_loop || node.kind() == "for_statement";

    if next_inside_loop && node.kind() == "call_expression" {
        let function_node = node.child_by_field_name("function");
        if let Some(function_node) = function_node {
            let target = source.get(function_node.byte_range()).unwrap_or("").trim();
            if is_mutex_lock_call(target) {
                lines.push(node.start_position().row + 1);
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_mutex_lock_in_loop(child, source, next_inside_loop, lines);
    }
}

fn is_mutex_lock_call(target: &str) -> bool {
    target.ends_with(".Lock") || target.ends_with(".RLock")
}

fn collect_allocation_in_loop_lines(
    body_node: Node<'_>,
    source: &str,
    imports: &[ImportSpec],
) -> Vec<usize> {
    let mut lines = Vec::new();
    visit_for_allocation_in_loop(body_node, source, imports, false, &mut lines);
    lines
}

fn visit_for_allocation_in_loop(
    node: Node<'_>,
    source: &str,
    imports: &[ImportSpec],
    inside_loop: bool,
    lines: &mut Vec<usize>,
) {
    let next_inside_loop = inside_loop || node.kind() == "for_statement";

    if next_inside_loop && node.kind() == "call_expression" {
        let function_node = node.child_by_field_name("function");
        if let Some(function_node) = function_node {
            let target = source.get(function_node.byte_range()).unwrap_or("").trim();
            if is_allocation_call(target, imports) {
                lines.push(node.start_position().row + 1);
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_allocation_in_loop(child, source, imports, next_inside_loop, lines);
    }
}

fn is_allocation_call(target: &str, imports: &[ImportSpec]) -> bool {
    if matches!(target, "make" | "new") {
        return true;
    }

    imports.iter().any(|import| {
        matches!(import.path.as_str(), "bytes")
            && (target == format!("{}.NewBuffer", import.alias)
                || target == format!("{}.NewBufferString", import.alias))
    })
}

fn collect_fmt_in_loop_lines(body_node: Node<'_>, source: &str, imports: &[ImportSpec]) -> Vec<usize> {
    let mut lines = Vec::new();
    visit_for_fmt_in_loop(body_node, source, imports, false, &mut lines);
    lines
}

fn visit_for_fmt_in_loop(
    node: Node<'_>,
    source: &str,
    imports: &[ImportSpec],
    inside_loop: bool,
    lines: &mut Vec<usize>,
) {
    let next_inside_loop = inside_loop || node.kind() == "for_statement";

    if next_inside_loop && node.kind() == "call_expression" {
        let function_node = node.child_by_field_name("function");
        if let Some(function_node) = function_node {
            let target = source.get(function_node.byte_range()).unwrap_or("").trim();
            if is_fmt_hot_path_call(target, imports) {
                lines.push(node.start_position().row + 1);
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_fmt_in_loop(child, source, imports, next_inside_loop, lines);
    }
}

fn is_fmt_hot_path_call(target: &str, imports: &[ImportSpec]) -> bool {
    imports.iter().any(|import| {
        import.path == "fmt"
            && ["Sprintf", "Sprint", "Sprintln", "Fprintf", "Fprint", "Fprintln"]
                .iter()
                .any(|name| target == format!("{}.{}", import.alias, name))
    })
}

fn collect_reflection_in_loop_lines(
    body_node: Node<'_>,
    source: &str,
    imports: &[ImportSpec],
) -> Vec<usize> {
    let mut lines = Vec::new();
    visit_for_reflection_in_loop(body_node, source, imports, false, &mut lines);
    lines
}

fn visit_for_reflection_in_loop(
    node: Node<'_>,
    source: &str,
    imports: &[ImportSpec],
    inside_loop: bool,
    lines: &mut Vec<usize>,
) {
    let next_inside_loop = inside_loop || node.kind() == "for_statement";

    if next_inside_loop && node.kind() == "call_expression" {
        let function_node = node.child_by_field_name("function");
        if let Some(function_node) = function_node {
            let target = source.get(function_node.byte_range()).unwrap_or("").trim();
            if is_reflection_call(target, imports) {
                lines.push(node.start_position().row + 1);
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_reflection_in_loop(child, source, imports, next_inside_loop, lines);
    }
}

fn is_reflection_call(target: &str, imports: &[ImportSpec]) -> bool {
    imports
        .iter()
        .filter(|import| import.path == "reflect")
        .any(|import| target.starts_with(&format!("{}.", import.alias)))
}

fn collect_json_marshal_in_loop_lines(
    body_node: Node<'_>,
    source: &str,
    imports: &[ImportSpec],
) -> Vec<usize> {
    let mut lines = Vec::new();
    visit_for_json_marshal_in_loop(body_node, source, imports, false, &mut lines);
    lines
}

fn visit_for_json_marshal_in_loop(
    node: Node<'_>,
    source: &str,
    imports: &[ImportSpec],
    inside_loop: bool,
    lines: &mut Vec<usize>,
) {
    let next_inside_loop = inside_loop || node.kind() == "for_statement";

    if next_inside_loop && node.kind() == "call_expression" {
        let function_node = node.child_by_field_name("function");
        if let Some(function_node) = function_node {
            let target = source.get(function_node.byte_range()).unwrap_or("").trim();
            if is_json_marshal_call(target, imports) {
                lines.push(node.start_position().row + 1);
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_json_marshal_in_loop(child, source, imports, next_inside_loop, lines);
    }
}

fn is_json_marshal_call(target: &str, imports: &[ImportSpec]) -> bool {
    imports
        .iter()
        .filter(|import| import.path == "encoding/json")
        .any(|import| {
            target == format!("{}.Marshal", import.alias)
                || target == format!("{}.MarshalIndent", import.alias)
        })
}

fn collect_db_query_calls(body_node: Node<'_>, source: &str) -> Vec<DbQueryCall> {
    let mut calls = Vec::new();
    visit_for_db_query_calls(body_node, source, false, &mut calls);
    calls
}

fn visit_for_db_query_calls(
    node: Node<'_>,
    source: &str,
    inside_loop: bool,
    calls: &mut Vec<DbQueryCall>,
) {
    let next_inside_loop = inside_loop || node.kind() == "for_statement";

    if node.kind() == "call_expression" {
        let function_node = node.child_by_field_name("function");
        let arguments_node = node.child_by_field_name("arguments");

        if let Some(function_node) = function_node {
            if let Some((receiver, name)) = extract_call_target(function_node, source) {
                if is_database_query_method(&name) {
                    let query_text = arguments_node.and_then(|arguments| first_string_literal(arguments, source));
                    calls.push(DbQueryCall {
                        line: node.start_position().row + 1,
                        receiver,
                        method_name: name,
                        query_text,
                        in_loop: next_inside_loop,
                    });
                }
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_db_query_calls(child, source, next_inside_loop, calls);
    }
}

fn is_database_query_method(name: &str) -> bool {
    matches!(
        name,
        "Query"
            | "QueryContext"
            | "QueryRow"
            | "QueryRowContext"
            | "Exec"
            | "ExecContext"
            | "Get"
            | "Select"
            | "Raw"
            | "First"
            | "Find"
            | "Take"
            | "Preload"
    )
}

fn collect_explicit_string_variables(body_node: Node<'_>, source: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    visit_for_string_variables(body_node, source, &mut names);
    names
}

fn visit_for_string_variables(node: Node<'_>, source: &str, names: &mut BTreeSet<String>) {
    match node.kind() {
        "var_spec" => {
            let Some(type_node) = node.child_by_field_name("type") else {
                return;
            };
            if source
                .get(type_node.byte_range())
                .is_some_and(|text| text.trim() == "string")
            {
                if let Some(name_node) = find_var_name_node(node) {
                    for (name, _) in collect_identifiers(name_node, source) {
                        names.insert(name);
                    }
                }
            }
        }
        "short_var_declaration" | "assignment_statement" => {
            if let Some(text) = source.get(node.byte_range()) {
                if let Some((left, right)) = split_assignment(text) {
                    let left = left.trim();
                    if is_identifier_name(left) && contains_string_literal(right) {
                        names.insert(left.to_string());
                    }
                }
            }
        }
        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_string_variables(child, source, names);
    }
}

fn visit_for_string_concat_in_loop(
    node: Node<'_>,
    source: &str,
    string_variables: &BTreeSet<String>,
    inside_loop: bool,
    lines: &mut Vec<usize>,
) {
    let next_inside_loop = inside_loop || node.kind() == "for_statement";

    if next_inside_loop && node.kind() == "assignment_statement" {
        if let Some(text) = source.get(node.byte_range()) {
            if is_string_concat_assignment(text, string_variables) {
                lines.push(node.start_position().row + 1);
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_string_concat_in_loop(child, source, string_variables, next_inside_loop, lines);
    }
}

fn is_string_concat_assignment(text: &str, string_variables: &BTreeSet<String>) -> bool {
    let compact = text.split_whitespace().collect::<String>();

    if let Some((left, right)) = compact.split_once("+=") {
        return is_identifier_name(left)
            && (string_variables.contains(left) || contains_string_literal(right));
    }

    let Some((left, right)) = compact.split_once('=') else {
        return false;
    };
    if !is_identifier_name(left) || !string_variables.contains(left) {
        return false;
    }

    right.starts_with(&format!("{left}+"))
        || right.contains(&format!("+\""))
        || right.contains("+`")
}

fn split_assignment(text: &str) -> Option<(&str, &str)> {
    text.split_once(":=").or_else(|| text.split_once('='))
}

fn is_identifier_name(text: &str) -> bool {
    !text.is_empty()
        && text
            .chars()
            .all(|character| character == '_' || character.is_ascii_alphanumeric())
        && text
            .chars()
            .next()
            .is_some_and(|character| character == '_' || character.is_ascii_alphabetic())
}

fn contains_string_literal(text: &str) -> bool {
    text.contains('"') || text.contains('`')
}

fn extract_doc_comment(source: &str, function_start_row: usize) -> Option<String> {
    let lines = source.lines().collect::<Vec<_>>();
    if function_start_row == 0 || function_start_row > lines.len() {
        return None;
    }

    let mut index = function_start_row;
    let mut comment_lines = Vec::new();

    while index > 0 {
        index -= 1;
        let trimmed = lines[index].trim();

        if trimmed.is_empty() {
            break;
        }

        if trimmed.starts_with("//") {
            comment_lines.push(trimmed.trim_start_matches("//").trim().to_string());
            continue;
        }

        if trimmed.ends_with("*/") {
            let mut block_lines = vec![trimmed.to_string()];
            while index > 0 {
                index -= 1;
                block_lines.push(lines[index].trim().to_string());
                if lines[index].trim().starts_with("/*") {
                    break;
                }
            }
            block_lines.reverse();
            let normalized = block_lines
                .into_iter()
                .map(|line| {
                    line.trim_start_matches("/*")
                        .trim_end_matches("*/")
                        .trim_start_matches('*')
                        .trim()
                        .to_string()
                })
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>();
            if normalized.is_empty() {
                return None;
            }
            return Some(normalized.join("\n"));
        }

        break;
    }

    if comment_lines.is_empty() {
        None
    } else {
        comment_lines.reverse();
        Some(comment_lines.join("\n"))
    }
}

fn collect_dropped_error_lines(body_node: Node<'_>, source: &str) -> Vec<usize> {
    let mut lines = Vec::new();
    visit_for_dropped_errors(body_node, source, &mut lines);
    lines
}

fn visit_for_dropped_errors(node: Node<'_>, source: &str, lines: &mut Vec<usize>) {
    if matches!(
        node.kind(),
        "assignment_statement" | "short_var_declaration"
    ) {
        if let Some(text) = source.get(node.byte_range()) {
            let compact = text.split_whitespace().collect::<String>();
            let drops_named_err = compact.starts_with("_=err")
                || compact.starts_with("_=ctx.Err()")
                || compact.contains(",_=err")
                || compact.contains(",_=ctx.Err()");
            if drops_named_err {
                lines.push(node.start_position().row + 1);
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_dropped_errors(child, source, lines);
    }
}

fn collect_panic_on_error_lines(body_node: Node<'_>, source: &str) -> Vec<usize> {
    let mut lines = Vec::new();
    visit_for_panic_on_error(body_node, source, &mut lines);
    lines
}

fn visit_for_panic_on_error(node: Node<'_>, source: &str, lines: &mut Vec<usize>) {
    if node.kind() == "if_statement" {
        let condition = node
            .child_by_field_name("condition")
            .and_then(|condition| source.get(condition.byte_range()));
        let consequence = node
            .child_by_field_name("consequence")
            .and_then(|consequence| source.get(consequence.byte_range()));

        if let (Some(condition), Some(consequence)) = (condition, consequence) {
            let normalized_condition = condition.split_whitespace().collect::<String>();
            let panic_like = consequence.contains("panic(")
                || consequence.contains("log.Fatal(")
                || consequence.contains("log.Fatalf(")
                || consequence.contains("log.Fatalln(");
            if normalized_condition.contains("err!=nil") && panic_like {
                lines.push(node.start_position().row + 1);
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_panic_on_error(child, source, lines);
    }
}

fn collect_errorf_calls(body_node: Node<'_>, source: &str) -> Vec<FormattedErrorCall> {
    let mut calls = Vec::new();
    visit_for_errorf_calls(body_node, source, &mut calls);
    calls
}

fn visit_for_errorf_calls(node: Node<'_>, source: &str, calls: &mut Vec<FormattedErrorCall>) {
    if node.kind() == "call_expression" {
        let function_node = node.child_by_field_name("function");
        let arguments_node = node.child_by_field_name("arguments");

        if let (Some(function_node), Some(arguments_node)) = (function_node, arguments_node) {
            let target = source.get(function_node.byte_range()).unwrap_or("");
            if target.trim() == "fmt.Errorf" {
                let arguments = source.get(arguments_node.byte_range()).unwrap_or("");
                let format_string = first_string_literal(arguments_node, source);
                calls.push(FormattedErrorCall {
                    line: node.start_position().row + 1,
                    format_string,
                    mentions_err: arguments.contains("err"),
                    uses_percent_w: arguments.contains("%w"),
                });
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_for_errorf_calls(child, source, calls);
    }
}

fn first_string_literal(node: Node<'_>, source: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if matches!(
            child.kind(),
            "interpreted_string_literal" | "raw_string_literal"
        ) {
            let literal = source.get(child.byte_range())?;
            return Some(literal.trim_matches('"').trim_matches('`').to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{extract_doc_comment, package_alias_from_import_path, parse_file};
    use crate::model::SymbolKind;
    use std::path::Path;

    #[test]
    fn derives_import_alias_from_path() {
        assert_eq!(
            package_alias_from_import_path("github.com/acme/utils"),
            "utils"
        );
    }

    #[test]
    fn collects_package_level_function_alias_vars_as_symbols() {
        let source = r#"package pdf

import font "example.com/font"

var (
    IsCustomFont = font.IsCustomFont
    PlainValue = 42
)

func collectAllStandardFontsInTemplate() {
    IsCustomFont("Helvetica")
}
"#;

        let parsed = parse_file(Path::new("sample.go"), source).expect("parse should work");
        assert!(parsed.symbols.iter().any(|symbol| {
            symbol.name == "IsCustomFont" && matches!(symbol.kind, SymbolKind::Function)
        }));
        assert!(
            !parsed
                .symbols
                .iter()
                .any(|symbol| symbol.name == "PlainValue")
        );
    }

    #[test]
    fn extracts_doc_comment_text() {
        let source = "// Run Processes The Input\n// This function does X by doing Y because Z\nfunc Run() {}\n";

        let comment = extract_doc_comment(source, 2).expect("doc comment should exist");
        assert_eq!(
            comment,
            "Run Processes The Input\nThis function does X by doing Y because Z"
        );
    }

    #[test]
    fn collects_error_handling_signals() {
        let source = r#"package sample

import (
    "fmt"
    "log"
)

func Run(err error) error {
    _ = err
    if err != nil {
        panic(err)
    }
    return fmt.Errorf("wrap: %v", err)
}

func LogOnly(err error) {
    if err != nil {
        log.Fatal(err)
    }
}
"#;

        let parsed = parse_file(Path::new("sample.go"), source).expect("parse should work");
        let run = parsed
            .functions
            .iter()
            .find(|function| function.fingerprint.name == "Run")
            .expect("Run should be parsed");
        let log_only = parsed
            .functions
            .iter()
            .find(|function| function.fingerprint.name == "LogOnly")
            .expect("LogOnly should be parsed");

        assert_eq!(run.dropped_error_lines, vec![9]);
        assert_eq!(run.panic_on_error_lines, vec![10]);
        assert_eq!(run.errorf_calls.len(), 1);
        assert!(run.errorf_calls[0].mentions_err);
        assert!(!run.errorf_calls[0].uses_percent_w);
        assert_eq!(log_only.panic_on_error_lines, vec![17]);
    }

    #[test]
    fn collects_context_and_sleep_signals() {
        let source = r#"package sample

import (
    "context"
    "time"
)

func Poll(ctx context.Context) {
    for {
        time.Sleep(time.Second)
        _ = ctx
    }
}
"#;

        let parsed = parse_file(Path::new("sample.go"), source).expect("parse should work");
        let poll = parsed
            .functions
            .iter()
            .find(|function| function.fingerprint.name == "Poll")
            .expect("Poll should be parsed");

        assert!(poll.has_context_parameter);
        assert_eq!(poll.sleep_in_loop_lines, vec![10]);
    }

    #[test]
    fn collects_context_factory_busy_wait_and_json_signals() {
        let source = r#"package sample

import (
    "context"
    "encoding/json"
    "time"
)

func Run(parent context.Context, items []string) {
    ctx, cancel := context.WithTimeout(parent, time.Second)
    _ = ctx

    for {
        select {
        default:
            return
        }
    }

    for _, item := range items {
        _, _ = json.Marshal(item)
    }

    _ = cancel
}
"#;

        let parsed = parse_file(Path::new("sample.go"), source).expect("parse should work");
        let run = parsed
            .functions
            .iter()
            .find(|function| function.fingerprint.name == "Run")
            .expect("Run should be parsed");

        assert_eq!(run.context_factory_calls.len(), 1);
        assert_eq!(run.context_factory_calls[0].cancel_name, "cancel");
        assert_eq!(run.context_factory_calls[0].factory_name, "WithTimeout");
        assert_eq!(run.busy_wait_lines, vec![14]);
        assert_eq!(run.json_marshal_in_loop_lines, vec![21]);
    }

    #[test]
    fn collects_concurrency_and_db_signals() {
        let source = r#"package sample

import (
    "context"
    "fmt"
    "reflect"
    "time"
)

func Run(ctx context.Context, db Queryer, items []string, mu MutexLike) {
    go func() {
        for {
            _ = ctx
        }
    }()

    for _, item := range items {
        mu.Lock()
        time.Sleep(time.Millisecond)
        _, _ = db.QueryContext(ctx, "SELECT * FROM widgets WHERE name LIKE '%foo%'")
        _ = fmt.Sprintf("%s", item)
        _ = reflect.TypeOf(item)
        _ = make([]byte, 16)
        mu.Unlock()
    }
}

type Queryer interface {
    QueryContext(context.Context, string) (any, error)
}

type MutexLike interface {
    Lock()
    Unlock()
}
"#;

        let parsed = parse_file(Path::new("sample.go"), source).expect("parse should work");
        let run = parsed
            .functions
            .iter()
            .find(|function| function.fingerprint.name == "Run")
            .expect("Run should be parsed");

        assert_eq!(run.goroutine_without_shutdown_lines, vec![11]);
        assert_eq!(run.mutex_lock_in_loop_lines, vec![18]);
        assert_eq!(run.allocation_in_loop_lines, vec![23]);
        assert_eq!(run.fmt_in_loop_lines, vec![21]);
        assert_eq!(run.reflection_in_loop_lines, vec![22]);
        assert_eq!(run.db_query_calls.len(), 1);
        assert!(run.db_query_calls[0].in_loop);
        assert_eq!(
            run.db_query_calls[0].query_text.as_deref(),
            Some("SELECT * FROM widgets WHERE name LIKE '%foo%'")
        );
    }

    #[test]
    fn collects_string_concat_and_goroutine_signals() {
        let source = r#"package sample

func Build(parts []string) string {
    out := ""
    for _, part := range parts {
        out += part
        go notify(part)
    }
    return out
}

func notify(value string) {}
"#;

        let parsed = parse_file(Path::new("sample.go"), source).expect("parse should work");
        let build = parsed
            .functions
            .iter()
            .find(|function| function.fingerprint.name == "Build")
            .expect("Build should be parsed");

        assert_eq!(build.string_concat_in_loop_lines, vec![6]);
        assert_eq!(build.goroutine_launch_lines, vec![7]);
    }
}
