import { TodoistApi } from '@doist/todoist-api-typescript';
import * as path from 'path';
import dotenv from 'dotenv';

// dotenvで環境変数を読み込む
dotenv.config({ path: path.resolve(process.cwd(), '.env') });

// デバッグモード設定
let debugMode = process.env.DEBUG_MODE === 'true';

/**
 * デバッグモードを設定する
 * @param isDebug - デバッグモードを有効にするかどうか
 */
export function setDebugMode(isDebug: boolean): void {
  debugMode = isDebug;
}

/**
 * デバッグメッセージを出力する
 * @param message - 出力するメッセージ
 * @param data - 追加のデータ（オプション）
 */
export function debug(message: string, data?: any): void {
  if (debugMode) {
    if (data !== undefined) {
      console.log(`[DEBUG] ${message}`, data);
    } else {
      console.log(`[DEBUG] ${message}`);
    }
  }
}

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
  debug(`Using API token: ${token.substring(0, 5)}...${token.substring(token.length - 3)}`);

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
    debug('API response type:', typeof response);
    debug('API response structure:', response);

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
    debug('Using direct today filter');
    debug('Today tasks response:', response);

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
      // parentId、parent_id、parentの3つの形式に対応（内部的にはparentIdに正規化）
      parentId: task.parentId || task.parent_id || task.parent,
      subTasks: [],
      // 全ての親子関係表現形式に対応
      isSubTask: !!(task.parentId || task.parent_id || task.parent),
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
      // parentId、parent_id、parentの3つの形式に対応
      const parentId = task.parentId || task.parent_id || task.parent;
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
 * 直接APIからサブタスクを取得する関数
 * @param api - TodoistApiインスタンス 
 * @param parentId - 親タスクID
 * @returns 親タスクの直下のサブタスク配列
 */
export async function getSubtasks(
  api: TodoistApi | any,
  parentId: string
): Promise<TodoistTask[]> {
  console.log(`親タスク(ID: ${parentId})のサブタスクを直接APIから取得します...`);
  
  try {
    // 全タスクを取得
    const allTasks = await getTasks(api);
    
    // 親IDに一致するサブタスクをフィルタリング
    const subtasks = allTasks.filter(task => {
      // parentId、parent_id、parentのいずれかを使用して親子関係を検出
      const taskParentId = task.parentId || task.parent_id || task.parent;
      return taskParentId === parentId;
    });
    
    return subtasks;
  } catch (error) {
    console.error(`サブタスク取得中にエラーが発生しました:`, error);
    throw error;
  }
}

/**
 * 再帰的にサブタスクを取得して階層構造を構築する関数
 * @param api - TodoistApiインスタンス
 * @param parentId - 親タスクID
 * @param level - 階層レベル（再帰呼び出し用）
 * @returns 階層構造化されたサブタスクの配列
 */
export async function getSubtasksRecursive(
  api: TodoistApi | any,
  parentId: string,
  level = 1
): Promise<HierarchicalTask[]> {
  console.log(`階層レベル ${level} - 親タスク(ID: ${parentId})のサブタスクを再帰的に取得します...`);
  
  try {
    // 直接のサブタスクを取得
    const directSubtasks = await getSubtasks(api, parentId);
    
    // 階層構造を構築
    const result: HierarchicalTask[] = [];
    
    for (const task of directSubtasks) {
      // 階層タスクの構築
      const hierarchicalTask: HierarchicalTask = {
        ...task,
        parentId: task.parentId || task.parent_id || task.parent,
        subTasks: [], // 後で更新
        isSubTask: true,
        level: level
      };
      
      // 子タスク（このタスクのサブタスク）を再帰的に取得
      hierarchicalTask.subTasks = await getSubtasksRecursive(api, task.id, level + 1);
      
      result.push(hierarchicalTask);
    }
    
    return result;
  } catch (error) {
    console.error(`再帰的サブタスク取得中にエラーが発生しました:`, error);
    throw error;
  }
}

/**
 * 指定されたタスクIDのすべてのサブタスク（多階層）を取得
 * @param tasks - すべてのタスクの配列
 * @param parentId - 親タスクID
 * @param collectedIds - 既に集められたタスクIDのセット（再帰呼び出し用）
 * @returns 親タスクの下位にあるすべてのサブタスクIDのセット
 */
export function collectAllSubtaskIds(
  tasks: TodoistTask[],
  parentId: string,
  collectedIds: Set<string> = new Set<string>()
): Set<string> {
  // 親タスクの直下のサブタスクを検索
  const directSubtasks = tasks.filter(task => {
    // parentId、parent_id、parentのいずれかを使用して親子関係を検出
    const taskParentId = task.parentId || task.parent_id || task.parent;
    return taskParentId === parentId;
  });

  // 直下のサブタスクがない場合
  if (directSubtasks.length === 0) {
    return collectedIds;
  }

  // 直下のサブタスクをセットに追加
  directSubtasks.forEach(subtask => {
    collectedIds.add(subtask.id);
    // 再帰的に各サブタスクのサブタスクを検索
    collectAllSubtaskIds(tasks, subtask.id, collectedIds);
  });

  return collectedIds;
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
    // まず全タスクを取得
    const allTasks = await getTasks(api);

    // 今日期限のタスクを取得
    const todayTasks = await getTodayTasks(api);

    // 今日期限のタスクがない場合は空配列を返す
    if (todayTasks.length === 0) {
      return [];
    }

    // 階層構造を持つタスク配列
    const hierarchicalTasks: HierarchicalTask[] = [];
    
    // 今日期限の各タスクを処理
    for (const todayTask of todayTasks) {
      // 親タスクの階層構造を作成
      const hierarchicalTask: HierarchicalTask = {
        ...todayTask,
        subTasks: [], // 後で埋める
        isSubTask: false,
        level: 0
      };
      
      // サブタスクを再帰的に取得して階層構造を構築
      hierarchicalTask.subTasks = await getSubtasksRecursive(api, todayTask.id);
      
      // 結果配列に追加
      hierarchicalTasks.push(hierarchicalTask);
    }
    
    return hierarchicalTasks;
  } catch (error) {
    console.error('getTodayTasksWithSubtasks: エラーが発生しました', error);
    throw error;
  }
}