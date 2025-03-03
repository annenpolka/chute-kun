import { TodoistApi } from '@doist/todoist-api-typescript';
import * as path from 'path';
import dotenv from 'dotenv';

// dotenvで環境変数を読み込む
dotenv.config({ path: path.resolve(process.cwd(), '.env') });

/**
 * タスク検索条件のインターフェース
 */
export interface TaskFilter {
  projectId?: string;
  labelIds?: string[];
  dueDate?: string;
  isCompleted?: boolean;
  priority?: number;
}

/**
 * 階層構造を持つタスクのインターフェース
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
  // 階層構造のための追加フィールド
  subTasks: HierarchicalTask[];
  isSubTask: boolean;
  level: number;
  [key: string]: any; // その他のプロパティを許容
}

/**
 * Todoistタスクの基本インターフェース（最小限の型定義）
 */
export interface TodoistTask {
  id: string;
  content: string;
  projectId: string;
  parentId?: string | null;
  isCompleted: boolean; // 必須プロパティとして追加
  [key: string]: any; // その他のプロパティを許容
}

/**
 * Todoist APIクライアントを作成
 * @param apiToken - Todoist APIトークン（省略時は環境変数から取得）
 * @returns TodoistApiインスタンス
 */
export function createTodoistApi(apiToken?: string): TodoistApi {
  const token = apiToken || process.env.TODOIST_API_TOKEN;
  if (!token) {
    throw new Error('Todoist APIトークンが設定されていません。');
  }

  // デバッグ用にトークンの最初の数文字を表示（セキュリティのため全ては表示しない）
  console.log(`Using API token: ${token.substring(0, 5)}...${token.substring(token.length - 3)}`);

  return new TodoistApi(token);
}

/**
 * 条件に基づいてタスクを取得
 * @param api - TodoistApiインスタンス
 * @param filter - タスク検索条件
 * @returns 条件に合致するタスクの配列
 */
export async function getTasks(api: TodoistApi | any, filter?: TaskFilter): Promise<TodoistTask[]> {
  try {
    const response = await api.getTasks();

    // APIの戻り値を確認
    console.log('API response type:', typeof response);

    // 新しいTodoist APIではレスポンスがオブジェクトで、resultsプロパティに配列がある
    // APIの戻り値を正規化（配列、resultsプロパティ、itemsプロパティのいずれかに対応）
    // Array format: [task1, task2, ...]
    // Object format: { results: [task1, task2, ...] } or { items: [task1, task2, ...] }
    const tasks = Array.isArray(response) ? response : (response.results || response.items || []);

    if (!filter) {
      return tasks as TodoistTask[];
    }

    return tasks.filter((task: any) => {
      // プロジェクトIDでフィルタリング
      if (filter.projectId && task.projectId !== filter.projectId) {
        return false;
      }

      // ラベルでフィルタリング
      if (filter.labelIds && filter.labelIds.length > 0) {
        // APIの戻り値によってlabels or labelIdsのどちらかを使用
        const taskLabels = task.labelIds || task.labels;
        if (!taskLabels || !filter.labelIds.some(id => taskLabels.includes(id))) {
          return false;
        }
      }

      // 期限日でフィルタリング
      if (filter.dueDate && task.due) {
        const dueDate = task.due.date;
        if (dueDate !== filter.dueDate) {
          return false;
        }
      }

      // 完了状態でフィルタリング
      if (filter.isCompleted !== undefined && task.isCompleted !== filter.isCompleted) {
        return false;
      }

      // 優先度でフィルタリング
      if (filter.priority !== undefined && task.priority !== filter.priority) {
        return false;
      }

      return true;
    }) as TodoistTask[];
  } catch (error) {
    console.error('タスク取得中にエラーが発生しました:', error);
    throw error;
  }
}

/**
 * 今日期限のタスクを取得
 * @param api - TodoistApiインスタンス
 * @returns 今日期限のタスク一覧
 */
export async function getTodayTasks(api: TodoistApi | any): Promise<TodoistTask[]> {
  try {
    // 新APIではdueストリングでフィルタできるため、直接取得を試みる
    const response = await api.getTasks({ filter: 'today' });
    console.log('Using direct today filter');

    // APIの戻り値を正規化
    // 配列形式、resultsプロパティ、itemsプロパティのいずれかに対応
    // Array format: [task1, task2, ...]
    // Object format: { results: [task1, task2, ...] } or { items: [task1, task2, ...] }
    const tasks = Array.isArray(response) ? response : (response.results || response.items || []);

    // 完了済みタスクを除外
    return tasks.filter((task: any) => !task.isCompleted) as TodoistTask[];
  } catch (error) {
    // 直接フィルタが失敗した場合は従来の方法を使用
    console.log('Fallback to date filtering');
    const today = new Date().toISOString().split('T')[0]; // YYYY-MM-DD形式
    return getTasks(api, { dueDate: today, isCompleted: false });
  }
}

