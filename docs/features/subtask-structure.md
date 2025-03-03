# サブタスク構造化機能 / Subtask Structuring Feature

*最終更新日 / Last Updated: 2025-03-03*

## 機能概要 / Feature Overview

サブタスク構造化機能は、Todoistのサブタスク（親子関係）機能を活用し、階層構造を持つタスクデータを取得・管理するための機能です。この機能により、複雑なタスクを階層的に整理し、タイムブロック生成時にサブタスクを考慮した効率的なスケジューリングが可能になります。

This feature utilizes Todoist's subtask (parent-child relationship) functionality to retrieve and manage hierarchically structured task data. It enables organizing complex tasks hierarchically and efficient scheduling that considers subtasks during time block generation.

## TaskChute手法との関連性 / Relation to TaskChute Methodology

サブタスク構造化機能は、TaskChute手法の3つの中核機能と以下のように関連しています：

1. **計画 / Plan**
   - 複雑なタスクを階層的に分解することで、より精緻な計画が可能になります
   - 親タスクとサブタスクの関係性を考慮した時間配分により、効率的なタイムブロック生成を実現します

2. **記録 / Log**
   - サブタスクの完了状況を追跡することで、親タスクの進捗を正確に把握できます
   - 階層レベルごとの実績データを収集し、より詳細な振り返りと分析が可能になります

3. **ルーティン / Routine**
   - 繰り返しタスクにおいても階層構造を維持し、ルーティンの効率化を図ります
   - サブタスクパターンの認識により、類似タスクの効率的な実行を支援します

## ユースケース / Use Cases

1. **階層的なタスク管理 / Hierarchical Task Management**
   - ユーザーがTodoistで作成した親タスクとサブタスクの関係を保持したままタスクを取得
   - 多階層（複数レベルのネスト）のサブタスク構造に対応

2. **サブタスクを考慮したタイムブロック生成 / Time Block Generation with Subtasks**
   - 親タスクとサブタスクの関係性を考慮した時間配分
   - サブタスクの完了状況に基づく親タスクの進捗管理

3. **階層的なタスク表示 / Hierarchical Task Display**
   - インデントや階層を視覚的に表現したタスク一覧表示
   - 親タスクの展開・折りたたみ機能

## 入力データ / Input Data

1. **Todoistタスクデータ / Todoist Task Data**
   - 標準的なタスク情報（ID、タイトル、期限日など）
   - 親子関係情報（`parentId`属性）

2. **フィルター条件 / Filter Conditions**
   - プロジェクトID、ラベル、期限日などの標準的なフィルター
   - サブタスク表示に関する設定（すべて表示、親タスクのみ表示など）

## 出力データ / Output Data

1. **階層構造化されたタスク配列 / Hierarchically Structured Task Array**
   ```typescript
   // Todoistタスクの基本インターフェース / Basic Todoist Task Interface
   interface TodoistTask {
     id: string;
     content: string;
     projectId: string;
     parentId?: string | null;
     isCompleted: boolean;
     [key: string]: any; // その他のプロパティを許容 / Allow other properties
   }

   // 階層構造を持つタスクのインターフェース / Hierarchical Task Interface
   interface HierarchicalTask {
     id: string;
     content: string;
     projectId: string;
     parentId?: string | null;
     labels?: string[];
     priority?: number;
     isCompleted: boolean;
     due?: { date: string };
     // 階層構造のための追加フィールド / Additional fields for hierarchy
     subTasks: HierarchicalTask[];
     isSubTask: boolean;
     level: number;
     [key: string]: any; // その他のプロパティを許容 / Allow other properties
   }
   ```

2. **フラット化されたタスク配列（表示用） / Flattened Task Array (for display)**
   - 階層レベル情報を保持したままフラット化されたタスク配列
   - 表示順序は親タスクの後にサブタスクが続く形式

## アルゴリズム概要 / Algorithm Overview

1. **データ取得 / Data Retrieval**
   - Todoist APIからタスクデータを取得
   - 親子関係情報（`parentId`）を含むすべてのタスク情報を収集

