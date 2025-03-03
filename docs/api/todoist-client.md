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
async function getTasks(api: TodoistApi | any, filter?: TaskFilter): Promise<TodoistTask[]>
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
- `Promise<TodoistTask[]>`: タスクオブジェクトの配列

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
async function getTodayTasks(api: TodoistApi | any): Promise<TodoistTask[]>
```

**説明**:
今日期限の未完了タスクを取得します。

**パラメータ**:
- `api`: Todoist APIクライアントのインスタンス

**戻り値**:
- `Promise<TodoistTask[]>`: 今日期限の未完了タスクオブジェクトの配列

**実装詳細**:
- 可能な場合は `filter: 'today'` パラメータを使用してAPIから直接フィルタリング
- それ以外の場合は当日の日付で `getTasks()` を使用

**使用例**:
```typescript
const todaysTasks = await getTodayTasks(api);
console.log(`今日のタスク数: ${todaysTasks.length}`);
```

### buildTaskHierarchy

```typescript
function buildTaskHierarchy(tasks: TodoistTask[]): HierarchicalTask[]
```

**説明**:
フラットなタスク配列を階層構造に変換します。親子関係は`parentId`または`parent_id`プロパティに基づいて構築されます。

**パラメータ**:
- `tasks`: フラットなタスク配列

**戻り値**:
- `HierarchicalTask[]`: 階層構造化されたタスク配列

**HierarchicalTask インターフェース**:
```typescript
interface HierarchicalTask {
  id: string;
  content: string;
  projectId: string;
  parentId?: string | null;
  labels?: string[];
  priority?: number;
  isCompleted: boolean;
  due?: { date: string };
  // 階層構造のための追加フィールド
  subTasks: HierarchicalTask[];
  isSubTask: boolean;
  level: number;
  [key: string]: any; // その他のプロパティを許容
}
```

**使用例**:
```typescript
const allTasks = await getTasks(api);
const hierarchicalTasks = buildTaskHierarchy(allTasks);
```

### flattenTaskHierarchy

```typescript
function flattenTaskHierarchy(tasks: HierarchicalTask[]): HierarchicalTask[]
```

**説明**:
階層構造をフラットな配列に変換します（表示用）。階層情報（level、isSubTask）は保持されます。

**パラメータ**:
- `tasks`: 階層構造化されたタスク配列

**戻り値**:
- `HierarchicalTask[]`: フラット化されたタスク配列（階層情報付き）

**使用例**:
```typescript
const hierarchicalTasks = await getTasksWithSubtasks(api);
const flattenedTasks = flattenTaskHierarchy(hierarchicalTasks);
```

### getTasksWithSubtasks

```typescript
async function getTasksWithSubtasks(
  api: TodoistApi | any,
  filter?: TaskFilter
): Promise<HierarchicalTask[]>
```

**説明**:
サブタスク情報を含むタスクを取得し、階層構造化して返します。

**パラメータ**:
- `api`: Todoist APIクライアントのインスタンス
- `filter`: (オプション) タスクフィルター条件

**戻り値**:
- `Promise<HierarchicalTask[]>`: 階層構造化されたタスクの配列

**使用例**:
```typescript
const tasksWithSubtasks = await getTasksWithSubtasks(api, { projectId: 'project123' });
```

### getTodayTasksWithSubtasks

```typescript
async function getTodayTasksWithSubtasks(
  api: TodoistApi | any
): Promise<HierarchicalTask[]>
```

**説明**:
今日期限のタスクとそのサブタスク（期限問わず）を取得し、階層構造化して返します。

**パラメータ**:
- `api`: Todoist APIクライアントのインスタンス

**戻り値**:
- `Promise<HierarchicalTask[]>`: 階層構造化された今日期限のタスクとそのサブタスクの配列

**使用例**:
```typescript
const todayTasksWithSubtasks = await getTodayTasksWithSubtasks(api);
```

## API互換性に関する注意事項

### Todoist APIバージョン対応

- このモジュールは Todoist API v9 (REST API) 向けに設計されています
- `@doist/todoist-api-typescript` バージョン `4.0.0-alpha.3` を使用
- 最新のTodoist APIレスポンス形式（`{ results: [] }` 形式）に対応しています
- **parentId/parent_id両方の形式に対応**しています

### レスポンスの正規化

Todoist APIの応答形式変更に対応するため、以下の正規化を行っています：

```typescript
// APIの戻り値を正規化（配列、resultsプロパティ、itemsプロパティのいずれかに対応）
const tasks = Array.isArray(response) ? response : (response.results || response.items || []);
```

この処理により、APIの応答形式に関わらず一貫した形式でタスク配列を取得できます。

### 親子関係の検出

Todoist APIでは、サブタスク関係を表現するために以下の複数の形式が使用される可能性があります：

- `parentId`: TypeScriptクライアントの形式
- `parent_id`: API応答の形式
- `parent`: 別の可能性のある形式

これらの形式すべてに対応するため、以下のような検出ロジックを実装しています：

```typescript
// parentIdとparent_idの両方に対応
const parentId = task.parentId || task.parent_id;
if (parentId && taskMap.has(parentId)) {
  // 親子関係の処理
}