/**
 * タスク配列を階層構造に変換
 * @param tasks - フラットなタスク配列
 * @returns 階層構造化されたタスク配列
 */
export function buildTaskHierarchy(tasks: TodoistTask[]): HierarchicalTask[] {
  // タスクをIDでマップ化
  const taskMap = new Map<string, HierarchicalTask>();

  // 基本的なタスク構造を作成してマップに追加
  tasks.forEach(task => {
    const structuredTask: HierarchicalTask = {
      ...task,
      // parentIdとparent_idの両方に対応（内部的にはparentIdに正規化）
      parentId: task.parentId || task.parent_id,
      subTasks: [],
      // parentIdとparent_idの両方に対応
      isSubTask: !!(task.parentId || task.parent_id),
      level: 0
    };
    taskMap.set(task.id, structuredTask);
  });

  // ルートタスク配列
  const rootTasks: HierarchicalTask[] = [];

  // 親子関係を構築
  tasks.forEach(task => {
    const structuredTask = taskMap.get(task.id);

    if (structuredTask) {
      // parentIdとparent_idの両方に対応
      const parentId = task.parentId || task.parent_id;
      if (parentId && taskMap.has(parentId)) {
        // 親タスクが存在する場合、サブタスクとして追加
        const parentTask = taskMap.get(parentId);
        if (parentTask) {
          parentTask.subTasks.push(structuredTask);
          structuredTask.level = parentTask.level + 1;
          structuredTask.isSubTask = true;
        }
      } else {
        // 親タスクがない場合はルートタスク
        rootTasks.push(structuredTask);
      }
    }
  });

  return rootTasks;
}

/**
 * 階層構造をフラットな配列に変換（表示用）
 * @param tasks - 階層構造化されたタスク配列
 * @returns フラット化されたタスク配列（階層情報付き）
 */
export function flattenTaskHierarchy(tasks: HierarchicalTask[]): HierarchicalTask[] {
  const result: HierarchicalTask[] = [];

  // 再帰的にタスクとそのサブタスクを追加
  function traverse(task: HierarchicalTask) {
    result.push(task);
    if (task.subTasks && task.subTasks.length > 0) {
      task.subTasks.forEach(subTask => traverse(subTask));
    }
  }

  // 各ルートタスクから処理開始
  tasks.forEach(task => traverse(task));

  return result;
}

/**
 * サブタスク情報を含むタスクを取得
 * @param api - TodoistApiインスタンス
 * @param filter - タスク検索条件
 * @returns 階層構造化されたタスクの配列
 */
export async function getTasksWithSubtasks(
  api: TodoistApi | any,
  filter?: TaskFilter
): Promise<HierarchicalTask[]> {
  // 全タスクを取得
  const allTasks = await getTasks(api, filter);

  // タスクを階層構造化
  return buildTaskHierarchy(allTasks);
}

/**
 * 今日期限のタスクとそのサブタスク（期限問わず）を取得
 * @param api - TodoistApiインスタンス
 * @returns 階層構造化された今日期限のタスクとそのサブタスクの配列
 */
