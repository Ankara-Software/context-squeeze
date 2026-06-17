use super::Symbol;
use regex::Regex;

pub struct PythonParser<'a> {
    content: &'a str,
}

impl<'a> PythonParser<'a> {
    pub fn new(content: &'a str) -> Self {
        Self { content }
    }

    pub fn parse(&self) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = self.content.lines().collect();
        let class_regex = Regex::new(r"^\s*class\s+(\w+)(?:\s*\(([^)]*)\))?\s*:").unwrap();
        let func_regex = Regex::new(r"^\s*def\s+(\w+)\s*\(([^)]*)\)(?:\s*->\s*([^:]+))?\s*:").unwrap();

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];
            let trim_line = line.trim();
            if trim_line.is_empty() || trim_line.starts_with('#') {
                i += 1;
                continue;
            }

            let mut matched = false;
            let mut name = String::new();
            let mut kind = String::new();
            let mut signature = String::new();
            let mut line_start = i + 1;

            if let Some(caps) = class_regex.captures(line) {
                matched = true;
                name = caps.get(1).unwrap().as_str().to_string();
                kind = "class".to_string();
                signature = trim_line.to_string();
            } else if let Some(caps) = func_regex.captures(line) {
                matched = true;
                name = caps.get(1).unwrap().as_str().to_string();
                kind = "function".to_string();
                signature = trim_line.to_string();
            }

            if matched {
                // Determine indentation of the match
                let start_indent = line.len() - trim_line.len();
                let mut line_end = i + 1;
                
                // Scan forward to find where the block ends based on indentation
                let mut j = i + 1;
                while j < lines.len() {
                    let next_line = lines[j];
                    let next_trim = next_line.trim();
                    if next_trim.is_empty() {
                        j += 1;
                        continue;
                    }
                    let next_indent = next_line.len() - next_trim.len();
                    if next_indent <= start_indent {
                        break;
                    }
                    line_end = j + 1;
                    j += 1;
                }

                // Construct body
                let body_lines = &lines[i..=line_end - 1];
                let body = body_lines.join("\n");

                symbols.push(Symbol {
                    name,
                    kind,
                    signature,
                    body,
                    line_start,
                    line_end,
                });
                
                i = j - 1; // Advance loop
            }
            i += 1;
        }

        symbols
    }
}
