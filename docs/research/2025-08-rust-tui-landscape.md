# Rust TUI ライブラリ技術調査（2025-08）

調査日: 2025-08-29
対象: 端末上で動作するText-based UI（TUI）をRustで構築するための主要ライブラリ群。

## 要約（結論）
- 現時点の第一候補は `ratatui`（UI） + `crossterm`（端末制御）。
- 画面遷移/フォーム主導のアプリには `Cursive` も有力代替。より低レベル制御や特殊端末機能が重要なら `termwiz` 検討余地。
- 旧 `tui-rs` はアーカイブ済みのため新規採用は推奨しない。

関連ADR: docs/adr/ADR-003-Rust-TUI-Library-Selection.md

## 評価基準
- 保守状況: メンテの活発さ、最新Rustとの互換性、バグ対応速度
- クロスプラットフォーム: macOS/Linux/Windows
- 入力/描画機能: キー/マウス、色/スタイル、Unicode/幅、レイアウト/ウィジェット
- 学習コスト: ドキュメント、サンプル、テンプレートの有無
- 拡張性: カスタムウィジェット、非同期処理/並行タスクとの統合

## 候補と所見

### ratatui（UIフレームワーク）
- 概要: `tui-rs` のコミュニティフォークとして活発に開発。レイアウト、表、リスト、チャートなどのウィジェットを提供。
- 長所: ドキュメント/テンプレートが充実。カスタム描画がしやすく、設計の自由度が高い。
- 短所: イベントループ/状態管理はアプリ側で設計が必要。
- 参考: 公式サイト/ドキュメント https://ratatui.rs , テンプレート https://github.com/ratatui-org/templates

### crossterm（端末制御）
- 概要: クロスプラットフォームな端末制御。raw mode、alternate screen、カラー、イベント（キー/マウス）等を提供。
- 長所: Windowsを含む幅広い互換性、安定したAPI、コミュニティ利用が多い。
- 短所: 端末依存の差異は一部アプリ側で吸収が必要。
- 参考: crates.io https://crates.io/crates/crossterm , ドキュメント https://docs.rs/crossterm

### Cursive（高水準フレームワーク）
- 概要: ビュー/ダイアログ/フォーカス/イベント伝播など高水準機能を提供。バックエンドに `crossterm` 等を利用。
- 長所: フォームやダイアログ中心のアプリを素早く構築可能。
- 短所: 描画の自由度は `ratatui` 直書きに劣る場合がある。
- 参考: https://github.com/gyscos/cursive

### termion（端末制御/Unix系）
- 概要: 依存の少ないUnix系端末制御クレート（macOS/Linux/BSD）。
- 長所: シンプルで軽量。外部C依存なし。
- 短所: Windowsをサポートしない。
- 参考: https://lib.rs/crates/termion

### termwiz（高度端末機能）
- 概要: WezTerm 由来の端末抽象化。ハイパーリンク、画像等の高度機能に対応（端末実装に依存）。
- 長所: 端末機能を広くカバー。描画/セルグリッドモデルが強力。
- 短所: 学習コスト。一般的なTUIにはやや過剰な場合がある。
- 参考: https://github.com/wez/wezterm/tree/main/termwiz

### ncurses/pancurses（Cライブラリバインディング）
- 概要: 歴史の長い `ncurses` をRustから利用。システムライブラリ依存。
- 長所: 機能は枯れて安定。CUIの基本機能が一通り揃う。
- 短所: ビルド環境依存、Windows対応は制限あり、抽象度は低め。
- 参考: https://crates.io/crates/pancurses , https://crates.io/crates/ncurses

### tui-rs（参考：旧/アーカイブ）
- 概要: 歴史的TUIフレームワーク。現在はアーカイブされメンテ対象外。
- 参考: https://github.com/fdehau/tui-rs

## 補助クレート（ratatuiエコシステム）
- 入力: `tui-textarea`（複数行テキスト入力） https://github.com/rhysd/tui-textarea
- 入力（軽量）: `tui-input` https://github.com/sayanarijit/tui-input
- 表/装飾: `comfy-table`（テーブル描画補助） https://github.com/Nukesor/comfy-table
- ログUI: `tui-logger` https://github.com/gin66/tui-logger

## 推奨スタックとレシピ
- UI: `ratatui`
- 端末制御: `crossterm`
- ランタイム/並行処理: 必要に応じ `tokio`、イベントは `mpsc`/`watch` で分配
- ログ/診断: `tracing` + `tracing-subscriber`、`color-eyre`
- スキャフォールド: `cargo generate ratatui-org/templates`

### 最小サンプル（概念スケッチ）
```rust
use std::io::{stdout};
use crossterm::{terminal, execute, event::{self, Event, KeyCode}, cursor};
use ratatui::{prelude::*, widgets::{Block, Borders}};

fn main() -> anyhow::Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Chute_kun").borders(Borders::ALL);
            f.render_widget(block, size);
        })?;
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(k) = event::read()? {
                if k.code == KeyCode::Char('q') { break; }
            }
        }
    }

    // teardown
    let mut out = terminal.into_inner();
    execute!(out, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
```

## 採用時の注意点
- 端末差異（Windowsのキー修飾、IME挙動等）は早期に検証。マウスイベントも `crossterm` の仕様に沿って扱う。
- レンダリング負荷が高い画面（大量リスト等）は差分描画やスクロール最適化を検討。
- テスト: レイアウト/状態遷移はユニットテスト、描画はバッファ比較（`ratatui`の`Buffer`）でカバー可能。

## 出典リンク
- ratatui: https://ratatui.rs , https://docs.rs/ratatui , https://github.com/ratatui-org/ratatui
- crossterm: https://crates.io/crates/crossterm , https://docs.rs/crossterm
- Cursive: https://github.com/gyscos/cursive
- termion: https://lib.rs/crates/termion
- termwiz: https://github.com/wez/wezterm/tree/main/termwiz
- ncurses/pancurses: https://crates.io/crates/ncurses , https://crates.io/crates/pancurses
- tui-textarea: https://github.com/rhysd/tui-textarea , tui-input: https://github.com/sayanarijit/tui-input
