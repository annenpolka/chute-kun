# ADR-001: Todoist API の選択

## ステータス
提案

## 日付
2025-03-01

## コンテキスト
タスクシュート手法を実装するためのバックエンドとして、既存のタスク管理APIを利用することでシンプルな構造と素早い開発を実現したい。選択肢としてTodoist、Microsoft To Do、Asana、その他のタスク管理APIが考えられる。

## 決定
Todoist REST API v2を主要なタスク管理バックエンドとして採用する。

## 根拠
1. Todoistはプロジェクト、セクション、タスク、コメント、ラベルなどタスクシュートの概念と親和性の高い構造を持つ
2. REST API v2は最新かつドキュメント化が充実しており、開発効率が高い
3. 認証が簡素（API Tokenのみ）で実装が容易
4. 複数のプログラミング言語（JavaScript、Python等）の公式クライアントライブラリが存在
5. API使用制限が他のサービスと比較して寛容（開発中のテストにも適している）

## 影響
1. ユーザーはTodoistアカウントが必要となり、API Tokenの管理が発生する
2. Todoistの構造に合わせてタスクシュートの概念をマッピングする必要がある
3. 将来的にTodoist以外のバックエンドを追加する場合は、抽象化レイヤーの導入が必要となる

## Todoist API 基本情報

### エンドポイント
- ベースURL: `https://api.todoist.com/rest/v2/`
- 認証: Bearer Token（ヘッダーに `Authorization: Bearer {api_token}` を設定）

### 主要リソース
1. **プロジェクト (Projects)** 
   - タスクシュートの「プロジェクト」または「セクション」に相当
   - エンドポイント: `/projects`

2. **セクション (Sections)**
   - タスクシュートの「モード」や細分化された「セクション」に相当
   - エンドポイント: `/sections`

3. **タスク (Tasks)**
   - タスクシュートの中核機能「タスク」に相当
   - エンドポイント: `/tasks`
   - due_date, priority, labelなどで拡張可能

4. **ラベル (Labels)**
   - タスクシュートの「カテゴリ」や「フラグ」に相当
   - エンドポイント: `/labels`

5. **コメント (Comments)**
   - タスクシュートの「ログ」記録に活用可能
   - エンドポイント: `/comments`

## 関連
- [Todoist Developer Documentation](https://developer.todoist.com/)
- [Todoist REST API v2](https://developer.todoist.com/rest/v2/)