# Todoist クライアントAPI

*最終更新日: 2025-03-03*

## 概要

Todoistクライアントは、TodoistのREST APIとの通信を担当するモジュールです。このモジュールはTodoistのタスク管理機能へのアクセスを抽象化し、TaskChuteアプリケーションとTodoist APIの間のデータ変換を行います。

## 関数一覧

### createTodoistApi

```typescript
function createTodoistApi(apiToken?: string): TodoistApi
```

**説明**:  
Todoist APIクライアントのインスタンスを作成します。APIトークンは引数として渡すか、環境変数から読み込みます。

**パラメータ**:
- `apiToken`: (オプション) Todoist APIトークン。指定しない場合は環境変数 `TODOIST_API_TOKEN` から読み込みます。

**戻り値**:
- `TodoistApi`: Todoist APIクライアントのインスタンス

**例外**:
- APIトークンが設定されていない場合にエラーをスローします。

**使用例**:
```typescript
// 環境変数から読み込む場合
const api = createTodoistApi();

// 直接トークンを指定する場合
const api = createTodoistApi('your_api_token_here');
```

### getTasks

```typescript
async function getTasks(api: TodoistApi | any, filter?: TaskFilter): Promise<any[]>
```

**説明**:  
指定されたフィルター条件に基づいてTodoistタスクを取得します。

**パラメータ**:
- `api`: Todoist APIクライアントのインスタンス
- `filter`: (オプション) タスクフィルター条件

**TaskFilter インターフェース**:
```typescript
interface TaskFilter {
  projectId?: string;
  labelIds?: string[];
  dueDate?: string;
  isCompleted?: boolean;
  priority?: number;
}
```

**戻り値**:
- `Promise<any[]>`: タスクオブジェクトの配列

**例外**:
- API通信エラー時に例外をスローします。

**使用例**:
```typescript
// すべてのタスクを取得
const allTasks = await getTasks(api);

// フィルター条件付きでタスクを取得
const filteredTasks = await getTasks(api, {
  projectId: 'project123',
  isCompleted: false,
  priority: 4
});
```

### getTodayTasks

```typescript
async function getTodayTasks(api: TodoistApi | any): Promise<any[]>
```

**説明**:  
今日期限の未完了タスクを取得します。

**パラメータ**:
- `api`: Todoist APIクライアントのインスタンス

**戻り値**:
- `Promise<any[]>`: 今日期限の未完了タスクオブジェクトの配列

**実装詳細**:
- 可能な場合は `filter: 'today'` パラメータを使用してAPIから直接フィルタリング
- それ以外の場合は当日の日付で `getTasks()` を使用

**使用例**:
```typescript
const todaysTasks = await getTodayTasks(api);
console.log(`今日のタスク数: ${todaysTasks.length}`);
```

## API互換性に関する注意事項

### Todoist APIバージョン対応

- このモジュールは Todoist API v9 (REST API) 向けに設計されています
- `@doist/todoist-api-typescript` バージョン `4.0.0-alpha.3` を使用
- 最新のTodoist APIレスポンス形式（`{ results: [] }` 形式）に対応しています

### レスポンスの正規化

Todoist APIの応答形式変更に対応するため、以下の正規化を行っています：

```typescript
// APIの戻り値を正規化
const tasks = Array.isArray(response) ? response : (response.results || []);
```

この処理により、APIの応答形式に関わらず一貫した形式でタスク配列を取得できます。