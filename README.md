# Snip

[![CI](https://github.com/studio2201/snip/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/studio2201/snip/actions/workflows/ci.yml)
[![Release](https://img.shields.io/badge/version-v0.2.5-blue.svg)](https://github.com/studio2201/snip/releases)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Pure std::](https://img.shields.io/badge/pure-std%3A%3A-success.svg)](https://studio2201.com)
[![Reproducible](https://img.shields.io/badge/reproducible-OK-brightgreen.svg)](tools/dev/repro.sh)
[![Max LOC](https://img.shields.io/badge/max%20LOC-%E2%89%A4256-brightgreen.svg)](https://studio2201.com)

[![Vibe-Safe](https://img.shields.io/badge/vibe--safe-SHIP-brightgreen.svg)](https://studio2201.com/snip)
[![Security Gate](https://img.shields.io/badge/security%20gate-PASSED-brightgreen.svg)](https://studio2201.com/snip)
[![MCP Protocol](https://img.shields.io/badge/MCP-2024--11--05-blue.svg)](https://studio2201.com/snip)
[![Secret Leaks](https://img.shields.io/badge/secret%20leaks-0%20detected-brightgreen.svg)](https://studio2201.com/snip)
[![Supabase RLS](https://img.shields.io/badge/Supabase%20RLS-AUDITED-brightgreen.svg)](https://studio2201.com/snip)

**Vibe-code security gate.** Pre-deploy audit for AI-generated code diffs at the moment of generation.

## Why This Matters & Authoritative Research

### 1. The Vibe-Coding Security Blindspot
AI coding agents (Cursor, Claude Code, Windsurf, Bolt, Lovable, v0) generate full-stack applications in seconds. However, LLMs routinely hallucinate live production credentials into frontend code, emit permissive CORS configurations, and omit database security boundaries.
- **[GitGuardian State of Secrets Sprawl](https://www.gitguardian.com/state-of-secrets-sprawl)**: Security research reveals that AI-assisted commits leak API keys and secrets at more than double the human baseline (3.2% vs 1.5%).
- **[OWASP Top 10 for Large Language Model Applications](https://owasp.org/www-project-top-10-for-large-language-model-applications/)**: Specifically targets LLM02 (Insecure Output Handling) and LLM06 (Sensitive Information Disclosure).
- **[Supabase Row Level Security Advisory](https://supabase.com/docs/guides/database/postgres/row-level-security)**: Unprotected PostgreSQL tables created without `ENABLE ROW LEVEL SECURITY` expose entire customer databases to public anon API keys.
- **[Model Context Protocol (MCP)](https://modelcontextprotocol.io/)**: Open specification allowing AI coding assistants to invoke external verification tools dynamically before saving files.

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

## GitHub Action Usage

Gate pull requests against credential leaks and broken RLS policies:

```yaml
- name: Snip Vibe-Code Security Gate
  uses: studio2201/snip@master
  with:
    path: '.'
    format: 'text'
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