2. **階層構造化処理 / Hierarchical Structuring**
   - タスクをIDでマップ化
   - 親子関係に基づいて階層構造を構築
   - 各タスクの階層レベルを計算

3. **表示・操作用の変換 / Conversion for Display and Operation**
   - 階層構造をフラット配列に変換（表示用）
   - 親タスクの展開・折りたたみ状態の管理

4. **タイムブロック生成連携 / Time Block Generation Integration**
   - サブタスクを考慮した時間配分計算
   - 親タスクとサブタスクの依存関係の反映

## 実装詳細 / Implementation Details

### 1. Todoistクライアント拡張 / Todoist Client Extension

#### 1.1 インターフェース定義 / Interface Definitions

`todoistClient.ts`に以下のインターフェースを追加しました：

```typescript
/**
 * Todoistタスクの基本インターフェース（最小限の型定義）
 * Basic Todoist Task Interface (minimal type definition)
 */
export interface TodoistTask {
  id: string;
  content: string;
  projectId: string;
  parentId?: string | null;
  isCompleted: boolean;
  [key: string]: any; // その他のプロパティを許容 / Allow other properties
}

/**
 * 階層構造を持つタスクのインターフェース
 * Hierarchical Task Interface
 */
export interface HierarchicalTask {
  id: string;
  content: string;
  projectId: string;
  parentId?: string | null;
  labels?: string[];
  priority?: number;
  isCompleted: boolean;
  due?: { date: string };
  // 階層構造のための追加フィールド / Additional fields for hierarchy
  subTasks: HierarchicalTask[];
  isSubTask: boolean;
  level: number;
  [key: string]: any; // その他のプロパティを許容 / Allow other properties
}
```

#### 1.2 サブタスク情報取得機能の追加 / Adding Subtask Information Retrieval

`todoistClient.ts`に以下の関数を追加しました：

```typescript
/**
 * サブタスク情報を含むタスクを取得
 * Retrieve tasks with subtask information
 * @param api - TodoistApiインスタンス / TodoistApi instance
 * @param filter - タスク検索条件 / Task filter conditions
 * @returns 階層構造化されたタスクの配列 / Array of hierarchically structured tasks
 */
export async function getTasksWithSubtasks(
  api: TodoistApi | any,
  filter?: TaskFilter
): Promise<HierarchicalTask[]> {
  // 全タスクを取得 / Get all tasks
  const allTasks = await getTasks(api, filter);

  // タスクを階層構造化 / Structure tasks hierarchically
  return buildTaskHierarchy(allTasks);
}

/**
 * 今日期限のサブタスク情報を含むタスクを取得
 * Retrieve today's tasks with subtask information
 * @param api - TodoistApiインスタンス / TodoistApi instance
 * @returns 階層構造化された今日期限のタスクの配列 / Array of hierarchically structured tasks due today
 */
export async function getTodayTasksWithSubtasks(
  api: TodoistApi | any
): Promise<HierarchicalTask[]> {
  // 今日のタスクを取得 / Get today's tasks
  const todayTasks = await getTodayTasks(api);

  // タスクを階層構造化 / Structure tasks hierarchically
  return buildTaskHierarchy(todayTasks);
}
```

#### 1.3 階層構造化処理関数の実装 / Implementing Hierarchical Structuring Functions

