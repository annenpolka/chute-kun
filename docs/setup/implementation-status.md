# 実装状況（2025-08-30 現在）

## サマリ
- スタック: Rust + `ratatui 0.29`（UI）, `crossterm 0.29`（端末制御）
- 進捗: タスク追加/選択/開始・一時停止・完了、翌日送り、並べ替え、履歴ビュー、ESD/ヘッダ表示を実装。
- 実績時間: 秒単位の計測を導入（ループで経過ミリ秒→秒へ集約）。各タスクに秒を保持し、60秒で分に繰上げ。Pause→Resumeでも部分秒は保持。
- テスト: ユニット/レンダリングスモークに加え、`tick` の秒→分繰上げ、停止→再開時の部分秒リセットを検証。
- スクリプト: なし（`cargo run` / `cargo test` を利用）

## 実行・テスト
- 実行: `cargo run`（`Enter` で開始/一時停止をトグル、`Shift+Enter` 完了、`p` 翌日送り、`q` 終了）
- テスト: `cargo test`

## 主要ファイル
- `src/cli/main.rs`: 端末初期化/終了、描画ループ、入力ポーリング、秒単位 `tick` 呼び出し
- `src/lib/app.rs`: アプリ状態、キー操作、`tick(seconds)` による実績加算（60秒で1分）
- `src/lib/ui.rs`: ヘッダ（ESD/Est/Act/View）とタスクリスト描画
- `src/lib.rs`: 上記モジュールのエクスポート（パス指定）

## 依存関係
- ランタイム/端末: `crossterm = 0.29`
- UI: `ratatui = 0.29`
- 設定/シリアライズ: `toml = 0.9`, `serde = 1`
- パス解決: `dirs = 6`
- 文字幅: `unicode-width = 0.2`
- エラー/診断: `anyhow`, `color-eyre`, `tracing`, `tracing-subscriber`

## テスト現況
- `tests/app_test.rs`:
  - 初期状態の検証（タイトル、終了フラグ）
  - `q` キーで終了フラグが立つ
- `tests/ui_draw_test.rs`:
  - `TestBackend` を用いた描画スモークテスト（エラーなく1フレーム描画）

## 設計メモ（現状）
- 入力処理: `crossterm::event::poll/read` によるポーリング（100ms）
- 描画: 毎ループで `Terminal::draw` により全描画（差分適用は未実装）
- 終了: `App::handle_key` が `q` を受け取り `should_quit = true`

## 未実装（バックログ）
- UI: 詳細ペイン/セッション一覧/レイアウト強化
- ドメイン: Todoist 同期、設定読取
- テスト: レンダリングバッファ直接アサート、境界ケースの充実

## 参照
- ADR-003: Rust製TUIライブラリ選定（ratatui + crossterm 採用）
- `docs/features/tui-testlist-v1.md`（TDD用テストリスト）
- `docs/features/tui-app-spec-v1.md`（MVP仕様）
