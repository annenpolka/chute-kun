# TaskChute システム概要（現状）

最終更新日: 2025-08-30

## 目的
TaskChute の「プラン・ログ・ルーチン」のうち、まずは「プラン/ログ」をローカル環境だけで高速に回す最小の TUI を提供します。外部 API（Todoist 等）や LLM 連携は今後の拡張対象です。

## 現状の基本機能
- Today/Past/Future の 3 ビュー切替とタスクリスト操作
- Start/Pause/Resume/Finish（選択タスク）と実績の秒→分繰り上げ
- 見積編集（±5m / j,k / 右クリック）と ESD（見込み終了時刻）のヘッダ表示
- 入力モード（新規/割込）と IME 日本語入力・貼り付け対応
- 並べ替え・翌日送り・Future からの Bring・削除確認ダイアログ
- 終了時に TOML スナップショットへ自動保存（XDG 準拠の既定パス）

## アーキテクチャ（Rust TUI）

```
┌──────────────────────────────┐
│  CLI/TUI (ratatui + crossterm) │  main.rs: 端末初期化/イベントループ/描画
└───────────────┬───────────────┘
                │
                ▼
┌──────────────────────────────────────────────┐
│  lib                                         │
│  ├─ app.rs     — アプリ状態/操作（選択/入力/遷移/秒積算）      │
│  ├─ ui.rs      — 描画（Frame←App）・ヒットボックス・ヘルプ       │
│  ├─ task.rs    — Task/DayPlan/Session/ESD 計算                   │
│  ├─ storage.rs — SnapshotV1(TOML) の保存/読込と XDG パス解決     │
│  ├─ config.rs  — day_start とキーマップの TOML 読取/雛形書出し   │
│  ├─ clock.rs   — 依存性注入可能なローカル時刻（分）              │
│  └─ date.rs    — `YYYYMMDD` のローカル日付ユーティリティ          │
└──────────────────────────────────────────────┘
```

設計の要点:
- UI は `ui::draw` の純粋関数を基本とし、テストでは `TestBackend` と `Buffer` 比較を使用
- 実績の秒積算はメインループで `Instant` を用いて実装（60 秒で 1 分繰り上げ）
- IME 互換性のため、キーボード拡張は `DISAMBIGUATE_ESCAPE_CODES | REPORT_EVENT_TYPES` のみを使用（ADR-004）

## データストレージ
- 形式: TOML（`SnapshotV1`）
- 構造: `today[]`, `future[]`, `past[]` の配列（順序保持）
- フィールド（Task 抜粋）: `title`, `estimate_min`, `actual_min`, `state`, `actual_carry_sec`, `started_at_min`, `finished_at_min`, `sessions[]`, `done_ymd`
- 保存タイミング: アプリ終了時に自動保存（失敗はログ出力）
- 仕様: `docs/specs/chute-snapshot-format-v1.md`

## 今後の拡張候補（未実装）
- Todoist との双方向同期（取得/完了/コメント追記）
- ルーチンの当日生成・繰り返し
- 詳細ペイン/セッション一覧/フィルタリング
- LLM による見積/優先度提案

本ドキュメントは現状（2025-08-30）の実装を反映しています。拡張時は本ページを更新します。
