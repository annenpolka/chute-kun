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

