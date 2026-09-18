# Snip

**Vibe-code security gate.** Pre-deploy audit at the moment of generation.

**Status:** pre-release scaffold (2026-09-17). No source code yet.

## What it does

Run against the output of any vibe-coding platform (Cursor / Claude Code / Windsurf / Cline / Bolt / Lovable / Replit / v0). Emits a "Ship / Fix / Block" verdict with a one-pager in the platform's chat dialect. Detects:

- Hardcoded API-key shapes (Stripe, OpenAI, GitHub, AWS, Anthropic…)
- Unauthenticated endpoints
- Missing CSP / CORS
- Accidental PQC bypasses
- Broken Supabase RLS

Ships as: MCP server, GitHub Action, CLI, Cursor skill pack.

## Why

- GitGuardian 2026: 3.2% of AI-assisted commits leak credentials vs 1.5% baseline.
- 21% of AI-generated apps carry security bugs.
- Cursor (SpaceX $60B exit), Lovable ($6.6B), Bolt, Replit — they all face the same "vibe-coded app got popped" brand risk.

## Commercial plane

"Vibe-safe" badge attest partner program. Snyk / Semgrep / Aikido / Drata / Vanta integration.

## License

Apache-2.0.
