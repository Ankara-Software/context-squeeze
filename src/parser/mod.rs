use serde::{Serialize, Deserialize};

pub mod python;
pub mod typescript;
pub mod rust;
pub mod go;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub kind: String, // "class" | "function" | "interface" | "struct" | "enum" | "trait" | "type"
    pub signature: String,
    pub body: String,
    pub line_start: usize,
    pub line_end: usize,
}

pub trait Parser {
    fn parse(&self) -> Vec<Symbol>;
}

pub fn parse_file(extension: &str, content: &str) -> Vec<Symbol> {
    match extension {
        "py" => python::PythonParser::new(content).parse(),
        "ts" | "tsx" | "js" | "jsx" => typescript::TypeScriptParser::new(content).parse(),
        "rs" => rust::RustParser::new(content).parse(),
        "go" => go::GoParser::new(content).parse(),
        _ => Vec::new(),
    }
}
