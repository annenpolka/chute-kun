# TUI アプリ仕様（初版 / 現状）

最終更新: 2025-08-30

## 目的/スコープ
- タスクシュートの「プラン/ログ」をローカル TUI で実現する。
- 現在の MVP はオフライン完結（TOML スナップショット保存）。外部 API/LLM は範囲外（将来の拡張）。

## 画面構成
- ヘッダ: `YYYY-MM-DD` ベース情報 + 見積合計/実績合計 + ESD（見込み終了時刻）
  - 右上: アクションボタン（New | Start | Stop | Finish | Delete）
    - 表示: 太字の黒文字＋色付きピル（有効時）。無効時は灰文字（背景なし・非太字）。
    - ホバー: シアン背景でハイライト（有効時のみ）。
    - クリック: 有効時のみ動作（無効時は抑止）。
- メインリスト（Today Queue／ratatui::Table）:
  - カラム: `Plan(予定時刻)` / `Actual(実測要約)` / `Task(状態+タイトル+見積/実績)`
  - 選択行の背景ハイライト、ホバー表示、空時のヒント表示

### Plan 列の算出ルール（重要）
- 基準: コンフィグの `day_start`（例: 09:00）を起点に、上から順にタスクの所要時間を累積して各行の予定開始を算出する。
- 所要時間の扱い:
  - `Done`: 見積（`estimate_min`）を用いる。完了済みであっても、後続タスクの予定を押し出す要素として見積時間を加味する。
  - `Active/Paused/Planned`: 残り時間（`estimate_min - actual_min`）を用いる。
- 例: 09:00 起点で `A(30m)`, `B(20m)` の順。`A` が `Done` の場合でも `B` の Plan は 09:30 になる。
- 詳細/フィルタ等は将来拡張（現状は未実装）

## キー操作（既定）
- Start/Pause/Resume: `Enter`
- Finish（選択タスク）: `Shift+Enter`（代替: `f`）
- Navigate: `↑/↓` または `j/k`
- Reorder: `[` / `]`
- Edit Estimate: `e`（±5m ステッパー）
  - Date Picker: `.` で +1 日、`,` で −1 日（今日より前にはならない）。Date 行に曜日付きで表示（例: `Today (Wed)` / `YYYY-MM-DD (Fri)`）。マウスは `<`/`>` クリックで変更可能。
- Postpone（翌日へ）: `p`
- Bring from Future: `b`
- New Task: `i`（入力モード）
- Interrupt: `I`（入力モード／デフォ 15m）
- Delete（確認ダイアログ）: `x`（Enter=Delete / Esc=Cancel）
- Quit: `q`

（注）キーバインドは後続実装で微調整可。原則として片手操作と視線移動最小化を重視。

### ヘルプ表示とキーバインド
- 画面下部のヘルプ行は現在のコンフィグ（`config.toml`）のキーアサインを反映して表示されます。
- 例: `finish_active = "g"` の場合、Today ビューでは `g: finish`。`view_next = "Ctrl+N"` の場合は表示が正規化され `Ctrl+n: switch view`。
- 複数割り当ては `/` 区切りで表示: 例）`finish_active = ["Shift+Enter", "f"]` → `Shift+Enter/f: finish`。
- Delete もコンフィグ可能（例: `delete = "Ctrl+d"` → `Ctrl+d: delete`）。
- `BackTab` はヘルプ上は `Shift+Tab` として表示されます。設定は `BackTab`/`Shift+Tab` のどちらでも可。

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
