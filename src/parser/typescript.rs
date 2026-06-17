use super::Symbol;
use regex::Regex;

pub struct TypeScriptParser<'a> {
    content: &'a str,
}

impl<'a> TypeScriptParser<'a> {
    pub fn new(content: &'a str) -> Self {
        Self { content }
    }

    pub fn parse(&self) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = self.content.lines().collect();

        let class_regex = Regex::new(r"(?:export\s+)?(?:default\s+)?class\s+(\w+)").unwrap();
        let interface_regex = Regex::new(r"(?:export\s+)?interface\s+(\w+)").unwrap();
        let type_regex = Regex::new(r"(?:export\s+)?type\s+(\w+)").unwrap();
        let func_regex = Regex::new(r"(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s+(\w+)").unwrap();
        let arrow_regex = Regex::new(r"(?:export\s+)?const\s+(\w+)\s*=\s*.*=>").unwrap();

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

            if let Some(caps) = class_regex.captures(line) {
                matched = true;
                name = caps.get(1).unwrap().as_str().to_string();
                kind = "class".to_string();
            } else if let Some(caps) = interface_regex.captures(line) {
                matched = true;
                name = caps.get(1).unwrap().as_str().to_string();
                kind = "interface".to_string();
            } else if let Some(caps) = type_regex.captures(line) {
                matched = true;
                name = caps.get(1).unwrap().as_str().to_string();
                kind = "type".to_string();
            } else if let Some(caps) = func_regex.captures(line) {
                matched = true;
                name = caps.get(1).unwrap().as_str().to_string();
                kind = "function".to_string();
            } else if let Some(caps) = arrow_regex.captures(line) {
                matched = true;
                name = caps.get(1).unwrap().as_str().to_string();
                kind = "function".to_string();
            }

            if matched {
                let line_start = i + 1;
                let signature = trim_line.to_string();
                let mut line_end = i + 1;
                
                // For 'type' declarations, they might end with a semicolon on the same line or subsequent lines
                if kind == "type" && !trim_line.contains('{') {
                    let mut j = i;
                    while j < lines.len() {
                        let next_line = lines[j];
                        line_end = j + 1;
                        if next_line.contains(';') || next_line.trim().is_empty() {
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
                } else {
                    // C-style brace counting
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
            }
            i += 1;
        }

        symbols
    }
}