```typescript
/**
 * タスク配列を階層構造に変換
 * Convert task array to hierarchical structure
 * @param tasks - フラットなタスク配列 / Flat task array
 * @returns 階層構造化されたタスク配列 / Hierarchically structured task array
 */
export function buildTaskHierarchy(tasks: TodoistTask[]): HierarchicalTask[] {
  // タスクをIDでマップ化 / Map tasks by ID
  const taskMap = new Map<string, HierarchicalTask>();

  // 基本的なタスク構造を作成してマップに追加 / Create basic task structure and add to map
  tasks.forEach(task => {
    const structuredTask: HierarchicalTask = {
      ...task,
      subTasks: [],
      isSubTask: !!task.parentId,
      level: 0
    };
    taskMap.set(task.id, structuredTask);
  });

  // ルートタスク配列 / Root task array
  const rootTasks: HierarchicalTask[] = [];

  // 親子関係を構築 / Build parent-child relationships
  tasks.forEach(task => {
    const structuredTask = taskMap.get(task.id);

    if (structuredTask) {
      if (task.parentId && taskMap.has(task.parentId)) {
        // 親タスクが存在する場合、サブタスクとして追加 / If parent task exists, add as subtask
        const parentTask = taskMap.get(task.parentId);
        if (parentTask) {
          parentTask.subTasks.push(structuredTask);
          structuredTask.level = parentTask.level + 1;
          structuredTask.isSubTask = true;
        }
      } else {
        // 親タスクがない場合はルートタスク / If no parent task, it's a root task
        rootTasks.push(structuredTask);
      }
    }
  });

  return rootTasks;
}

/**
 * 階層構造をフラットな配列に変換（表示用）
 * Convert hierarchical structure to flat array (for display)
 * @param tasks - 階層構造化されたタスク配列 / Hierarchically structured task array
 * @returns フラット化されたタスク配列（階層情報付き） / Flattened task array (with hierarchy information)
 */
export function flattenTaskHierarchy(tasks: HierarchicalTask[]): HierarchicalTask[] {
  const result: HierarchicalTask[] = [];

  // 再帰的にタスクとそのサブタスクを追加 / Recursively add tasks and their subtasks
  function traverse(task: HierarchicalTask) {
    result.push(task);
    if (task.subTasks && task.subTasks.length > 0) {
      task.subTasks.forEach(subTask => traverse(subTask));
    }
  }

  // 各ルートタスクから処理開始 / Start processing from each root task
  tasks.forEach(task => traverse(task));

  return result;
}
```

### 2. タイムブロック生成機能の拡張 / Time Block Generation Extension

タイムブロック生成アルゴリズムを拡張して、サブタスクを考慮した時間配分を行います：

```typescript
/**
 * サブタスクを考慮したタイムブロック生成
 * Time block generation considering subtasks
 * @param tasks - 階層構造化されたタスク配列 / Hierarchically structured task array
 * @returns タイムブロックの配列 / Array of time blocks
 */
export function generateTimeBlocksWithSubtasks(tasks: HierarchicalTask[]): TimeBlock[] {
  const timeBlocks: TimeBlock[] = [];

  // 親タスクとサブタスクの時間配分を計算 / Calculate time allocation for parent tasks and subtasks
  function processTaskWithSubtasks(task: HierarchicalTask, startTime: Date) {
    let currentTime = new Date(startTime);

    // 親タスク自体の時間ブロックを作成 / Create time block for the parent task itself
    if (!task.subTasks || task.subTasks.length === 0) {
      // サブタスクがない場合は単純にタイムブロックを作成 / If no subtasks, simply create a time block
      const endTime = new Date(currentTime.getTime() + task.estimatedTime * 60000);
      timeBlocks.push({
        id: generateId(),
        taskId: task.id,
        startTime: currentTime.toISOString(),
        endTime: endTime.toISOString(),
        duration: task.estimatedTime,
        status: 'scheduled'
      });
      return endTime;
    }

    // 親タスク自体の時間（サブタスク以外の作業時間） / Time for the parent task itself (work time excluding subtasks)
    const parentTaskTime = Math.max(
      task.estimatedTime * 0.2, // 親タスク時間の最低20%は親タスク自体に割り当て / Allocate at least 20% of parent task time to the parent task itself
      task.estimatedTime - task.subTasks.reduce((sum, st) => sum + (st.estimatedTime || 0), 0)
    );

    if (parentTaskTime > 0) {
      const parentEndTime = new Date(currentTime.getTime() + parentTaskTime * 60000);
      timeBlocks.push({
        id: generateId(),
        taskId: task.id,
        startTime: currentTime.toISOString(),
        endTime: parentEndTime.toISOString(),
        duration: parentTaskTime,
        status: 'scheduled'
      });
      currentTime = parentEndTime;
    }

    // サブタスクの時間ブロックを作成 / Create time blocks for subtasks
    for (const subTask of task.subTasks) {
      currentTime = processTaskWithSubtasks(subTask, currentTime);
    }

    return currentTime;
  }

  // ルートタスクから処理開始 / Start processing from root tasks
  let currentTime = new Date(); // 開始時間（実際の実装では設定から取得） / Start time (in actual implementation, get from settings)
  for (const task of tasks) {
    currentTime = processTaskWithSubtasks(task, currentTime);
  }

  return timeBlocks;
}
```

