# Ratatui向け 責務分割ポリシーと命名規約

最終更新: 2025-08-29

本ドキュメントは、Ratatui を用いたTUIアプリの実装で一般的に採用される責務分割と命名のベストプラクティスを整理し、本リポジトリの標準とする。参考にした資料は末尾「参考」に記載。

## 責務分割ポリシー（推奨）
- 基本方針: 「状態（App）」「描画（UI）」「イベント」「端末制御（TUI）」「更新（Update）」を分離する。Ratatui Book の複数ファイル構成をベースにする。 
  - `app`（状態/モデル）: 画面全体の状態（選択、モード、タイマー、フラグ等）とドメイン操作の公開関数。
  - `ui`（描画/ビュー）: `draw(&mut Frame, &App)` のみ。副作用を持たない純粋関数に寄せる。
  - `event`（入力/タイマー）: `crossterm` からキー/マウス/リサイズ/タイマー `Tick` を購読・正規化。
  - `tui`（端末ラッパ）: raw mode、alternate screen、フレーム/バックエンド型のエイリアス (`type Frame = ratatui::Frame<'a>`) と入退出を隠蔽。
  - `update`（状態更新/MVU）: イベント（またはメッセージ）に応じて `App` を更新する関数群（The Elm Architecture 風）。
- ディレクトリ例（本リポジトリの方針に合わせて）
  - CLI入口: `src/cli/main.rs`
  - 再利用ロジック: `src/lib/{app.rs, ui.rs, event.rs, tui.rs, update.rs}`
  - 複雑化時は `src/lib/components/` に分割（後述の Component 方式）。

### 規模に応じた選択肢
- 小〜中規模: 上記「Multiple Files + MVU(update)」を既定値。テスト友好（状態と描画の分離）。
- 画面が増える/部品化が必要: Componentアーキテクチャ（各コンポーネントが `init/handle_*_events/update/render` を持つ）。
- 複雑な非同期や外部入出力が多い: Flux/Unidirectional Data Flow（`Action` → Dispatcher → Store → UI）で予測可能性を向上。

## 命名規約（Ratatui慣習に沿う）
- 型/構造体: `CamelCase`。例: `App`, `Action`, `Event`, `Tui`, `TaskList`, `StatusBar`。
- 関数/モジュール/ファイル: `snake_case`。例: `handle_key_events`, `render_status_bar`, `event.rs`。
- 定数/環境変数: `SCREAMING_SNAKE_CASE`。例: `DEFAULT_TICK_RATE_MS`。
- 端末ラッパ/別名:
  - `type Frame<'a> = ratatui::Frame<'a>` を `tui.rs` に集約。
  - `Tui` 構造体で `enter/exit/draw` を隠蔽（テンプレート慣習）。
- イベント/メッセージ名:
  - 入力: `Event::Key`, `Event::Mouse`, `Event::Resize`, `Event::Tick` を基本。更新側は `Action`/`Command`/`Message` のいずれかを選び、語彙はドメイン動詞で統一（例: `Action::Start`, `Action::Pause`, `Action::Finish`, `Action::ReorderUp`）。
  - ハンドラ: コンポーネント方式では `handle_events`, `handle_key_events`, `handle_mouse_events` を採用。
- アプリ終了フラグ: `should_quit`（テンプレートで広く使われる表現）。
- テスト命名/配置:
  - ユニット: `tests/app_test.rs`, `tests/ui_draw_test.rs`（本リポジトリ既存の慣習）。
  - バッファ比較: `Buffer::with_lines([...])` で描画の期待値を表現（色は制限あり）。

## 具体例（最小）
- `src/lib/app.rs`
  - `pub struct App { should_quit: bool, ... }`
  - `impl App { pub fn handle_key(&mut self, KeyCode) { ... } }`
- `src/lib/ui.rs`
  - `pub fn draw(f: &mut Frame, app: &App)` に限定。
- `src/lib/event.rs`
  - `pub enum Event { Key(KeyEvent), Mouse(MouseEvent), Resize(u16,u16), Tick }`
- `src/lib/update.rs`
  - `pub enum Action { Quit, Tick, Start, Pause, ... }`
  - `pub fn update(app: &mut App, action: Action)`

## 適用ガイド
- まず MVP では「Multiple Files + MVU」を既定とし、画面が増えたら `components/` に移行可能な設計にする。
- コンポーネントを導入する場合、各コンポーネントは「自身の状態 + 入力処理 + 更新 + 描画」を内包し、上位 `App` は画面遷移とコンポーネントの組版に集中。
- Builder Lite パターンを徹底（メソッドチェーンの戻り値を必ず受ける）。

## このリポジトリへの反映
- 現在の構成: `src/cli/main.rs`, `src/lib/{app.rs, ui.rs}` は既にガイドに準拠。
- 今後: `src/lib/{event.rs, tui.rs, update.rs}` と `components/` を必要に応じて追加。`tests/` には描画バッファ比較を段階的に導入。

## 参考
- Ratatui Book: 複数ファイル構成（`app.rs`, `event.rs`, `ui.rs`, `tui.rs`, `update.rs`）と役割分担。 
- Component Architecture: `init/handle_*_events/update/render` を持つコンポーネント分割。 
- Flux Architecture: 一方向データフロー（Dispatcher/Action/Store）の適用例。 
- Templates: シンプル/イベント駆動/コンポーネント各テンプレートの存在と命名慣習。 
- Builder Lite Pattern: ウィジェット構築のメソッドチェーン（戻り値の未使用に注意）。 
- テスト参考: `TestBackend`/`Buffer::with_lines` での描画アサートの慣習。

