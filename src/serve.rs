//! serve.rs — Model Context Protocol (MCP) JSON-RPC 2.0 server for snip.
use snip::{check, emit_generic_text, Diff, Policy};
use std::io::{self, BufRead};

fn extract_str(line: &str, key: &str) -> Option<String> {
    let k = format!("\"{}\":", key);
    if let Some(p) = line.find(&k) {
        let rest = line[p + k.len()..].trim_start();
        if let Some(stripped) = rest.strip_prefix('"') {
            if let Some(end) = stripped.find('"') {
                return Some(stripped[..end].to_string());
            }
        }
    }
    None
}

fn extract_id(line: &str) -> String {
    if let Some(p) = line.find("\"id\":") {
        let rest = line[p + 5..].trim_start();
        let end = rest.find(|c: char| c == ',' || c == '}' || c.is_whitespace()).unwrap_or(rest.len());
        return rest[..end].trim().to_string();
    }
    "null".to_string()
}

fn handle_tools_call(line: &str) -> String {
    let diff_text = extract_str(line, "diff").unwrap_or_default();
    let diff = Diff::from_raw(&diff_text);
    let policy = Policy::default();
    let result = check(&diff, &policy);
    let text = emit_generic_text(&result);
    let escaped = text.replace('\\', "\\\\").replace('\"', "\\\"").replace('\n', "\\n");
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
                let call_result = handle_tools_call(trimmed);
                println!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"result\":{}}}", id, call_result);
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
