//! serve.rs — Model Context Protocol (MCP) JSON-RPC 2.0 server for snip.
use snip::{check, emit_generic_text, Diff, Policy};
use std::io::{self, BufRead};

fn extract_str(line: &str, key: &str) -> Option<String> {
    let key_pat = format!("\"{}\"", key);
    let mut search_from = 0;
    while let Some(pos) = line[search_from..].find(&key_pat) {
        let abs_pos = search_from + pos + key_pat.len();
        let after_key = &line[abs_pos..];
        let trimmed_after = after_key.trim_start();
        if let Some(after_colon) = trimmed_after.strip_prefix(':') {
            let val_part = after_colon.trim_start();
            if let Some(stripped) = val_part.strip_prefix('"') {
                let mut out = String::new();
                let mut chars = stripped.chars();
                let mut closed = false;
                while let Some(c) = chars.next() {
                    if c == '\\' {
                        match chars.next() {
                            Some('"') => out.push('"'),
                            Some('\\') => out.push('\\'),
                            Some('/') => out.push('/'),
                            Some('n') => out.push('\n'),
                            Some('r') => out.push('\r'),
                            Some('t') => out.push('\t'),
                            Some('b') => out.push('\x08'),
                            Some('f') => out.push('\x0C'),
                            Some('u') => {
                                let hex: String = chars.by_ref().take(4).collect();
                                if let Ok(cp) = u32::from_str_radix(&hex, 16) {
                                    if let Some(ch) = char::from_u32(cp) {
                                        out.push(ch);
                                        continue;
                                    }
                                }
                                out.push_str("\\u");
                                out.push_str(&hex);
                            }
                            Some(other) => {
                                out.push('\\');
                                out.push(other);
                            }
                            None => break,
                        }
                    } else if c == '"' {
                        closed = true;
                        break;
                    } else {
                        out.push(c);
                    }
                }
                if closed {
                    return Some(out);
                }
            }
        }
        search_from = abs_pos;
    }
    None
}

fn extract_id(line: &str) -> String {
    if let Some(p) = line.find("\"id\"") {
        let after_key = &line[p + 4..];
        let trimmed = after_key.trim_start();
        if let Some(after_colon) = trimmed.strip_prefix(':') {
            let rest = after_colon.trim_start();
            let end = rest
                .find(|c: char| c == ',' || c == '}' || c.is_whitespace())
                .unwrap_or(rest.len());
            let val = rest[..end].trim();
            if !val.is_empty() {
                return val.to_string();
            }
        }
    }
    "null".to_string()
}

fn handle_tools_call(line: &str) -> String {
    let diff_text = extract_str(line, "diff").unwrap_or_default();
    let diff = Diff::from_raw(&diff_text);
    let policy = Policy::default();
    let result = check(&diff, &policy);
    let text = emit_generic_text(&result);
    let escaped = text
        .replace('\\', "\\\\")
        .replace('\"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");
    format!(
        "{{\"content\":[{{\"type\":\"text\",\"text\":\"{}\"}}],\"isError\":false}}",
        escaped
    )
}

pub fn run_server() -> Result<i32, String> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let method = extract_str(trimmed, "method").unwrap_or_default();
        let id = extract_id(trimmed);

        match method.as_str() {
            "initialize" => {
                println!(
                    "{{\"jsonrpc\":\"2.0\",\"id\":{},\"result\":{{\"protocolVersion\":\"2024-11-05\",\
                     \"capabilities\":{{\"tools\":{{}}}},\"serverInfo\":{{\"name\":\"snip\",\"version\":\"0.2.1\"}}}}}}",
                    id
                );
            }
            "notifications/initialized" => {
                // Notifications do not return responses in JSON-RPC
            }
            "tools/list" => {
                println!(
                    "{{\"jsonrpc\":\"2.0\",\"id\":{},\"result\":{{\"tools\":[{{\"name\":\"snip_audit\",\
                     \"description\":\"Audit unified git diff for vibe-code hazards and supply chain regressions\",\
                     \"inputSchema\":{{\"type\":\"object\",\"properties\":{{\"diff\":{{\"type\":\"string\",\
                     \"description\":\"Unified git diff to audit\"}}}},\"required\":[\"diff\"]}}}}]}}}}",
                    id
                );
            }
            "tools/call" => {
                let tool_name = extract_str(trimmed, "name").unwrap_or_default();
                if tool_name != "snip_audit" {
                    let esc = tool_name.replace('\\', "\\\\").replace('\"', "\\\"");
                    println!(
                        "{{\"jsonrpc\":\"2.0\",\"id\":{},\"error\":{{\"code\":-32602,\"message\":\"Unknown tool: {}\"}}}}",
                        id, esc
                    );
                } else {
                    let call_result = handle_tools_call(trimmed);
                    println!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"result\":{}}}", id, call_result);
                }
            }
            _ => {
                if id != "null" {
                    println!(
                        "{{\"jsonrpc\":\"2.0\",\"id\":{},\"error\":{{\"code\":-32601,\"message\":\"Method not found\"}}}}",
                        id
                    );
                }
            }
        }
    }
    Ok(0)
}
