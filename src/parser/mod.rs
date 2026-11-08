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

#[cfg(test)]
mod tests {
    use super::*;

    fn symbol_names(extension: &str, content: &str) -> Vec<String> {
        parse_file(extension, content)
            .into_iter()
            .map(|symbol| symbol.name)
            .collect()
    }

    #[test]
    fn test_python_class_and_function() {
        let content = "class Greeter:\n    pass\n\ndef top_level(x):\n    return x * 2\n";
        let names = symbol_names("py", content);
        assert!(names.contains(&"Greeter".to_string()));
        assert!(names.contains(&"top_level".to_string()));
    }

    #[test]
    fn test_typescript_interface_function_class() {
        let content = "export interface User { id: string }\nexport function makeUser(id: string): User {\n  return { id };\n}\nexport class Repo {\n}\n";
        let names = symbol_names("ts", content);
        assert!(names.contains(&"User".to_string()));
        assert!(names.contains(&"makeUser".to_string()));
        assert!(names.contains(&"Repo".to_string()));
    }

    #[test]
    fn test_go_struct_and_function() {
        let content = "package main\n\ntype Server struct{ Port int }\n\nfunc main() {}\n";
        let names = symbol_names("go", content);
        assert!(names.contains(&"Server".to_string()));
        assert!(names.contains(&"main".to_string()));
    }

    #[test]
    fn test_rust_struct_and_function() {
        let content = "pub struct Item {\n    pub id: u32,\n}\n\npub fn build() -> Item {\n    Item { id: 1 }\n}\n";
        let names = symbol_names("rs", content);
        assert!(names.contains(&"Item".to_string()));
        assert!(names.contains(&"build".to_string()));
    }

    #[test]
    fn test_unknown_extension_returns_nothing() {
        assert!(parse_file("txt", "anything").is_empty());
    }
}
