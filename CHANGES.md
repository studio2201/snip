# Changelog — snip

All notable changes to this project are documented here.
Format: [Keep a Changelog](https://keepachangelog.com/) 1.1.0.
This project adheres to [Semantic Versioning](https://semver.org/).

## [0.2.10] — 2026-09-19

### Changed
- Standardized centered 2-column Security & Governance Scorecard table `<div align="center">` in README.

## [0.2.9] — 2026-09-19

### Added
- Multi-platform static binary release packaging and two-stage publish workflow.

## [0.2.8] — 2026-09-19

### Added
- Embedded Option 2 detailed governance scorecard into `README.md`.

## [0.2.7] — 2026-09-19

### Changed
- Updated README header to unified Option 1 Single Suite Badge.
- Bumped version to 0.2.7.

## [0.2.6] — 2026-09-19

### Added
- Native composite GitHub Action (`action.yml`) with sub-2s fast bootstrap via `install.sh`.
- Native `$GITHUB_STEP_SUMMARY` Markdown scorecard and diff audit reporting.
- Human-first 'Why This Action Is Needed' rationale with GitGuardian/OWASP citations.
- Agent-first 'Prompt for your AI Agent' blocks and drop-in CI workflow YAML.

## [0.2.5] — 2026-09-18

### Added
- Expanded documentation in README with authoritative problem descriptions and citations:
  - GitGuardian State of Secrets Sprawl statistics on AI-generated code leaks.
  - OWASP Top 10 for Large Language Model Applications (LLM02 and LLM06).
  - Supabase Row Level Security (RLS) unprotected database vulnerabilities.
  - Anthropic Model Context Protocol (MCP) specifications.
- Added comprehensive "How It Works Under the Hood" architectural breakdown.
- Upgraded release metadata and diagnostic baseline.

## [0.2.4] — 2026-09-18

### Added
- Tool-specific badges on README: Vibe-Safe SHIP verdict, Security Gate status, MCP protocol compliance, Secret Leaks zero-count, and Supabase RLS audit.
- README guide for embedding Vibe-Safe and MCP badges in AI project repositories.
- Upgraded release metadata and diagnostic baseline.

## [0.2.0] — 2026-09-18

### Added
- Working pure `std::` Rust implementation of Snip vibe-code security gate.
- Fast static pass auditing diffs for leaked secrets (OpenAI, AWS, Stripe, GitHub, private keys), broken Supabase RLS, and permissive CORS.
- Immediate Ship/Fix/Block verdict evaluation with policy thresholds.
- Cursor dialect, generic terminal text, and JSON structured reporting.
- Standardized CLI flags: `-h/--help`, `-V/--version`, `--format`, `-o/--output`, `-q/--quiet`, `-v/--verbose`.
- Performance test verifying 200 KiB diff audited in < 0.5ms (budget 400ms).

## [0.1.2] — 2026-09-17

### Notes
- No content changes; snip README had no openOODA substrate references.
  Bumped to keep cadence with the v0.1.2 doctrine-level cleanup.

## [0.1.1] — 2026-09-17

### Added
- §15 threat model: `docs/threat-model.md` (snip-specific adversary:
  attacker crafting LLM-generated diffs that bypass static rules via
  novel encodings)
- §16 reproducible builds: `tools/dev/repro.sh` with per-host baselines
- §17 security disclosure: `SECURITY.md` pointing at GHSA tab
- §18 performance budgets: `tools/perf/budget.md` and
  `tests/integration.rs::perf_snip_check_within_budget` (std::time, median-of-5)

### Notes
- Pre-1.0.0: GHSA-only security advisories; CVEs reserved for 1.0.0+
- Budget defaults are first-cut placeholders, not aspirational

## [0.1.0] — 2026-09-17

### Added
- Initial scaffold: Apache-2.0 LICENSE, README, .gitignore
- One question (§0): "Did the AI leaky-anything it shouldn't have?"
