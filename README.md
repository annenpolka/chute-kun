# Chute_kun

TaskChute をローカルで素早く回す Rust 製 TUI（ratatui + crossterm）

![License](https://img.shields.io/badge/license-MIT-green.svg)

現状は「ローカル保存・オフライン完結」の最小実装です。Todoist など外部 API 連携や LLM 連携は未実装（将来の検討対象）です。

## 特徴（現状）

- **TUI 操作**: Today/Past/Future のタブ切替、行選択ハイライト、ヘルプ行表示
- **時間管理**: 秒単位の実績計測と 60 秒での分繰り上げ、見込み終了時刻（ESD）をヘッダに表示
- **基本操作**: Start/Pause/Resume/Finish（選択タスク）、Reorder、Estimate 編集（±5m / 右クリック）、Postpone（翌日へ）、Bring from Future、Delete（確認ダイアログ）
- **入力**: `i` 新規、`I` 割り込み。IME 日本語入力と貼り付けに対応
- **マウス対応**: 行ホバー、クリック選択、ダブルクリックで開始/一時停止、右クリックで見積エディタ
- **スナップショット保存**: 終了時に TOML（`SnapshotV1`）へ自動保存（パスは XDG 風）

詳細は `docs/setup/implementation-status.md` を参照（最終更新: 2025-08-30）。

## Rust TUI テンプレート（ratatui + crossterm）

- エントリポイント: `src/cli/main.rs`
- 再利用ロジック: `src/lib/`
- テスト: `tests/`（`cargo test`）

## 使い方

実行:

```
cargo run
```

テスト:

```
cargo test
```

設定（任意）:

```
chute_kun --init-config
chute_kun --set-day-start HH:MM   # 予定の基準時刻を変更（config.toml を更新）
```

TUI から変更（永続化）:

```
起動中に `:` を押してコマンドパレット → `base HH:MM` または `base HHMM` → Enter（`config.toml` に保存）
```

生成場所やキー設定の詳細は `docs/setup/configuration.md` を参照。

## ライセンス

MIT

## 関連ドキュメント

- [ドキュメント一覧](docs/README.md)
- [システム概要（現状）](docs/system-overview.md)
- [実装状況](docs/setup/implementation-status.md)
- [開発計画](docs/planning/development-plan.md)
