//! audit.rs — Static security pass over code diffs.
//! Detects leaked credentials, broken Supabase RLS, and permissive configurations.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Low => write!(f, "LOW"),
            Severity::Medium => write!(f, "MEDIUM"),
            Severity::High => write!(f, "HIGH"),
            Severity::Critical => write!(f, "CRITICAL"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    SecretLeak,
    BrokenRLS,
    PermissiveCORS,
    InsecureEval,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::SecretLeak => write!(f, "Secret Leak"),
            Category::BrokenRLS => write!(f, "Broken Supabase RLS"),
            Category::PermissiveCORS => write!(f, "Permissive CORS"),
            Category::InsecureEval => write!(f, "Insecure Code Execution"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub file: String,
    pub line: usize,
    pub severity: Severity,
    pub category: Category,
    pub description: String,
    pub snippet: String,
    pub remediation: String,
}

#[derive(Debug, Clone)]
pub struct DiffFile {
    pub path: String,
    pub additions: Vec<(usize, String)>,
}

#[derive(Debug, Clone)]
pub struct Diff {
    pub raw: String,
    pub files: Vec<DiffFile>,
}

impl Diff {
    pub fn from_raw(raw: &str) -> Self {
        let mut files = Vec::new();
        let mut cur_file = "stdin".to_string();
        let mut additions = Vec::new();
        let mut line_num = 1;

        for line in raw.lines() {
            if line.starts_with("+++ b/") {
                if !additions.is_empty() {
                    files.push(DiffFile { path: cur_file, additions: additions.clone() });
                    additions.clear();
                }
                cur_file = line["+++ b/".len()..].trim().to_string();
            } else if line.starts_with("@@ ") {
                line_num = parse_hunk_start(line).unwrap_or(1);
            } else if line.starts_with('+') && !line.starts_with("+++") {
                let added = line[1..].to_string();
                additions.push((line_num, added));
                line_num += 1;
            } else if !line.starts_with('-') {
                line_num += 1;
            }
        }

        if !additions.is_empty() {
            files.push(DiffFile { path: cur_file, additions });
        } else if files.is_empty() && !raw.trim().is_empty() {
            // Treat unformatted raw text as added lines to stdin
            let fallback_adds: Vec<(usize, String)> = raw.lines()
                .enumerate().map(|(idx, l)| (idx + 1, l.to_string())).collect();
            files.push(DiffFile { path: "stdin".into(), additions: fallback_adds });
        }

        Diff { raw: raw.to_string(), files }
    }
}

fn parse_hunk_start(line: &str) -> Option<usize> {
    let plus_pos = line.find('+')?;
    let rest = &line[plus_pos + 1..];
    let end_pos = rest.find(|c: char| c == ',' || c == ' ')?;
    rest[..end_pos].parse::<usize>().ok()
}

pub fn audit_diff(diff: &Diff) -> Vec<Finding> {
    let mut findings = Vec::new();

    for file in &diff.files {
        let mut seen_create_table = false;
        let mut has_enable_rls = false;

        for (line_num, line) in &file.additions {
            let lower = line.to_lowercase();

            // 1. Secret scanning
            if let Some(f) = check_secrets(&file.path, *line_num, line) {
                findings.push(f);
            }

            // 2. Broken Supabase RLS
            if lower.contains("create table ") {
                seen_create_table = true;
            }
            if lower.contains("enable row level security") {
                has_enable_rls = true;
            }
            if lower.contains("disable row level security") {
                findings.push(Finding {
                    file: file.path.clone(),
                    line: *line_num,
                    severity: Severity::Critical,
                    category: Category::BrokenRLS,
                    description: "Explicitly disabled Row Level Security (RLS)".into(),
                    snippet: line.trim().into(),
                    remediation: "Never disable RLS; write explicit permissive policies instead.".into(),
                });
            }

            // 3. Permissive CORS
            if line.contains("Access-Control-Allow-Origin: *") || line.contains("origin: \"*\"") || line.contains("origin: '*'") {
                findings.push(Finding {
                    file: file.path.clone(),
                    line: *line_num,
                    severity: Severity::High,
                    category: Category::PermissiveCORS,
                    description: "Wildcard CORS origin detected ('*')".into(),
                    snippet: line.trim().into(),
                    remediation: "Restrict Access-Control-Allow-Origin to authorized domain origins.".into(),
                });
            }

            // 4. Insecure Eval
            if line.contains("eval(") || line.contains("dangerouslySetInnerHTML") {
                findings.push(Finding {
                    file: file.path.clone(),
                    line: *line_num,
                    severity: Severity::High,
                    category: Category::InsecureEval,
                    description: "Potentially vulnerable arbitrary execution construct".into(),
                    snippet: line.trim().into(),
                    remediation: "Avoid eval / innerHTML; use structured DOM or typed parsers.".into(),
                });
            }
        }

        if seen_create_table && !has_enable_rls && file.path.ends_with(".sql") {
            findings.push(Finding {
                file: file.path.clone(),
                line: 1,
                severity: Severity::High,
                category: Category::BrokenRLS,
                description: "Table created in SQL migration without enabling Row Level Security".into(),
                snippet: "create table ...".into(),
                remediation: "Add `alter table <name> enable row level security;`".into(),
            });
        }
    }

    findings
}

fn check_secrets(file: &str, line_num: usize, line: &str) -> Option<Finding> {
    if line.contains("sk-proj-") || line.contains("sk-") && contains_long_alphanumeric(line, "sk-", 30) {
        return Some(Finding {
            file: file.to_string(), line: line_num, severity: Severity::Critical,
            category: Category::SecretLeak, description: "Hardcoded OpenAI / LLM API Key".into(),
            snippet: redact(line, "sk-"), remediation: "Move secret to environment variables or secret manager.".into(),
        });
    }
    if line.contains("AKIA") && contains_long_alphanumeric(line, "AKIA", 16) {
        return Some(Finding {
            file: file.to_string(), line: line_num, severity: Severity::Critical,
            category: Category::SecretLeak, description: "Hardcoded AWS Access Key ID".into(),
            snippet: redact(line, "AKIA"), remediation: "Use IAM roles or AWS SSM Parameter Store.".into(),
        });
    }
    if line.contains("sk_live_") {
        return Some(Finding {
            file: file.to_string(), line: line_num, severity: Severity::Critical,
            category: Category::SecretLeak, description: "Hardcoded Stripe Production Secret Key".into(),
            snippet: redact(line, "sk_live_"), remediation: "Rotate key immediately and pass via ENV.".into(),
        });
    }
    if line.contains("ghp_") || line.contains("github_pat_") {
        return Some(Finding {
            file: file.to_string(), line: line_num, severity: Severity::Critical,
            category: Category::SecretLeak, description: "Hardcoded GitHub Personal Access Token".into(),
            snippet: redact(line, "ghp_"), remediation: "Revoke token and use GitHub Actions secrets.".into(),
        });
    }
    if line.contains("-----BEGIN") && (line.contains("PRIVATE KEY-----") || line.contains("RSA PRIVATE KEY")) {
        return Some(Finding {
            file: file.to_string(), line: line_num, severity: Severity::Critical,
            category: Category::SecretLeak, description: "Hardcoded Private Key block".into(),
            snippet: "-----BEGIN PRIVATE KEY... [REDACTED]".into(),
            remediation: "Do not commit private keys to version control.".into(),
        });
    }
    None
}

fn contains_long_alphanumeric(line: &str, prefix: &str, min_len: usize) -> bool {
    if let Some(pos) = line.find(prefix) {
        let candidate = &line[pos + prefix.len()..];
        let count = candidate.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-').count();
        count >= min_len
    } else {
        false
    }
}

fn redact(line: &str, prefix: &str) -> String {
    if let Some(pos) = line.find(prefix) {
        let end = (pos + prefix.len() + 6).min(line.len());
        format!("{}... [REDACTED]", &line[..end])
    } else {
        "[REDACTED]".into()
    }
}
