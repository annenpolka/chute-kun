# TUI アプリ仕様（初版 / 現状）

最終更新: 2025-08-30

## 目的/スコープ
- タスクシュートの「プラン/ログ」をローカル TUI で実現する。
- 現在の MVP はオフライン完結（TOML スナップショット保存）。外部 API/LLM は範囲外（将来の拡張）。

## 画面構成
- ヘッダ: `YYYY-MM-DD` ベース情報 + 見積合計/実績合計 + ESD（見込み終了時刻）
- メインリスト（Today Queue／ratatui::Table）:
  - カラム: `Plan(予定時刻)` / `Actual(実測要約)` / `Task(状態+タイトル+見積/実績)`
  - 選択行の背景ハイライト、ホバー表示、空時のヒント表示
- 詳細/フィルタ等は将来拡張（現状は未実装）

## キー操作（既定）
- Start/Pause/Resume: `Enter`
- Finish（選択タスク）: `Shift+Enter`（代替: `f`）
- Navigate: `↑/↓` または `j/k`
- Reorder: `[` / `]`
- Edit Estimate: `e`（±5m ステッパー）
- Postpone（翌日へ）: `p`
- Bring from Future: `b`
- New Task: `i`（入力モード）
- Interrupt: `I`（入力モード／デフォ 15m）
- Delete（確認ダイアログ）: `x`（Enter=Delete / Esc=Cancel）
- Quit: `q`

（注）キーバインドは後続実装で微調整可。原則として片手操作と視線移動最小化を重視。

## 状態遷移（タスク）
- `planned → (start) → active → (pause) → paused → (resume) → active → (finish) → done`
- 例外: `planned → (finish:記録のみ) → done`、`active → (drop) → planned|dropped`、`active → (interrupt) → paused`。
- 制約: 同時 `active` は1件。新規 `start` は既存 `active` を自動 `pause`。

## タイマーとログ
- アクティブ時はモノトニックタイマーで秒刻み計測。タスクは複数セッションを持てる。
- 休憩は「タスクなし区間」として扱い、必要なら `break` タグで記録。
- 手動補正を許容（開始/終了の再入力、加減算）。

## 見積と終了見込み
- `ESD(見込み終了時刻) = now + キューの見積合計` を随時計算（実績の進捗とは連動しない）。
- オーバー/余裕は `ESD - 希望終業時刻`。

### 入力モード（New/Interrupt）
- 画面下部に `Input: <buffer> (Enter=Add Esc=Cancel)` を表示。
- デフォルト見積: New Task=25m、Interrupt=15m。
- タイトル未入力で `Enter` の場合はそれぞれ `New Task` / `Interrupt` で作成。

## 外部連携（将来）
- Todoist 取得/完了/コメント追記、ルーチン当日生成 等はバックログ。

## エラー/オフライン指針
- オフライン時はローカルキューに書き込み、再接続で差分同期。
- 衝突は「最後のユーザー操作優先 + マージ（コメント追記）」を基本。

## MVP 受け入れ基準（現状）
- [x] Today キュー表示（状態/見積/実績/ESD）
- [x] Start/Pause/Resume/Finish が動作し、セッション/実績が更新される
- [x] Reorder/Estimate 編集/Interrupt/持越し/Bring が可能
- [x] ESD の計算が正しい（単体テストで検証）
- [x] スナップショットの保存/読み込み（TOML）

## 非対象（初版）
- LLM による見積自動化/優先最適化
- マルチデイの自動再配置/自動リスケ
- 外部カレンダー双方向同期

## 参考
- 再調査メモ: `docs/research/taskchute-research-2025-08.md`
- Todoist API（将来検討）: `docs/adr/ADR-001-TaskManagementAPI-Selection.md`
