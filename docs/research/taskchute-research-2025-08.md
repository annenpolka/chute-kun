# タスクシュート再調査メモ（2025-08）

- 目的: 既存ドキュメントの補強と、TUIアプリ設計に必要十分な一次情報を整理する。
- 対象: メソッドの核（プラン/ログ/ルーチン）、運用原則、データ構造、他手法との境界、実装に効く要件。
- 参考: 公式情報（TaskChute Cloud/タスクシュート協会）を主とし、歴史的背景は提唱者の発信を参照。

## 1. コア概念（要約）

- プラン: 今日1日のタスクリストを実行順に並べ、各タスクに見積時間を付与し、合計から終業時刻（ETA/ESD）を把握する。
- ログ: タスクの開始/終了時刻を記録し、実績時間を保持する（完了しても削除しない）。
- ルーチン: 定期的に発生するタスクの雛形。ログから抽出・更新され、翌日以降のプランに自動展開される。

出典: タスクシュート協会の3機能解説、TaskChute Cloud 公式のサイクル説明。

## 2. 運用原則（TUIに落とすための要点）

- 単一実行原則: 原則として同時にアクティブなタスクは1つ。新規開始時は既存を一時停止または終了。
- シーケンシャル実行: 今日のキューは上から順に処理。順序変更は可能だが、常に「次にやる1つ」が明確。
- 時間の見える化: 残余見積の総和と現在時刻から「見込み終了時刻」を即時計算・提示。
- ログの不可逆性: 実績ログは改竄よりも追記（セッション追加/補正）を基本とする。
- ルーチンの漸進更新: 実績から所要時間・開始推奨時刻を学習し、以後の生成に反映。
- 中断/割込の扱い: 割込は独立タスクとして即時記録し、元タスクは自動一時停止。終了後に復帰操作が容易であること。
- 境界の越日処理: 日付境界で未完了は「翌日へ持ち越す/分割して完了」いずれかを選べる。

## 3. 最低限のデータモデル（概念）

- Task: `id, title, estimate_min, priority, project, section, labels, notes`。
- Session(Log): `task_id, started_at, ended_at, duration_min, note, source (manual/timer)`。
- DayPlan: `date, ordered_task_ids[], total_estimate_min, total_actual_min, eta, overflow_min`。
- Routine: `template_id, title, default_estimate_min, cadence (cron/weekday), anchor_time (optional)`。
- State: `planned | active | paused | done | dropped`（Task単位）。

## 4. Todoist マッピング（実装ガイドライン）

- Task ⇔ Todoist Task
  - 見積: 説明(description)に `tc: estimate=25m` などのキー=値形式、または `est_25m` ラベル。
  - カテゴリ/モード/タグ: Todoistラベル/プロジェクト/セクションを活用。
- Log ⇔ Todoist Comment
  - 例: `tc-log 2025-08-29 09:00-09:25 (25m) #deep`
- Routine ⇔ 繰り返しタスク or テンプレート
  - Todoistの繰り返し due を基本とし、必要に応じてラベル `routine` で識別。

注: 具体実装は後続ADRで最終決定。ここではTUI要件を満たす最小構成を提案。

## 5. 他手法との境界・補完

- GTD: 収集〜見直しのフローは補完関係。タスクシュートは「今日の実行」と「実績学習」に特化。
- タイムブロッキング/ポモドーロ: セクション/休憩挿入などと親和。TUIでは休憩の自動差込設定を提供予定。

## 6. 画面/操作に落とす要件（抜粋）

- ヘッダ: 日付/現在時刻、合計見積、実績、見込み終了時刻、オーバー/余裕。
- メインリスト: 今日のタスク（状態・見積・実績・予測終了時刻）。
- 詳細ペイン: ノート/ログセッション一覧/編集。
- 基本操作: Start/Stop/Pause/Resume、Reorder、Estimate編集、割込作成、持越し、完了、分割。

## 7. 参考資料（主要）

- TaskChute Cloud 公式: サイクル説明、歴史、思想 https://www.taskchute.cloud/index.html
- タスクシュート協会: 3機能の解説記事 https://blog.taskchute.cloud/the-three-taskchute-basics/
- 歴史・Excel起源（シゴタノ発信等）

（注）外部URLは将来の参照用。要約は原典の表現を損なわない範囲で再構成。

