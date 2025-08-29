# Chute_kun

TaskChute メソッドを実装した Todoist タスク管理 CLI ツール

![License](https://img.shields.io/badge/license-MIT-green.svg)

TaskChute の考え方を Todoist API と連携し、コマンドラインから効率的なタスク管理を実現します。

## 特徴

- **シンプルな CLI インターフェース** - コマンドラインから Todoist タスクを管理
- **タスクフィルタリング** - 様々な条件でタスクを絞り込み
- **優先度管理** - TaskChute の優先度付け手法の適用（開発中）
- **時間管理** - タスクの所要時間計測と予測（開発中）
- **タイムブロック生成** - 最適なスケジュール提案（開発中）

## Rust TUI テンプレート（ratatui + crossterm）

このリポジトリには Rust 製 TUI の最小テンプレートが含まれます。

- エントリポイント: `src/cli/main.rs`
- 再利用ロジック: `src/lib/`
- テスト: `tests/`（`cargo test` を使用。追加の scripts は不要）

実行:

```
cargo run
```

テスト:

```
cargo test
```

## ライセンス

MIT

## 関連ドキュメント

- [TaskChute methodology](docs/TaskChute_methodology.md)
- [ドキュメント一覧](docs/README.md)
- [システム概要](docs/system-overview.md)
- [実装状況](docs/setup/implementation-status.md)
- [開発計画](docs/planning/development-plan.md)
