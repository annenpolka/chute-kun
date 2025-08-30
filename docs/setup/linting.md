# Linting & Formatting

本リポジトリのリンターは Rust 標準ツールチェーン（`rustfmt` と `clippy`）を使います。ローカルとCIの両方で同じコマンドが動くように設定しています。

## 必要条件
- Rust `stable`（`rustup` 推奨）。`rust-toolchain.toml` により自動で安定版が選択されます。
- コンポーネント: `rustfmt`, `clippy`（`rustup component add rustfmt clippy`）

## コマンド
- フォーマット適用: `cargo fmt --all`
- フォーマット検査: `cargo fmt --all -- --check`
- リント（警告をエラーとして扱う）: `cargo lint`
  - `cargo lint` は `.cargo/config.toml` のエイリアスです（`cargo clippy --workspace --all-targets --all-features -- -D warnings`）。

## CI
- GitHub Actions で以下を実行します：
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo test --workspace --all-targets --all-features`

## 補足
- `rustfmt.toml` は安定版のオプションのみを使用しています。
- 強めのルールを導入したい場合は、PRで議論のうえ `clippy.toml`/crate attributes の追加を検討してください。

## Git Hooks（Husky 推奨）
- デフォルトのフックは Husky 形式（`.husky/` ディレクトリ）です。
  - 有効化（クローン直後に1回）: `git config core.hooksPath .husky`
  - `pre-commit`: `cargo fmt-check` と `cargo lint` を実行します。
  - `pre-push`: `cargo test --workspace --all-features --all-targets` を実行します。
    - テストを一時的にスキップ: `SKIP_CARGO_TEST=1 git push`

## 代替: pre-commit ツール
- 既存の `.pre-commit-config.yaml` も利用できます（任意）。
  - インストール: `pipx install pre-commit` または `pip install pre-commit`
  - 有効化: `pre-commit install`
  - 動作は上記 Husky と同等（fmt/clippy、任意で test）です。

## 代替: 素の Git hooks
- ツールを使わない場合は `.githooks/pre-commit` を利用できます。
  - 有効化: `git config core.hooksPath .githooks`
  - テストのスキップ: `SKIP_CARGO_TEST=1 git commit ...`
