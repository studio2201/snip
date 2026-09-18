//! gate.rs — Ship/Fix/Block verdict evaluation.
//! Formulates immediate gate outcomes based on diff findings and policy.

use crate::audit::{Finding, Severity};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Ship,
    Fix,
    Block,
}

impl fmt::Display for Verdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Verdict::Ship => write!(f, "SHIP"),
            Verdict::Fix => write!(f, "FIX"),
            Verdict::Block => write!(f, "BLOCK"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Policy {
    pub block_on_secrets: bool,
    pub block_on_broken_rls: bool,
    pub max_critical: usize,
    pub max_high: usize,
}

impl Default for Policy {
    fn default() -> Self {
        Policy {
            block_on_secrets: true,
            block_on_broken_rls: true,
            max_critical: 0,
            max_high: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GateResult {
    pub verdict: Verdict,
    pub reasons: Vec<String>,
    pub findings: Vec<Finding>,
}

pub fn evaluate(findings: &[Finding], policy: &Policy) -> GateResult {
    let mut reasons = Vec::new();
    let mut critical_count = 0;
    let mut high_count = 0;
    let mut medium_count = 0;

    for f in findings {
        match f.severity {
            Severity::Critical => critical_count += 1,
            Severity::High => high_count += 1,
            Severity::Medium => medium_count += 1,
            Severity::Low => {}
        }
    }

    if critical_count > policy.max_critical {
        reasons.push(format!(
            "Critical finding threshold exceeded: {} detected (max {})",
            critical_count, policy.max_critical
        ));
    }

    if high_count > policy.max_high {
        reasons.push(format!(
            "High severity finding threshold exceeded: {} detected (max {})",
            high_count, policy.max_high
        ));
    }

    let verdict = if critical_count > 0 || high_count > policy.max_high {
        Verdict::Block
    } else if medium_count > 0 {
        Verdict::Fix
    } else {
        Verdict::Ship
    };

    GateResult {
        verdict,
        reasons,
        findings: findings.to_vec(),
    }
}
