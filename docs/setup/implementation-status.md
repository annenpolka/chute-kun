# 実装状況（2025-08-29 現在）

## サマリ
- スタック: Rust + `ratatui 0.26`（UI）, `crossterm 0.27`（端末制御）
- 進捗: 最小のTUIテンプレートを実装（描画・イベントループ・`q` で終了）
- テスト: Cargo 標準のテストランナーでユニット/描画スモークを実行
- スクリプト: なし（`cargo run` / `cargo test` を利用）

## 実行・テスト
- 実行: `cargo run`（`q` で終了）
- テスト: `cargo test`

## 主要ファイル
- `src/cli/main.rs`: 端末初期化/終了、描画ループ、入力ポーリング
- `src/lib/app.rs`: アプリ状態（`title`, `should_quit`）と `handle_key`
- `src/lib/ui.rs`: 単一の枠ウィジェットを描画（タイトルは状態から）
- `src/lib.rs`: 上記モジュールのエクスポート（パス指定）

## 依存関係
- ランタイム/端末: `crossterm = 0.27`
- UI: `ratatui = 0.26`
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
- UI: リスト/テーブル/詳細ペイン、レイアウト、ステータスライン
- 入力: Start/Pause/Resume/Finish 他のキーバインド
- ドメイン: 見積/実績・ESD 計算、セッションモデル
- 同期: Todoist API 連携（取得/更新/完了の反映）
- 設定: 環境変数/設定ファイルの読込
- テスト: レンダリングバッファのアサート、ドメインの境界ケース

## 参照
- ADR-003: Rust製TUIライブラリ選定（ratatui + crossterm 採用）
- `docs/features/tui-testlist-v1.md`（TDD用テストリスト）
- `docs/features/tui-app-spec-v1.md`（MVP仕様）

