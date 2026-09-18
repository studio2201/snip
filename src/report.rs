//! report.rs — Formatter emitting verdicts in Cursor, Generic text, and JSON dialects.

use crate::gate::{GateResult, Verdict};

pub fn emit_cursor_markdown(result: &GateResult) -> String {
    let mut out = String::new();
    let icon = match result.verdict {
        Verdict::Ship => "🟢 **SHIP** — No security blockers found.",
        Verdict::Fix => "🟡 **FIX** — Minor concerns require adjustment before production.",
        Verdict::Block => "🔴 **BLOCK** — Critical vulnerability or secret leak detected!",
    };

    out.push_str(&format!("### Snip Security Gate: {}\n\n", icon));

    if result.findings.is_empty() {
        out.push_str("All code diff additions passed AI vibe-code security checks.\n");
        return out;
    }

    out.push_str("#### Findings Breakdown\n\n");
    for f in &result.findings {
        out.push_str(&format!(
            "- **[{}] `{}`** (Line {} in `{}`)\n  - **Snippet**: `{}`\n  - **Fix**: {}\n\n",
            f.severity, f.category, f.line, f.file, f.snippet, f.remediation
        ));
    }

    if !result.reasons.is_empty() {
        out.push_str("#### Blocking Reasons\n\n");
        for r in &result.reasons {
            out.push_str(&format!("- {}\n", r));
        }
    }

    out
}

pub fn emit_generic_text(result: &GateResult) -> String {
    let mut out = String::new();
    out.push_str(&format!("snip verdict: {}\n", result.verdict));
    out.push_str(&format!("findings: {}\n", result.findings.len()));

    for f in &result.findings {
        out.push_str(&format!(
            "  [{}] {}:{} — {} ({})\n",
            f.severity, f.file, f.line, f.description, f.snippet
        ));
    }

    for r in &result.reasons {
        out.push_str(&format!("  blocked: {}\n", r));
    }
    out
}

pub fn emit_json(result: &GateResult) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str(&format!("  \"verdict\": \"{}\",\n", result.verdict));
    out.push_str(&format!("  \"total_findings\": {},\n", result.findings.len()));
    out.push_str("  \"reasons\": [");
    for (i, r) in result.reasons.iter().enumerate() {
        let comma = if i + 1 < result.reasons.len() { ", " } else { "" };
        out.push_str(&format!("\"{}\"{}", r, comma));
    }
    out.push_str("],\n  \"findings\": [\n");

    for (i, f) in result.findings.iter().enumerate() {
        let comma = if i + 1 < result.findings.len() { "," } else { "" };
        out.push_str(&format!(
            "    {{\"file\": \"{}\", \"line\": {}, \"severity\": \"{}\", \"category\": \"{}\", \"description\": \"{}\", \"snippet\": \"{}\"}}{}\n",
            f.file, f.line, f.severity, f.category, f.description, f.snippet, comma
        ));
    }

    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}
