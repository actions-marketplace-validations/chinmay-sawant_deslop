mod parser;

use std::path::Path;

use anyhow::Result;

use crate::index::RepositoryIndex;
use crate::model::{Finding, Severity};
use crate::analysis::{Language, LanguageBackend, ParsedFile};

#[derive(Debug, Clone, Copy)]
pub(crate) struct RustAnalyzer;

impl LanguageBackend for RustAnalyzer {
    fn language(&self) -> Language {
        Language::Rust
    }

    fn supported_extensions(&self) -> &'static [&'static str] {
        &["rs"]
    }

    fn supports_path(&self, path: &Path) -> bool {
        path.extension().and_then(|ext| ext.to_str()) == Some("rs")
    }

    fn parse_file(&self, path: &Path, source: &str) -> Result<ParsedFile> {
        parser::parse_file(path, source)
    }

    fn evaluate_file_findings(&self, file: &ParsedFile, _index: &RepositoryIndex) -> Vec<Finding> {
        evaluate_rust_file_findings(file)
    }
}

fn evaluate_rust_file_findings(file: &ParsedFile) -> Vec<Finding> {
    let mut findings = Vec::new();

    for function in &file.functions {
        findings.extend(non_test_macro_findings(
            file,
            function,
            "todo!",
            "todo_macro_leftover",
            "leaves todo! in non-test Rust code",
        ));
        findings.extend(non_test_macro_findings(
            file,
            function,
            "unimplemented!",
            "unimplemented_macro_leftover",
            "leaves unimplemented! in non-test Rust code",
        ));
        findings.extend(non_test_macro_findings(
            file,
            function,
            "dbg!",
            "dbg_macro_leftover",
            "leaves dbg! in non-test Rust code",
        ));
        findings.extend(non_test_macro_findings(
            file,
            function,
            "panic!",
            "panic_macro_leftover",
            "leaves panic! in non-test Rust code",
        ));
        findings.extend(non_test_macro_findings(
            file,
            function,
            "unreachable!",
            "unreachable_macro_leftover",
            "leaves unreachable! in non-test Rust code",
        ));
        findings.extend(non_test_call_findings(
            file,
            function,
            "unwrap",
            "unwrap_in_non_test_code",
            "calls unwrap() in non-test Rust code",
        ));
        findings.extend(non_test_call_findings(
            file,
            function,
            "expect",
            "expect_in_non_test_code",
            "calls expect() in non-test Rust code",
        ));
        findings.extend(unsafe_without_safety_comment_findings(file, function));
    }

    findings
}

fn non_test_macro_findings(
    file: &ParsedFile,
    function: &crate::analysis::ParsedFunction,
    macro_name: &str,
    rule_id: &str,
    message_suffix: &str,
) -> Vec<Finding> {
    if function.is_test_function {
        return Vec::new();
    }

    function
        .calls
        .iter()
        .filter(|call| call.name == macro_name)
        .map(|call| Finding {
            rule_id: rule_id.to_string(),
            severity: Severity::Warning,
            path: file.path.clone(),
            function_name: Some(function.fingerprint.name.clone()),
            start_line: call.line,
            end_line: call.line,
            message: format!("function {} {message_suffix}", function.fingerprint.name),
            evidence: vec![format!("macro invocation: {macro_name}")],
        })
        .collect()
}

fn non_test_call_findings(
    file: &ParsedFile,
    function: &crate::analysis::ParsedFunction,
    call_name: &str,
    rule_id: &str,
    message_suffix: &str,
) -> Vec<Finding> {
    if function.is_test_function {
        return Vec::new();
    }

    function
        .calls
        .iter()
        .filter(|call| call.name == call_name)
        .map(|call| Finding {
            rule_id: rule_id.to_string(),
            severity: Severity::Warning,
            path: file.path.clone(),
            function_name: Some(function.fingerprint.name.clone()),
            start_line: call.line,
            end_line: call.line,
            message: format!("function {} {message_suffix}", function.fingerprint.name),
            evidence: vec![match &call.receiver {
                Some(receiver) => format!("method call: {receiver}.{call_name}()"),
                None => format!("call: {call_name}()"),
            }],
        })
        .collect()
}

fn unsafe_without_safety_comment_findings(
    file: &ParsedFile,
    function: &crate::analysis::ParsedFunction,
) -> Vec<Finding> {
    function
        .unsafe_lines
        .iter()
        .filter(|unsafe_line| !has_nearby_safety_comment(**unsafe_line, &function.safety_comment_lines))
        .map(|unsafe_line| Finding {
            rule_id: "unsafe_without_safety_comment".to_string(),
            severity: Severity::Warning,
            path: file.path.clone(),
            function_name: Some(function.fingerprint.name.clone()),
            start_line: *unsafe_line,
            end_line: *unsafe_line,
            message: format!(
                "function {} uses unsafe without a nearby SAFETY comment",
                function.fingerprint.name
            ),
            evidence: vec![format!("unsafe usage line: {unsafe_line}")],
        })
        .collect()
}

fn has_nearby_safety_comment(unsafe_line: usize, safety_comment_lines: &[usize]) -> bool {
    let min_line = unsafe_line.saturating_sub(2);
    safety_comment_lines
        .iter()
        .any(|comment_line| *comment_line >= min_line && *comment_line <= unsafe_line)
}