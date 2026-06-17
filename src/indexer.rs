use std::path::{Path, PathBuf};
use std::fs;
use ignore::WalkBuilder;
use super::parser::{parse_file, Symbol};

pub struct Indexer {
    pub root_path: PathBuf,
}

impl Indexer {
    pub fn new(root_path: PathBuf) -> Self {
        Self { root_path }
    }

    pub fn build_repo_map(&self) -> String {
        let mut result = String::new();
        let walker = WalkBuilder::new(&self.root_path)
            .hidden(true)
            .git_ignore(true)
            .build();

        for entry in walker.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if matches!(ext, "py" | "ts" | "tsx" | "js" | "jsx" | "rs" | "go") {
                        if let Ok(relative) = path.strip_prefix(&self.root_path) {
                            if let Ok(content) = fs::read_to_string(path) {
                                let symbols = parse_file(ext, &content);
                                if !symbols.is_empty() {
                                    result.push_str(&format!("File: {}\n", relative.display()));
                                    for sym in &symbols {
                                        result.push_str(&format!("  - [{}] {}\n", sym.kind, sym.signature));
                                    }
                                    result.push('\n');
                                }
                            }
                        }
                    }
                }
            }
        }
        
        if result.is_empty() {
            "No parseable source code files found in the workspace.".to_string()
        } else {
            result
        }
    }

    pub fn get_symbol_body(&self, target_file: &str, symbol_name: &str) -> Option<String> {
        let full_path = self.root_path.join(target_file);
        if !full_path.exists() {
            return None;
        }
        if let Some(ext) = full_path.extension().and_then(|e| e.to_str()) {
            if let Ok(content) = fs::read_to_string(&full_path) {
                let symbols = parse_file(ext, &content);
                for sym in symbols {
                    if sym.name == symbol_name {
                        return Some(sym.body);
                    }
                }
            }
        }
        None
    }

    pub fn search_symbols(&self, query: &str) -> String {
        let mut results = String::new();
        let walker = WalkBuilder::new(&self.root_path)
            .hidden(true)
            .git_ignore(true)
            .build();

        let query_lower = query.to_lowercase();

        for entry in walker.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if matches!(ext, "py" | "ts" | "tsx" | "js" | "jsx" | "rs" | "go") {
                        if let Ok(relative) = path.strip_prefix(&self.root_path) {
                            if let Ok(content) = fs::read_to_string(path) {
                                let symbols = parse_file(ext, &content);
                                for sym in symbols {
                                    if sym.name.to_lowercase().contains(&query_lower) || sym.signature.to_lowercase().contains(&query_lower) {
                                        results.push_str(&format!("File: {} -> Symbol: {}\nKind: {}\nSignature: {}\nLines: {} - {}\n\n",
                                            relative.display(), sym.name, sym.kind, sym.signature, sym.line_start, sym.line_end
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if results.is_empty() {
            format!("No symbols found matching query: '{}'", query)
        } else {
            results
        }
    }
}
