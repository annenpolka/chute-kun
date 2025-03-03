# Chute_kun

TaskChuteメソッドを実装したTodoistタスク管理CLIツール

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
# 今日のタスクを表示
npm start

# 環境変数で設定せずにAPIトークンを指定して実行する場合
TODOIST_API_TOKEN=your_token_here npm start
```

## ライセンス

MIT

## 関連ドキュメント

- [TaskChute methodology](TaskChute_methodology.md)
- [システム概要](docs/system-overview.md)
- [データモデル](docs/data/models.md)