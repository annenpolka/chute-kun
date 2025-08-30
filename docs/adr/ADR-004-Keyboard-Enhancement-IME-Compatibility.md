# ADR-004: キーボード拡張フラグと IME（日本語入力）互換性方針

## ステータス
承認済

## 日付
2025-08-30

## コンテキスト
TUI では `crossterm` の Progressive Keyboard Enhancement を用いると、修飾キー（例: `Shift+Enter`）の識別精度が向上する。一方で、`REPORT_ALL_KEYS_AS_ESCAPE_CODES`（いわゆる CSI-u モード）を有効にすると、現行の crossterm 実装（0.27 系）では Unicode/IME 入力の扱いに制約があり、日本語入力が崩れる（文字として届かない／エスケープ列になる）事象がある。

本プロジェクトは日本語入力（IME）の円滑な利用を最優先としつつ、実用的な範囲で修飾キーの識別を維持したい。

## 決定
`REPORT_ALL_KEYS_AS_ESCAPE_CODES` は有効化しない。代わりに以下のフラグのみを使用する。

- `DISAMBIGUATE_ESCAPE_CODES`
- `REPORT_EVENT_TYPES`

さらに、ターミナルからの `Paste` イベントを取り込み、入力モード時は貼り付け文字列をそのままバッファに追記する（日本語含む）。

該当実装（crossterm 0.29 系）:

- `src/cli/main.rs` のキーボード拡張フラグ設定を上記 2 つに限定
- `src/cli/main.rs` に `Event::Paste` ハンドリングを追加
- `src/lib/app.rs` に `App::handle_paste(&str)` を追加
- `src/lib/app.rs` の `handle_key_event` では `KeyEventKind::Release` を無視（Press/Repeat のみ処理）し、
  一部ターミナル（iTerm2/Ghostty 等）が Press と Release の両方を報告する際の二重入力を防止する

## 根拠
- IME 入力（日本語）は CLI/TUI の主要ユースケースであり、入力の信頼性を最優先する
- `REPORT_ALL_KEYS_AS_ESCAPE_CODES` を使わずとも多くの環境で `Shift` などの修飾情報は十分に扱える
- `Paste` をサポートすることで、日本語テキストの投入経路をもう一つ確保できる

## 代替案（検討したが採用しない）
1. すべてのフラグを最大限有効化（CSI-u 完全有効）
   - 欠点: 現行 crossterm では Unicode/IME 互換性に難があるため日本語入力が壊れる可能性
2. 設定で切替（IME優先/修飾キー優先）
   - 将来の選択肢として有効。現時点では複雑性を避け、まず既定を IME 互換性優先とする

## 影響
- 良い影響: 日本語入力（変換/確定）を含む Unicode 入力が安定して受け取れる
- 注意点: 一部ターミナルでは `Shift+Enter` が常に届くとは限らない。既定の代替キー `f` で Finish 可能
- ユーザー向けガイド: `docs/troubleshooting/ime-and-japanese-input.md` を参照

## 参照
- Crossterm Keyboard Enhancement（KeyboardEnhancementFlags, Event）
  - https://docs.rs/crossterm/latest/crossterm/event/struct.KeyboardEnhancementFlags.html
  - https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html#variant.Paste
- Kitty/CSI-u に関する背景資料（参考）
  - https://sw.kovidgoyal.net/kitty/keyboard-protocol/
