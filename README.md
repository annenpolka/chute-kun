# Chute_kun

TaskChute メソッドを実装した Todoist タスク管理 CLI ツール

![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

TaskChute の考え方を Todoist API と連携し、コマンドラインから効率的なタスク管理を実現します。

## 特徴

- **シンプルな CLI インターフェース** - コマンドラインから Todoist タスクを管理
- **タスクフィルタリング** - 様々な条件でタスクを絞り込み
- **優先度管理** - TaskChute の優先度付け手法の適用（開発中）
- **時間管理** - タスクの所要時間計測と予測（開発中）
- **タイムブロック生成** - 最適なスケジュール提案（開発中）

## 開発環境のセットアップ

### 前提条件

- Node.js (v18 以上)
- npm または yarn
- Todoist アカウントとその API トークン

### インストール

```bash
# リポジトリのクローン
git clone https://github.com/annenpolka/chute_kun.git
cd chute_kun

# 依存パッケージのインストール
npm install

# 環境変数の設定
cp .env.example .env
# .envファイルを編集してTodoistのAPIトークンを設定
```

### 開発用コマンド

```bash
# 開発モードで実行
npm run dev

# CLIモードで実行
npm run cli

# テスト実行
npm test

# テストの監視モード（TDD用）
npm run test:watch

# ビルド
npm run build

# ビルド後の実行
npm start
```

## 使い方

```bash
# ヘルプを表示
npm run cli help

# 今日のタスクを表示
npm run cli today

# 条件を指定してタスクをフィルタリング
npm run cli filter --project プロジェクトID
npm run cli filter --due 2025-03-10
npm run cli filter --labels ラベル1,ラベル2 --completed false
npm run cli filter --priority 4  # 優先度4（最高）のタスクのみ表示

# APIトークンをコマンドラインから指定
npm run cli today --token your_todoist_api_token

# 環境変数で直接指定する別の方法
TODOIST_API_TOKEN=your_token_here npm run cli
```

### グローバルインストール (オプション)

ビルド後、グローバルにインストールすることも可能です:

```bash
# ビルド
npm run build

# グローバルインストール
npm install -g .

# コマンド実行
chute today
chute filter --project プロジェクトID
```

## 現在の機能

- **Todoist API 連携**: API を使用して Todoist タスクを取得・管理
- **タスクフィルタリング**: 複数の条件でタスク絞り込み
  - プロジェクト ID
  - ラベル（複数指定可能）
  - 期限日
  - 完了状態
  - 優先度（1-4）
- **今日のタスク表示**: 今日期限のタスクを簡単に確認
- **シンプルな CLI**: 分かりやすいコマンドインターフェース
- **柔軟な認証**: 環境変数またはコマンドラインからの API トークン指定

## ライセンス

MIT

## 関連ドキュメント

- [TaskChute methodology](TaskChute_methodology.md)
- [ドキュメント一覧](docs/README.md)
- [システム概要](docs/system-overview.md)
- [実装状況](docs/setup/implementation-status.md)
- [開発計画](docs/planning/development-plan.md)

## Cursor IDE 設定

このプロジェクトでは Cursor IDE の使用に最適化された`.cursorrules`設定を含んでいます。この設定はプロジェクトの開発原則を守り、一貫性のあるコードベースを維持するためのガイドラインとして機能します。

### 主要設定カテゴリ

1. **コア開発スタイル**

   - TypeScript コーディング規約（インデント、引用符、末尾カンマなど）
   - 関数型プログラミングの推奨
   - シンプルな設計原則
   - 型安全性の重視

2. **テスト戦略**

   - Red-Green-Refactor TDD ワークフロー
   - エッジケースを含む包括的テスト
   - 外部依存の適切なモック

3. **ドキュメント管理**

   - ドキュメント先行アプローチ（実装前にドキュメント）
   - 日本語と英語の両方でのコメント
   - JSDoc 形式の自動生成
   - **docs/フォルダでの一元管理**
   - **標準テンプレートの使用**
   - **メタデータ（更新日、作成者など）の必須化**

4. **開発ワークフロー**

   - 構造化されたコミットメッセージ形式
   - 原子的コミット（1 つの論理的変更）
   - ドキュメント更新の強制
   - アーキテクチャ決定記録（ADR）の管理

5. **TaskChute 固有機能**
   - 計画・記録・ルーティンの中核機能
   - Todoist API 連携の処理方針
   - CLI 出力のカスタマイズ
   - サーバーレスアーキテクチャ

### ドキュメント構造

プロジェクトのドキュメントは `docs/` ディレクトリで一元管理され、以下の構造に従って整理されています：

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
    ├── feature-template.md  # 機能仕様書テンプレート
    ├── module-template.md   # モジュール設計書テンプレート
    └── adr-template.md      # ADR記録テンプレート
```

すべての新規ドキュメントは適切なディレクトリに配置し、対応するテンプレートを使用して作成することが推奨されています。

### 開発者への影響

この設定は単なる好みの問題ではなく、TaskChute プロジェクトの設計思想と密接に関連しています：

- **安全性**: 型安全性とテストの重視
- **保守性**: 明確な構造と命名規則
- **一貫性**: 統一されたコードスタイルとドキュメント
- **効率性**: ドキュメント先行で設計ミスを早期に発見
- **追跡可能性**: 変更履歴と意思決定プロセスの明確化

設定の完全な詳細は `.cursorrules` ファイルを参照してください。この設定は Cursor IDE で自動的に適用され、プロジェクトの品質と一貫性を維持するのに役立ちます。
