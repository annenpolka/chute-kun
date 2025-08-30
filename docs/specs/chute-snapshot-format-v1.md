# Chute Snapshot Format v1 (TOML)

目的: タスク・スケジュールを「git-friendly」なテキストとして保存/復元するための仕様。差分が読みやすく、1 タスク = 1 ブロックでレビューしやすい構造を採用する。

- 形式: TOML（配列テーブル）
- バージョン: `version = 1`
- リスト: `[[today]]`, `[[future]]`, `[[past]]`（順序保持）
- Task フィールド（v1 現行実装）:
  - `title: string`
  - `estimate_min: u16`
  - `actual_min: u16`
  - `state: "Planned"|"Active"|"Paused"|"Done"`
  - `actual_carry_sec: u16`（既定 0）
  - `started_at_min: u16?`（初回開始の実測開始時刻）
  - `finished_at_min: u16?`（最終完了の実測終了時刻）
  - `done_ymd: u32?`（`YYYYMMDD`）
  - `sessions: [{ start_min: u16, end_min: u16? }, ... ]`

## サンプル

```toml
version = 1

[[today]]
title = "Deep work"
estimate_min = 50
actual_min = 10
state = "Active"
actual_carry_sec = 12
started_at_min = 540

[[today.sessions]]
start_min = 540

[[future]]
title = "Reply emails"
estimate_min = 20
actual_min = 0
state = "Planned"

[[past]]
title = "Daily review"
estimate_min = 15
actual_min = 15
state = "Done"
finished_at_min = 600
done_ymd = 20250829

[[past.sessions]]
start_min = 585
end_min = 600
```

## 設計方針

- Git diff 最適化: 1 タスク = 1 テーブルブロック（`[[today]]` など）で変更が最小行に収まる。
- 順序保証: 配列順序をそのまま保持（手動で上書き/レビューしやすい）。
- 冗長情報は保持しない: 選択中インデックスなど UI の一時情報は非保存。

## API（Rust）

- `storage::save_to_string(&App) -> Result<String>`: TOML 文字列にシリアライズ
- `storage::load_from_str(&str, Config) -> Result<App>`: TOML から `App` を復元
- `storage::save_to_path(&App, path) -> Result<()>`: ファイルに保存
- `storage::load_from_path(path, Config) -> Result<Option<App>>`: ファイルから読み込み（ファイルなしは `Ok(None)`）

将来拡張: `version` をインクリメントし、必要に応じてマイグレーションを実装。
