# Snip

[![CI](https://github.com/studio2201/snip/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/studio2201/snip/actions/workflows/ci.yml)
[![Release](https://img.shields.io/badge/version-v0.2.4-blue.svg)](https://github.com/studio2201/snip/releases)
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

## Quick Start

```bash
# Install via studio2201 installer
curl -fsSL https://studio2201.com/install.sh | sh -s snip

# Audit staged git diff
git diff --staged | snip audit

# Run as Model Context Protocol (MCP) server for Cursor / Claude
snip serve --mcp
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

## What it does

Run against the output of any vibe-coding platform (Cursor / Claude Code / Windsurf / Cline / Bolt / Lovable / Replit / v0). Emits a "Ship / Fix / Block" verdict:

- Hardcoded API keys (Stripe, OpenAI, GitHub, AWS, Anthropic…)
- Unauthenticated public endpoints
- Missing CSP / permissive CORS headers
- Accidental PQC bypasses
- Broken Supabase Row Level Security (RLS)

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

## Why

- GitGuardian 2026: 3.2% of AI-assisted commits leak credentials vs 1.5% baseline.
- 21% of AI-generated apps carry security bugs.
- Pure Rust, `std::` only. Zero crates.io dependencies. Strictly <= 256 LOC per source file.

## License

Apache-2.0.
