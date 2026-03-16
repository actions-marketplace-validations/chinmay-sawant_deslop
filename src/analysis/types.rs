use std::path::PathBuf;

use crate::model::{FileReport, FunctionFingerprint, SymbolKind};

#[derive(Debug, Clone)]
pub(crate) struct ParsedFile {
    pub path: PathBuf,
    pub package_name: Option<String>,
    pub syntax_error: bool,
    pub byte_size: usize,
    pub functions: Vec<ParsedFunction>,
    pub imports: Vec<ImportSpec>,
    pub symbols: Vec<DeclaredSymbol>,
}

#[derive(Debug, Clone)]
pub(crate) struct ParsedFunction {
    pub fingerprint: FunctionFingerprint,
    pub calls: Vec<CallSite>,
}

#[derive(Debug, Clone)]
pub(crate) struct CallSite {
    pub receiver: Option<String>,
    pub name: String,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct ImportSpec {
    pub alias: String,
    pub path: String,
}

#[derive(Debug, Clone)]
pub(crate) struct DeclaredSymbol {
    pub name: String,
    pub kind: SymbolKind,
    pub receiver_type: Option<String>,
    pub line: usize,
}

impl ParsedFile {
    pub fn to_report(&self) -> FileReport {
        FileReport {
            path: self.path.clone(),
            package_name: self.package_name.clone(),
            syntax_error: self.syntax_error,
            byte_size: self.byte_size,
            functions: self
                .functions
                .iter()
                .map(|function| function.fingerprint.clone())
                .collect(),
        }
    }
}