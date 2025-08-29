# Repository Guidelines

This guide explains how to contribute to the Chute_kun repository (TaskChute + Todoist CLI). Keep changes small, documented, and test‑driven. Tooling and language are intentionally stack‑agnostic until re‑selected.

## Project Structure & Module Organization
- `docs/`: ADRs, processes, templates, feature specs (authoritative).
- `src/` (when code is added): CLI entry under `src/cli/`, reusable logic in `src/lib/`.
- `tests/`: Unit/integration tests mirroring `src/` (e.g., `tests/lib/...`).
- `scripts/`: Developer utilities (e.g., `dev.sh`, `test.sh`, `format.sh`).

## Build, Test, and Development Commands
- Standardize via scripts in `scripts/` so they remain stable across stacks.
- Examples: `./scripts/dev.sh` (run locally), `./scripts/build.sh` (produce artifacts), `./scripts/test.sh` (execute tests).
- If a command is missing, add a script and update this section rather than documenting tool‑specific invocations.

 

## Testing Guidelines
- Approach: Follow TDD (t_wada) Red → Green → Refactor. Start with a failing, executable test; make it pass minimally; refactor with tests green. See `docs/adr/ADR-002-TDD-Development-Approach.md` and `docs/processes/development-workflow.md`.
- Framework: select per stack; name tests `*.spec.*` or `*_test.*` and mirror `src/` layout.
- Coverage: target ≥ 80% for core modules; include edge/boundary cases early.

## Commit & Pull Request Guidelines
- Messages (Conventional): `feat|fix|docs|style|refactor|test|chore: summary` (see `docs/processes/commit-strategy.md`).
- Branches: `{type}/{short-scope}` (e.g., `feat/cli-init`).
- PRs: clear description, linked issues, before/after or sample CLI output, updated docs (feature spec/ADR as needed).
- Keep commits atomic: code + tests + docs together.

## Security & Configuration Tips
- Secrets: never commit secrets. Use local `.env` (git‑ignored). Provide `.env.example` with keys only.
- Configuration lives in env vars; the app must read from env, not source control.

By contributing, you agree to keep docs current with behavior. When in doubt, update the relevant file in `docs/` first, then implement.
