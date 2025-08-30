# ADR-002: テスト駆動開発(TDD)アプローチの採用

## ステータス
承認済

## 日付
2025-03-01

## コンテキスト
TaskChuteの実装にあたり、コード品質の確保、要件の明確化、および長期的なメンテナンス性を考慮した開発アプローチを決定する必要がある。選択肢として、事前設計重視のアプローチ、アジャイル/スクラム、テスト駆動開発(TDD)などがある。

## 決定
ケント・ベックのテスト駆動開発（TDD）プラクティスを採用し、全ての機能実装はテストファーストで進める。

## 根拠
1. テストファーストのアプローチにより、要件が明確になり、設計がシンプルになる
2. Red-Green-Refactorサイクルにより、常に動作するコードを維持しながら改善できる
3. 自動テストの充実により、リファクタリングや新機能追加時の安全性が高まる
4. オーバーエンジニアリングを避け、必要十分なコードのみを実装できる
5. ドキュメントとコードの乖離を防ぎ、テストがライブドキュメントとしても機能する

## 影響
1. 肯定的影響:
   - 高品質なコードベースの構築
   - 設計の簡素化と明確化
   - 変更に対する柔軟性の向上
   - バグの早期発見
   - 継続的なリファクタリングの促進

2. 否定的影響:
   - 開発初期段階でのわずかな速度低下の可能性
   - チームメンバーがTDDに慣れるまでの学習コスト
   - テスト実装に関する追加作業

## 実装方針（本リポジトリ）
1. Kent Beck の「Test-Driven Development: By Example」のプラクティスを参考にする
2. 3 ステップの基本サイクルを徹底する:
   - Red: 失敗するテストを書く
   - Green: テストが通るように最小のコードを書く
   - Refactor: 設計を改善する（テストは常に Green）
3. Rust の標準ツールチェーンで実行する
   - 単体/統合テスト: `cargo test`
   - UI 描画: `ratatui::backend::TestBackend` と `Buffer` 比較で検証
   - 時刻依存: `clock::Clock` を注入し、決定的な値で検証
4. 外部依存（将来の Todoist 等）は薄いラッパで抽象化し、モックでテストする
5. 変更は「テスト + 実装 + ドキュメント」を同一コミットで更新する

## 関連
- Kent Beck「Test-Driven Development: By Example」
- Martin Fowler「Refactoring: Improving the Design of Existing Code」
- docs/processes/development-workflow.md
- docs/processes/tui-architecture-and-naming.md
- docs/adr/ADR-003-Rust-TUI-Library-Selection.md