export async function getTodayTasksWithSubtasks(
  api: TodoistApi | any
): Promise<HierarchicalTask[]> {
  try {
    console.log('getTodayTasksWithSubtasks: 今日期限のタスクとそのサブタスクを取得します');

    // 全タスクを取得
    const allTasks = await getTasks(api);

    // 今日期限のタスクを取得（Todoist APIのfilter: 'today'パラメータを使用）
    const todayTasks = await getTodayTasks(api);
    console.log(`今日期限のタスク数（getTodayTasks）: ${todayTasks.length}`);

    // テスト用のタスクを表示
    const testTasks = todayTasks.filter(task => task.content.includes('テストタスク'));
    console.log(`テスト用の今日期限のタスク数: ${testTasks.length}`);
    testTasks.forEach(task => console.log(`- ${task.content} (ID: ${task.id})`));

    // 今日期限のタスクがない場合は空配列を返す
    if (todayTasks.length === 0) {
      console.log('今日期限のタスクが見つかりませんでした');
      return [];
    }

    // 今日期限のタスクのIDを抽出
    const todayTaskIds = todayTasks.map(task => task.id);
    console.log(`今日期限のタスクID: ${todayTaskIds.join(', ')}`);

    // 全タスクを再取得（サブタスク情報を含む）
    console.log('全タスクを再取得します...');
    const response = await api.getTasks();
    // APIの戻り値を正規化（配列、resultsプロパティ、itemsプロパティのいずれかに対応）
    // Array format: [task1, task2, ...]
    // Object format: { results: [task1, task2, ...] } or { items: [task1, task2, ...] }
    const tasks = Array.isArray(response) ? response : (response.results || response.items || []) as TodoistTask[];
    console.log(`全タスク数（再取得）: ${tasks.length}`);

    // APIの戻り値の形式を確認
    if (tasks.length > 0) {
      const sampleTask = tasks[0];
      const sampleSubTask = tasks.find(task => task.parentId || task.parent_id);
      if (sampleSubTask) {
        console.log('サンプルサブタスクの形式:');
        console.log(JSON.stringify(sampleSubTask, null, 2));
      }

      console.log('サンプルタスクの形式:');
      console.log(JSON.stringify(sampleTask, null, 2));
    }

    // 今日期限のタスクを含める
    const relevantTasks: TodoistTask[] = [];

    // 今日期限のタスクを追加
    for (const todayTaskId of todayTaskIds) {
      const todayTask = tasks.find(task => task.id === todayTaskId);
      if (todayTask) {
        relevantTasks.push(todayTask);
        console.log(`今日期限のタスクを追加: ${todayTask.content} (ID: ${todayTask.id})`);
      }
    }

    // サブタスクを検索して追加
    for (const task of tasks) {
      // デバッグ: すべてのタスクの親子関係を確認
      // 親子関係の検出には、parentId、parent_id、parentのいずれかのプロパティを使用
      if (task.parentId || task.parent_id) {
        console.log(`サブタスク検出: ${task.content} (ID: ${task.id})`);
        console.log(`  親ID: ${task.parentId || task.parent_id}`);
      }

      // parentIdプロパティを確認（TypeScriptクライアント形式）
      if (task.parentId && todayTaskIds.includes(task.parentId)) {
        console.log(`サブタスク候補（parentId）: ${task.content} (ID: ${task.id}, 親ID: ${task.parentId})`);
        if (!relevantTasks.some(t => t.id === task.id)) {
          relevantTasks.push(task);
          console.log(`サブタスクを追加（parentId）: ${task.content} (ID: ${task.id}, 親ID: ${task.parentId})`);
        }
      }
      // parent_idプロパティを確認（API応答形式）
      else if (task.parent_id && todayTaskIds.includes(task.parent_id)) {
        console.log(`サブタスク候補（parent_id）: ${task.content} (ID: ${task.id}, 親ID: ${task.parent_id})`);
        if (!relevantTasks.some(t => t.id === task.id)) {
          const taskWithParentId = { ...task, parentId: task.parent_id };
          relevantTasks.push(taskWithParentId);
          console.log(`サブタスクを追加（parent_id）: ${task.content} (ID: ${task.id}, 親ID: ${task.parent_id})`);
        }
      }
      // 統合テスト用の特別処理：文字列形式のparentIdを確認
      else if (typeof task.parentId === 'string' && todayTaskIds.some(id => task.parentId === id)) {
        console.log(`サブタスク候補（文字列parentId）: ${task.content} (ID: ${task.id}, 親ID: ${task.parentId})`);
        if (!relevantTasks.some(t => t.id === task.id)) {
          relevantTasks.push(task);
          console.log(`サブタスクを追加（文字列parentId）: ${task.content} (ID: ${task.id}, 親ID: ${task.parentId})`);
        }
      // parentプロパティを確認（別の可能性のある形式）
      } else if (task.parent && todayTaskIds.includes(task.parent)) {
        console.log(`サブタスク候補（parent）: ${task.content} (ID: ${task.id}, 親ID: ${task.parent})`);
      }
    }

    console.log(`関連タスク数: ${relevantTasks.length}`);

    // タスクを階層構造化
    return buildTaskHierarchy(relevantTasks);
  } catch (error) {
    console.error('getTodayTasksWithSubtasks: エラーが発生しました', error);
    throw error;
  }
}