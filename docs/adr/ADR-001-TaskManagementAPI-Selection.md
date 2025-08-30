# ADR-001: Todoist API の選択

## ステータス
提案（未採用・未実装）

## 日付
2025-03-01

## コンテキスト
タスクシュート手法を実装するためのバックエンドとして、既存のタスク管理APIを利用することでシンプルな構造と素早い開発を実現したい。選択肢としてTodoist、Microsoft To Do、Asana、その他のタスク管理APIが考えられる。

## 提案内容
Todoist REST API v2 を主要なタスク管理バックエンドとして採用する案。

## 根拠
1. Todoistはプロジェクト、セクション、タスク、コメント、ラベルなどタスクシュートの概念と親和性の高い構造を持つ
2. REST API v2は最新かつドキュメント化が充実しており、開発効率が高い
3. 認証が簡素（API Tokenのみ）で実装が容易
4. 複数のプログラミング言語（JavaScript、Python等）の公式クライアントライブラリが存在
5. API使用制限が他のサービスと比較して寛容（開発中のテストにも適している）

## 影響（採用時）
1. ユーザーは Todoist アカウントと API Token の管理が必要
2. Todoist の構造に合わせた概念マッピングが必要
3. 将来の他バックエンド追加には抽象化レイヤーが必要

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

## 現状
2025-08-30 時点ではローカルの TOML スナップショットのみ（外部 API 未実装）。進捗は `docs/setup/implementation-status.md` を参照。

## 関連
- Todoist Developer Documentation: https://developer.todoist.com/
- Todoist REST API v2: https://developer.todoist.com/rest/v2/
