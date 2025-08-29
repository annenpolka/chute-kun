# Repository Guidelines

This guide explains how to contribute to the Chute_kun repository (TaskChute + Todoist CLI/TUI). Keep changes small, documented, and test‑driven. Documentation in `docs/` is authoritative.

## TDD Approach
- Follow t_wada’s Red → Green → Refactor. Start with a failing, executable test; make it pass minimally; refactor while keeping tests green. See `docs/adr/ADR-002-TDD-Development-Approach.md` and `docs/processes/development-workflow.md`.
- 日本語定義（要約）:
  1. テストリストを作る
  2. ひとつ選び実行可能なテストに翻訳し、失敗を確認する（Red）
  3. プロダクトコードを変更し、全テストを成功させる（Green）
  4. 必要に応じてリファクタリング（Refactor）
  5. テストリストが空になるまで繰り返す

## Project Structure & Module Organization
- `docs/`: ADRs, processes, templates, feature specs（authoritative）。
- `src/cli/`: CLI/TUI entry (`main.rs`).
- `src/lib/`: Reusable logic（`app.rs`, `ui.rs` など）。必要に応じ `event.rs`, `update.rs`, `tui.rs`, `components/` を追加。
- `tests/`: ユニット/統合テストは `src/` をミラー。命名は `*_test.rs` 推奨。

## Rust Build & Test (no scripts)
- Use Cargo directly; do not add local shell scripts for build/test.
- Run: `cargo run`
- Test: `cargo test`
- Formatting/lints（必要なら）: `cargo fmt`, `cargo clippy`

## Rust TUI Stack & Architecture
- Stack: `ratatui`（UI） + `crossterm`（terminal I/O）。選定根拠は `docs/adr/ADR-003-Rust-TUI-Library-Selection.md` と `docs/research/2025-08-rust-tui-landscape.md` を参照。
- Responsibility separation（推奨）: `app`（状態）/`ui`（描画）/`event`（入力/タイマー）/`update`（状態更新）/`tui`（端末ラッパ）。詳細は `docs/processes/tui-architecture-and-naming.md`。

## Naming Conventions (Rust/Ratatui)
- Types: CamelCase（例: `App`, `TaskList`, `StatusBar`）。
- Modules/funcs: snake_case（例: `handle_key_events`, `render_status_bar`）。
- Constants/env: SCREAMING_SNAKE_CASE（例: `DEFAULT_TICK_RATE_MS`）。
- Common fields: `should_quit` を終了フラグ名として統一。

## Testing Guidelines
- TDD first: 新規機能は必ず失敗するテストから。描画は `ratatui::backend::TestBackend` と `Buffer` 比較で検証可能。
- Coverage: コアモジュールは概ね ≥ 80% を目安に。早期に境界条件テストを追加。

## Commit & Pull Request Guidelines
- Messages (Conventional): `feat|fix|docs|style|refactor|test|chore: summary`。詳細は `docs/processes/commit-strategy.md`。
- Branches: `{type}/{short-scope}`（例: `feat/cli-init`）。
- PRs: 目的/変更点/動作確認/関連ドキュメント（ADR/仕様/実装状況）の更新を含める。
- Keep commits atomic: コード + テスト + ドキュメントを同時に更新。

## Security & Configuration Tips
- Secrets: never commit secrets. `.env` は git ignore 済み。必要なら `.env.example` にキー名のみ記載。
- Configuration lives in env vars; the app must read from env, not source control.

By contributing, you agree to keep docs current with behavior. When in doubt, update the relevant file in `docs/` first, then implement.