## 外部ライブラリ使用状況 / External Library Usage

本機能では、以下の外部ライブラリを使用しています：

1. **@doist/todoist-api-typescript**
   - バージョン: 4.0.0-alpha.3
   - 使用目的: Todoist APIとの通信
   - 型定義確認: 実装前に型定義ファイル（.d.ts）を確認し、特にTask型の構造とparentId属性の扱いを検証
   - API応答処理: 応答形式の変更に対応するため、Array.isArrayチェックと適切なフォールバック処理を実装

## 例外処理 / Exception Handling

1. **親タスクが存在しない場合 / When Parent Task Does Not Exist**
   - `parentId`が指定されているが、対応する親タスクが取得できない場合
   - 対応: 親タスクが見つからない場合はルートタスクとして扱う

2. **循環参照の検出 / Circular Reference Detection**
   - タスクの親子関係に循環参照がある場合
   - 対応: 循環を検出した場合は警告を表示し、循環を解消する

3. **深すぎる階層の処理 / Handling Too Deep Hierarchies**
   - 非常に深い階層（例: 10階層以上）のサブタスク構造
   - 対応: 最大階層数を設定し、それを超える場合は警告を表示

## パフォーマンス要件 / Performance Requirements

1. **応答時間 / Response Time**
   - 100タスク（サブタスク含む）の階層構造化: 500ms以内
   - タイムブロック生成: 1000ms以内

2. **メモリ使用量 / Memory Usage**
   - 大量のタスク（1000以上）でもメモリ消費を抑える
   - 必要に応じて遅延読み込みを実装

## 技術的依存性 / Technical Dependencies

1. **外部API / External APIs**
   - Todoist REST API v9
   - `@doist/todoist-api-typescript` パッケージ

2. **内部コンポーネント / Internal Components**
   - タスクマネージャーモジュール
   - タイムブロック生成エンジン
   - データモデル（Task, TimeBlock）

## テスト戦略 / Testing Strategy

1. **ユニットテスト / Unit Tests**
   - `buildTaskHierarchy` 関数のテスト
   - `flattenTaskHierarchy` 関数のテスト
   - サブタスクを含むタスク取得関数のテスト

2. **統合テスト / Integration Tests**
   - Todoistからのサブタスク取得～階層構造化～タイムブロック生成の一連のフロー
   - 様々な階層構造パターンでのテスト

3. **エッジケーステスト / Edge Case Tests**
   - 非常に深い階層構造
   - 循環参照を含むデータ
   - 親タスクが取得できないサブタスク

## 実装ロードマップ / Implementation Roadmap

1. **フェーズ1: 基本機能実装 / Phase 1: Basic Functionality Implementation** ✅
   - サブタスク情報を含むタスク取得機能
   - 階層構造化処理関数
   - 基本的なテストケース

2. **フェーズ2: タイムブロック連携 / Phase 2: Time Block Integration**
   - サブタスクを考慮したタイムブロック生成
   - 親子タスク間の時間配分ロジック
   - 拡張テストケース

3. **フェーズ3: UI/UX最適化 / Phase 3: UI/UX Optimization**
   - 階層的なタスク表示
   - 展開・折りたたみ機能
   - パフォーマンス最適化

## 関連ドキュメント / Related Documents

- [タスクマネージャーモジュール仕様 / Task Manager Module Specification](../modules/task-manager.md)
- [タイムブロック生成機能 / Time Block Generation Feature](./timeblock-generation.md)
- [データモデル定義 / Data Model Definition](../data/models.md)