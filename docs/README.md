# TaskChute プロジェクトドキュメント

_最終更新日: 2025-03-03_

## ドキュメント構造

TaskChute プロジェクトのドキュメントは以下の論理的な階層構造で整理されています。各ドキュメントは特定の目的を持ち、相互参照によって全体の理解を深められるよう設計しています。

```
docs/
├── README.md                # ドキュメント全体の目次と原則
├── system-overview.md       # システム全体の概要設計
├── adr/                     # アーキテクチャ決定記録
├── api/                     # API仕様書
├── data/                    # データモデル定義
├── decisions/               # 意思決定記録
├── features/                # 機能仕様書
├── modules/                 # モジュール設計書
├── planning/                # 計画・戦略文書
├── processes/               # 開発プロセス定義
├── research/                # 調査・分析結果
├── setup/                   # 環境構築・設定ガイド
└── templates/               # ドキュメントテンプレート
```

## ドキュメントカテゴリ

### 1. 設計・アーキテクチャ

- [**システム概要**](./system-overview.md) - システム全体の目的、構成、データフロー
- [**データモデル定義**](./data/models.md) - 主要データ構造とエンティティ関係
- アーキテクチャ決定記録 (ADR)
  - [**ADR-001: Todoist API 選定**](./adr/ADR-001-TaskManagementAPI-Selection.md)
  - [**ADR-002: TDD アプローチ採用**](./adr/ADR-002-TDD-Development-Approach.md)
  - [**ADR-003: サーバーレス設計**](./adr/ADR-003-Serverless-Architecture.md)
  - [**ADR-004: 技術スタック方針**](./adr/ADR-004-Technology-Stack-Direction.md)
  - [**ADR-005: 階層型タスク表示機能**](./adr/ADR-005-Hierarchical-Task-Display.md)
  - [**ADR-006: 統合テストのエラーハンドリング改善**](./adr/ADR-006-Integration-Test-Error-Handling.md)

### 2. 機能仕様

- [**タスクマネージャー**](./modules/task-manager.md) - タスク管理コアモジュール
- [**タイムブロック生成**](./features/timeblock-generation.md) - タイムブロック生成機能
- [**サブタスク構造**](./features/subtask-structure.md) - 階層的なタスク構造の実装
- [**タスクフォーマッター**](./features/task-formatter.md) - タスク表示形式の定義と実装

### 3. 開発・運用

- [**開発計画**](./planning/development-plan.md) - プロジェクトの開発計画と実装ステップ
- [**実装状況**](./setup/implementation-status.md) - 現在の実装状況と次のステップ
- [**環境変数管理**](./setup/environment-variables.md) - 環境変数設定と管理方法
- [**Todoist API クライアント**](./api/todoist-client.md) - Todoist API 連携の基本仕様
- [**Todoist API 実装詳細**](./api/todoist-client-implementation.md) - API 連携実装の詳細
- [**Todoist API 改善計画**](./planning/todoist-api-improvement.md) - API 連携の将来的な改善案

### 4. プロセス・方法論

- [**TaskChute 方法論**](../TaskChute_methodology.md) - タスクシュート手法の基本概念
- [**開発ワークフロー**](./processes/development-workflow.md) - ドキュメント駆動開発
- [**コミット戦略**](./processes/commit-strategy.md) - コミット粒度とメッセージ規約
- [**決定ログ**](./processes/decisions-log.md) - 日々の小規模な決定の記録
- [**統合テスト実施ガイド**](./processes/integration-testing-guide.md) - Todoist API との統合テスト実施手順

### 5. 調査・研究

- [**技術評価**](./decisions/technology-evaluation.md) - 技術選定評価の詳細
- [**CLI ツール比較**](./research/cli-tools-comparison.md) - CLI ツール開発技術の比較
- [**CLINE 設定計画**](./planning/cline-config.md) - CLINE ツール用のカスタムモード
- [**Jest 統合テスト実装研究**](./research/jest-integration-testing.md) - Jest を用いた外部 API 統合テストの実装研究

### 6. テンプレート

- [**ADR テンプレート**](./templates/adr-template.md) - アーキテクチャ決定記録のテンプレート
- [**機能仕様テンプレート**](./templates/feature-template.md) - 機能仕様書のテンプレート
- [**モジュール設計テンプレート**](./templates/module-template.md) - モジュール設計書のテンプレート

## ドキュメント管理原則

### 基本原則

1. **ドキュメント先行開発** - 実装前にドキュメントを作成・更新する
2. **一貫性の維持** - 共通フォーマット、用語、構造を使用する
3. **最新状態の保証** - コード変更と同時にドキュメントも更新する
4. **トレーサビリティ** - 要件から実装までの追跡を可能にする
5. **自己完結性** - 各ドキュメントは単独でも理解可能なように文脈を含める
6. **多言語対応** - 基本は日本語、必要に応じて英語を併記する

### ドキュメント作成ガイドライン

1. **適切な場所** - 新規ドキュメントは上記の構造に従って適切なディレクトリに配置する
2. **命名規則** - ファイル名はケバブケース（例: `task-manager.md`）を使用する
3. **メタデータ** - 各ドキュメントには最終更新日と作成者/更新者を記載する
4. **相互参照** - 関連するドキュメントへのリンクを積極的に含める
5. **テンプレート** - 各カテゴリのテンプレートを使用して一貫性を保つ

## 更新履歴

- 2025-03-03: ドキュメント構造を整理、カテゴリの見直しと詳細なガイドラインを追加、新規 ADR とテスト関連ドキュメントを追加、実際のファイル構造と整合
- 2025-03-01: 初版作成、基本構成の定義
