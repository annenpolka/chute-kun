# Chute_kun

TaskChuteメソッドを実装したTodoistタスク管理CLIツール

![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

TaskChuteの考え方をTodoist APIと連携し、コマンドラインから効率的なタスク管理を実現します。

## 特徴

- **シンプルなCLIインターフェース** - コマンドラインからTodoistタスクを管理
- **タスクフィルタリング** - 様々な条件でタスクを絞り込み
- **優先度管理** - TaskChuteの優先度付け手法の適用（開発中）
- **時間管理** - タスクの所要時間計測と予測（開発中）
- **タイムブロック生成** - 最適なスケジュール提案（開発中）

## 開発環境のセットアップ

### 前提条件

- Node.js (v18以上)
- npm または yarn
- TodoistアカウントとそのAPIトークン

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

- **Todoist API連携**: APIを使用してTodoistタスクを取得・管理
- **タスクフィルタリング**: 複数の条件でタスク絞り込み
  - プロジェクトID
  - ラベル（複数指定可能）
  - 期限日
  - 完了状態
  - 優先度（1-4）
- **今日のタスク表示**: 今日期限のタスクを簡単に確認
- **シンプルなCLI**: 分かりやすいコマンドインターフェース
- **柔軟な認証**: 環境変数またはコマンドラインからのAPIトークン指定

## ライセンス

MIT

## 関連ドキュメント

- [TaskChute methodology](TaskChute_methodology.md)
- [システム概要](docs/system-overview.md)
- [データモデル](docs/data/models.md)
- [実装状況](docs/IMPLEMENTATION.md)
- [開発計画](docs/DEVELOPMENT_PLAN.md)