//! lib.rs — Snip: Vibe-code security gate for AI diffs.
//! Pure std:: Rust (Edition 2021). Zero external dependencies.

pub mod audit;
pub mod gate;
pub mod report;

pub use audit::{audit_diff, Category, Diff, DiffFile, Finding, Severity};
pub use gate::{evaluate, GateResult, Policy, Verdict};
pub use report::{emit_cursor_markdown, emit_generic_text, emit_json};

/// Evaluates a diff against the given security policy and emits a GateResult.
pub fn check(diff: &Diff, policy: &Policy) -> GateResult {
    let findings = audit::audit_diff(diff);
    gate::evaluate(&findings, policy)
}
