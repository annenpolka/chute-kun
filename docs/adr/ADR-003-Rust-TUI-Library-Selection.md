# ADR-003: Rust製TUIライブラリ選定

## ステータス

承認済み

_最終更新日: 2025-08-29_
_作成者/更新者: chute_kun team_

## コンテキスト

本リポジトリでは、TaskChute + Todoist を操作するCLI/TUIツールをRustで実装する計画がある。
長期運用を見据え、以下を満たすTUIスタックを選定する必要がある。

- クロスプラットフォーム（macOS/Linux/Windows）での安定動作
- メンテナンスが活発で将来性があること
- 日本語・絵文字を含むUnicode描画、リッチなウィジェット、マウス/キーボード入力
- 学習コストと拡張性のバランス（必要十分な抽象度、過度に重くないこと）

## 検討された選択肢

### 選択肢 1: ratatui + crossterm

- 概要: `ratatui` は `tui-rs` のコミュニティ主導フォークで活発に開発されているTUIフレームワーク。端末制御は`crossterm`を用いる構成がデファクト。
- 利点:
  - 活発な開発と豊富なウィジェット（レイアウト、テーブル、リスト、チャート等）。
  - `crossterm` により Windows を含むクロスプラットフォーム対応、マウス・修飾キー入力、カラー/スタイルに強い。
  - 学習資料・テンプレートが充実（`ratatui-org/templates`）。
- 欠点:
  - 低レベルI/Oや端末仕様差異の吸収はアプリ側の設計次第（イベント駆動ループの設計が必要）。

### 選択肢 2: Cursive（backend: crossterm/termion）

- 概要: 画面遷移やフォーカス管理などを含む高水準TUIフレームワーク。
- 利点:
  - 既成のビュー/ダイアログが豊富でフォーム実装が容易。
  - 複数バックエンド対応（標準は `crossterm`）。
- 欠点:
  - ライブラリの思想（ビュー階層/コールバック中心）に合わせる必要があり、カスタム描画での柔軟性は `ratatui` 単体より低いことがある。

### 選択肢 3: 低レベル構成（termion/termwiz/pancurses など）

- 概要: 端末制御レイヤ（`termion`, `termwiz`, `ncurses`系）を直接利用し、ウィジェットは自作。
- 利点:
  - 依存を最小化し、要件に特化した実装が可能。
- 欠点:
  - クロスプラットフォーム対応やウィジェット作成のコストが高い。保守性・開発速度に不利。

### 参考（非推奨）: tui-rs（オリジナル）

- 概要: `fdehau/tui-rs` は歴史的に人気だったが、現在はアーカイブされメンテ対象外。
- コメント: 現在は `ratatui` への移行が推奨される。

## 決定

選択された選択肢: ratatui + crossterm

理由:
- メンテナンスの活発さとコミュニティ規模、テンプレートやエコシステムの充実。
- 端末依存差異の吸収（Windows含む）と入力機能を `crossterm` が提供。
- 必要十分な抽象度で、アプリ都合の描画/状態管理を柔軟に設計できる。

## 影響

### 肯定的な影響
- クロスプラットフォーム対応とUnicode表示の改善。
- 実装スピードの向上（既製ウィジェット/テンプレート活用）。

### 否定的な影響
- イベントループ/状態管理はアプリ側で設計が必要。
- `ratatui` のメジャー更新時に追従コストが発生。

### 中立/トレードオフ
- CursiveはフォームUIに優れるが、今回の柔軟なダッシュボード/パネル型UIには `ratatui` が適合。

## 実装詳細（ガイドライン）

- 端末制御: `crossterm`（raw mode, alternate screen, mouse capture）。
- ランタイム: 同期/ポーリングでもよいが、バックグラウンド処理が増える場合は `tokio` + `mpsc`/`watch`でイベント駆動。
- 入力補助: テキスト入力には `tui-textarea` または `tui-input` を用途に応じて選定。
- ロギング/診断: `tracing` + `tracing-subscriber`、エラーは `color-eyre`。
- スキャフォールド: `cargo generate ratatui-org/templates` で開始し、`src/cli/` と `src/lib/` へ分割。

## 代替案と将来の可能性

- Cursive への置換（大量のフォーム/ダイアログ中心の要件に変化した場合）。
- 端末機能（グラフィック/ハイパーリンク/画像）が重要になれば `termwiz` の検討余地。
- OS限定配布でよければ `termion`（Unix系に限定）も軽量選択肢。

## 関連する決定

- ADR-001: Todoist API の選択
- ADR-002: テスト駆動開発(TDD)アプローチの採用

## 参考資料

- ratatui（公式ドキュメント/テンプレート）: https://ratatui.rs , https://github.com/ratatui-org/templates
- crossterm（公式）: https://crates.io/crates/crossterm
- Cursive（公式）: https://github.com/gyscos/cursive
- termion（公式）: https://lib.rs/crates/termion
- termwiz（公式）: https://github.com/wez/wezterm/tree/main/termwiz
- tui-rs（旧/アーカイブ）: https://github.com/fdehau/tui-rs

