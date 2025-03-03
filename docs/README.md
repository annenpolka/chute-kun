# TaskChute プロジェクトドキュメント

*最終更新日: 2025-03-03*

## ドキュメント構成

TaskChuteプロジェクトのドキュメントは以下の構成で管理されています。各ドキュメントは特定の目的と対象読者を持ち、相互に参照し合う形で全体の理解を深められるよう設計されています。

### 1. 計画・設計

- [**開発計画**](./planning/development-plan.md) - プロジェクトの開発計画と段階的な実装ステップ
- [**CLINE設定計画**](./planning/cline-config.md) - CLINEツール用のカスタムモード定義
- [**TaskChute方法論**](../TaskChute_methodology.md) - タスクシュート手法の基本概念と原則

### 2. システム基本構造

- [**システム概要**](./system-overview.md) - システム全体の目的、構成、データフロー
- [**データモデル定義**](./data/models.md) - 主要データ構造とエンティティ関係

### 3. アーキテクチャ決定記録 (ADR)

主要な技術選択と設計判断を記録：

- [**ADR-001: Todoist API選定**](./adr/ADR-001-TaskManagementAPI-Selection.md)
- [**ADR-002: TDDアプローチ採用**](./adr/ADR-002-TDD-Development-Approach.md)
- [**ADR-003: サーバーレス設計**](./adr/ADR-003-Serverless-Architecture.md)
- [**ADR-004: 技術スタック方針**](./adr/ADR-004-Technology-Stack-Direction.md)

### 4. セットアップと状況

- [**実装状況**](./setup/implementation-status.md) - 現在の実装状況と次のステップ
- [**環境変数管理**](./setup/environment-variables.md) - 環境変数の設定と管理方法

### 5. モジュールと機能仕様

- [**タスクマネージャー**](./modules/task-manager.md) - タスク管理コアモジュール
- [**タイムブロック生成機能**](./features/timeblock-generation.md) - タイムブロック生成機能の詳細仕様

### 6. 開発プロセス

- [**開発ワークフロー**](./processes/development-workflow.md) - ドキュメント駆動開発とコミットフック
- [**コミット戦略**](./processes/commit-strategy.md) - コミット粒度とメッセージ規約
- [**決定ログ**](./processes/decisions-log.md) - 日々の小規模な決定の記録

### 7. 調査・研究

- [**技術評価**](./decisions/technology-evaluation.md) - 技術選定評価の詳細
- [**CLIツール比較**](./research/cli-tools-comparison.md) - CLIツール開発技術の比較調査

## ドキュメント管理原則

1. **ドキュメント先行** - 実装前にドキュメントを更新
2. **一貫性** - すべてのドキュメントは共通のフォーマットと用語を使用
3. **最新性** - コード変更に伴いドキュメントも同時更新
4. **トレーサビリティ** - 要件から実装までの追跡が可能
5. **自己完結性** - 各ドキュメントは必要な文脈を含み単独でも理解可能
6. **言語** - 基本的に日本語で記述し、必要に応じて英語を併記

## 更新履歴

- 2025-03-03: ドキュメント構成を整理、新しいカテゴリを追加
- 2025-03-01: 初版作成、基本構成の定義