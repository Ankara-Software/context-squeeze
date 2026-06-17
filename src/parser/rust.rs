use super::Symbol;
use regex::Regex;

pub struct RustParser<'a> {
    content: &'a str,
}

impl<'a> RustParser<'a> {
    pub fn new(content: &'a str) -> Self {
        Self { content }
    }

    pub fn parse(&self) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = self.content.lines().collect();

        let struct_regex = Regex::new(r"(?:pub\s+)?struct\s+(\w+)").unwrap();
        let enum_regex = Regex::new(r"(?:pub\s+)?enum\s+(\w+)").unwrap();
        let trait_regex = Regex::new(r"(?:pub\s+)?trait\s+(\w+)").unwrap();
        let fn_regex = Regex::new(r"(?:pub\s+)?(?:const\s+)?(?:async\s+)?fn\s+(\w+)").unwrap();
        let impl_regex = Regex::new(r"^impl(?:\s*<[^>]*>)?\s*(?:(?:\w+::)?\w+)?\s*(?:for\s+(\w+))?").unwrap();

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];
            let trim_line = line.trim();
            if trim_line.is_empty() || trim_line.starts_with("//") || trim_line.starts_with("/*") {
                i += 1;
                continue;
            }

            let mut matched = false;
            let mut name = String::new();
            let mut kind = String::new();

            if let Some(caps) = struct_regex.captures(line) {
                matched = true;
                name = caps.get(1).unwrap().as_str().to_string();
                kind = "struct".to_string();
            } else if let Some(caps) = enum_regex.captures(line) {
                matched = true;
                name = caps.get(1).unwrap().as_str().to_string();
                kind = "enum".to_string();
            } else if let Some(caps) = trait_regex.captures(line) {
                matched = true;
                name = caps.get(1).unwrap().as_str().to_string();
                kind = "trait".to_string();
            } else if let Some(caps) = fn_regex.captures(line) {
                matched = true;
                name = caps.get(1).unwrap().as_str().to_string();
                kind = "function".to_string();
            }

            if matched {
                let line_start = i + 1;
                let signature = trim_line.to_string();
                let mut line_end = i + 1;
                
                // Rust brace counting
                let mut brace_count = 0;
                let mut opened = false;
                let mut j = i;
                
                while j < lines.len() {
                    let current = lines[j];
                    for c in current.chars() {
                        if c == '{' {
                            brace_count += 1;
                            opened = true;
                        } else if c == '}' {
                            brace_count -= 1;
                        }
                    }
                    line_end = j + 1;
                    if opened && brace_count <= 0 {
                        break;
                    }
                    // Structs can end with a semicolon (like unit structs or tuple structs)
                    if !opened && current.contains(';') {
                        break;
                    }
                    j += 1;
                }

                let body = lines[i..=line_end - 1].join("\n");
                symbols.push(Symbol {
                    name,
                    kind,
                    signature,
                    body,
                    line_start,
                    line_end,
                });
                i = j;
            }
            i += 1;
        }

        symbols
    }
}
