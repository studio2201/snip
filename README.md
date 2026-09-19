# Snip

[![studio2201 Suite](https://img.shields.io/badge/studio2201-5%2F5%20Verified-2f6f5e?logo=shield)](https://studio2201.com/agents#badges)
[![Release](https://img.shields.io/badge/version-v0.2.8-blue.svg)](https://github.com/studio2201/snip/releases)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

<details>
<summary><a href="https://studio2201.com/agents#badges"><img src="https://img.shields.io/badge/studio2201-5%2F5%20Verified-2f6f5e?logo=shield" alt="studio2201 Suite"></a> <b>Detailed Governance Scorecard</b></summary>

| Tool | Focus | Verdict | Status Badge |
| :--- | :--- | :---: | :---: |
| [**Snip**](https://studio2201.com/snip) | Vibe-Code & Secrets Gate | `SHIP` | [![Vibe-Safe](https://img.shields.io/badge/vibe--safe-SHIP-brightgreen.svg)](https://studio2201.com/snip) |
| [**Vigil**](https://studio2201.com/vigil) | Supply-Chain Dormancy | `HEALTHY` | [![Dormancy](https://img.shields.io/badge/dormancy-healthy-2f6f5e.svg)](https://studio2201.com/vigil) |
| [**Aegis**](https://studio2201.com/aegis) | PQC & Post-Quantum Scans | `QUANTUM-SAFE` | [![PQC](https://img.shields.io/badge/PQC-Quantum--Safe-blueviolet.svg)](https://studio2201.com/aegis) |
| [**Proven**](https://studio2201.com/proven) | ML-DSA-65 Attestation | `VERIFIED` | [![SLSA](https://img.shields.io/badge/SLSA-Level%203%2B-blue.svg)](https://studio2201.com/proven) |
| [**Boneyard**](https://studio2201.com/boneyard) | Tech-Debt Radar | `0/100 DEBT` | [![Boneyard](https://img.shields.io/badge/boneyard%20index-0%2F100-brightgreen.svg)](https://studio2201.com/boneyard) |

</details>

**Vibe-code security gate.** Pre-deploy audit for AI-generated code diffs at the moment of generation.

## Why This Action Is Needed

### The Vibe-Coding Credential & Database Exposure Risk
AI coding agents (Cursor, Claude Code, Windsurf, Copilot, Devin) generate massive multi-file changes in seconds. While accelerating velocity, LLMs frequently hallucinate live production credentials into frontend bundles, generate permissive CORS configurations, and emit SQL schemas without Supabase Row Level Security (RLS).
- **[GitGuardian State of Secrets Sprawl](https://www.gitguardian.com/state-of-secrets-sprawl)**: Security telemetry demonstrates that AI-assisted commits leak API keys and private credentials at more than double the human baseline (3.2% vs 1.5%).
- **[Supabase RLS Security Architecture](https://supabase.com/docs/guides/database/postgres/row-level-security)**: Tables created without `ENABLE ROW LEVEL SECURITY` expose entire customer datasets to public anon API keys.
- **Automated CI Gates vs Manual Discipline**: Human reviewers reviewing 1,000-line AI diffs suffer from severe cognitive fatigue, easily overlooking a single live key or missing RLS statement. Snip provides an automated CI gate evaluating diff additions (`+`) in sub-5ms, failing closed on policy violations.

## Autonomous Agent Integration

Configure Snip in your repository via an AI coding assistant or directly add the workflow below.

### Prompt for your AI Agent

Copy and paste this prompt to Cursor, Claude Code, Copilot Workspace, or Devin:

```text
Add a GitHub Actions workflow to this repository at .github/workflows/snip.yml using studio2201/snip@master. Trigger on pull requests and pushes to master, audit git diffs for hardcoded secrets, missing Supabase RLS, and permissive CORS, publish the findings to the GitHub Step Summary, and block pull requests that fail the security gate.
```

### GitHub Actions Workflow

Commit this complete, production-ready workflow at `.github/workflows/snip.yml`:

```yaml
name: Snip Vibe-Code Security Gate
on:
  pull_request:
    branches: [ master, main ]
  push:
    branches: [ master, main ]
permissions:
  contents: read
jobs:
  snip-audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - name: Run Snip Diff Audit
        uses: studio2201/snip@master
        with:
          path: '.'
          format: 'markdown'
```

## How It Works Under the Hood

1. **Streaming Diff Lexer (`src/audit.rs`)**: Reads unified git diffs from stdin or file paths, focusing strictly on added lines (`+`) to prevent reviewing unchanged legacy code.
2. **High-Confidence Credential Patterns**: Zero-dependency pattern matcher detecting live Stripe (`sk_live_`), OpenAI (`sk-proj-`), GitHub (`ghp_`), AWS Access Keys (`AKIA`), Anthropic (`sk-ant-`), and PEM private keys.
3. **Database RLS Enforcer**: Inspects SQL migration diffs for table definitions lacking explicit Row Level Security enforcement.
4. **Deterministic Policy Gate (`src/gate.rs`)**: Emits `SHIP` (exit 0) when clean, `FIX` (exit 1) for warnings, or `BLOCK` (exit 1) on critical leaks.
5. **Native Model Context Protocol Server (`src/serve.rs`)**: Runs as a stdio MCP JSON-RPC server (`snip serve --mcp`), allowing Cursor and Claude to run pre-commit diff gates automatically.

## Quick Start

```bash
# Install via studio2201 installer
curl -fsSL https://studio2201.com/install.sh | sh -s snip

# Audit staged git diff
git diff --staged | snip audit

# Audit directory or path
snip check --path ./src

# Run as Model Context Protocol (MCP) server for Cursor / Claude
snip serve --mcp

# Run system diagnostics
snip doctor
```

## CLI Commands

- `snip audit` — Audit diff from stdin or file
- `snip check --path <PATH>` — Audit directory or file
- `snip serve --mcp` — Run stdio MCP server
- `snip doctor` — Run 7-point system diagnostics
- `snip update` / `snip upgrade` — Self-update binary
- `snip -h` / `--help` — Show help
- `snip -V` / `--version` — Show version

## Badges & Status

Certify that your vibe-coded repository passes Snip's security gate:

```markdown
<!-- Vibe-Safe Security Gate Verdict -->
[![Vibe-Safe](https://img.shields.io/badge/vibe--safe-SHIP-brightgreen.svg)](https://studio2201.com/snip)

<!-- Model Context Protocol (MCP) Server Status -->
[![MCP Protocol](https://img.shields.io/badge/MCP-2024--11--05-blue.svg)](https://studio2201.com/snip)
```

## License

Apache-2.0.
