# TUI アプリ仕様（初版）

*最終更新: 2025-08-29*

## 目的/スコープ
- タスクシュートの「プラン/ログ/ルーチン」を、Todoist と連携するTUIで実現する。
- MVPではLLMや高度な最適化を除外し、手動運用 + 正確なログと見積/実績の見える化を最優先。

## 画面構成
- ヘッダ: `YYYY-MM-DD (曜日) 現在時刻 | 見積合計/実績合計 | 見込み終了時刻 | 余裕(±分)`
- メインリスト（Today Queue）:
  - 表示列: `[state] title  est  act  ▲▽(順序)  ETA`（state: planned/active/paused/done）
  - フィルタ: セクション/プロジェクト/ラベル
- 詳細ペイン（トグル）:
  - タスク詳細: 説明、ラベル、プロジェクト/セクション
  - セッションログ: `開始-終了 (分) メモ`
- サイドペイン（トグル）:
  - Routine ライブラリ / History（過去n日）/ Inbox

## キー操作（MVP）
- Start/Resume: `Enter`
- Pause: `Space`
- Finish(Complete): `Shift+Enter`
- Navigate(選択移動): `↑/↓` または `j/k`
- Reorder: `Alt+↑/↓`（または `[`/`]`）
- Edit Estimate: `e`（数値分）
- Postpone(持越し): `p`（翌日へ/日時指定）
- Interrupt(割込作成のみ。開始は `Enter`): `i`
- Split(分割): `s`
- Toggle Detail: `d`
- Filter/Search: `/`
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
- `ESD(見込み終了時刻) = now + (active残 + キュー残の見積和)` を随時計算。
- オーバー/余裕は `ESD - 希望終業時刻`。

## Todoist 連携（プロトコル）
- 認証: API Token（環境変数）。
- 取得: 今日対象のタスク（プロジェクト/フィルタ条件は設定可能）。
- 更新: コメントに `tc-log` を追記、説明に `tc:` メタを保持。
- 完了: Todoist を完了状態に同期（任意設定で「TUI完了=Todoist完了」）。
- ルーチン: 繰り返し due を活用。必要に応じて TUI 側で当日生成をトリガー。

## エラー/オフライン指針
- オフライン時はローカルキューに書き込み、再接続で差分同期。
- 衝突は「最後のユーザー操作優先 + マージ（コメント追記）」を基本。

## MVP 受け入れ基準
- [ ] Today キュー表示（状態/見積/実績/ETA）
- [ ] Start/Pause/Resume/Finish が動作し、ログが Todoist コメントに残る
- [ ] Reorder/Estimate編集/Interrupt/持越しが可能
- [ ] ESDと余裕の算出が正しい（単体テストで検証）
- [ ] ルーチンの当日生成が行える（最小形）

## 非対象（初版）
- LLMによる見積自動化/優先度最適化
- マルチデイの自動再配置/自動リスケ
- 外部カレンダー双方向同期

## 参考
- 再調査メモ: `docs/research/taskchute-research-2025-08.md`
- Todoist API: `docs/adr/ADR-001-TaskManagementAPI-Selection.md`
