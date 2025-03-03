import { TodoistApi } from '@doist/todoist-api-typescript';
import dotenv from 'dotenv';
import * as path from 'path';

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
  priority?: number; // 優先度（1-4）
  due?: { date: string; }; // 期限
  [key: string]: any; // その他のプロパティを許容
}

// フォーマット関連機能は formatters/taskFormatter.ts へ移動しました

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
 * タスクを取得
 * @param api - TodoistApiインスタンス
 * @param filter - タスク検索条件
 * @returns 条件に合致するタスクの配列
 */
export async function getTasks(api: TodoistApi | any, filter?: TaskFilter): Promise<TodoistTask[]> {
  try {
    console.log('タスク取得開始...');

    // パラメータなしでAPI呼び出し
    const response: any = await api.getTasks();

    // APIの戻り値の型を確認
    console.log('API response type:', typeof response);

    // 新しいTodoist APIではレスポンスがオブジェクトで、resultsプロパティに配列がある
    // APIの戻り値を正規化
    const tasks = Array.isArray(response)
      ? response
      : (response.results || response.items || []);

    console.log(`タスク取得完了: ${tasks.length}件`);

    // APIがデフォルトで50件しか返さない場合、ユーザーに警告
    if (tasks.length === 50) {
      console.log('注意: Todoist APIがデフォルトで最大50件のタスクしか返しません。');
      console.log('取得されていないタスクがある可能性があります。');
    }

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
 * 日付をYYYY-MM-DD形式にフォーマットする
 * @param date - フォーマットする日付
 * @returns YYYY-MM-DD形式の日付文字列
 */
function formatDate(date: Date): string {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
}

/**
 * 今日のタスクを取得
 * @param api - TodoistApiインスタンス
 * @returns 今日が期限のタスク配列
 */
export async function getTodayTasks(api: TodoistApi | any): Promise<TodoistTask[]> {
  try {
    console.log('今日のタスクを取得中...');

    // すべてのタスクを取得（ページネーション対応済みのgetTasks関数を使用）
    const allTasks = await getTasks(api);

    // 今日の日付を取得
    const today = new Date();
    today.setHours(0, 0, 0, 0);
    const todayStr = formatDate(today);

    console.log(`今日の日付: ${todayStr}`);
    console.log(`取得したタスク総数: ${allTasks.length}件`);

    // 今日が期限のタスクをフィルタリング
    const todayTasks = allTasks.filter((task: TodoistTask) => {
      // 完了済みタスクを除外
      if (task.isCompleted) return false;

      // 期限日がないタスクを除外
      if (!task.due || !task.due.date) return false;

      // デバッグ情報を出力
      // console.log(`タスク「${task.content}」の期限: ${task.due.date}, 今日: ${todayStr}`);

      // 期限日が今日のタスクを抽出
      return task.due.date === todayStr;
    });

    console.log(`今日が期限のタスク数: ${todayTasks.length}件`);

    // デバッグ用に今日のタスクの内容と期限日を出力
    todayTasks.forEach((task: TodoistTask) => {
      console.log(`今日のタスク: 「${task.content}」(期限: ${task.due?.date})`);
    });

    return todayTasks;
  } catch (error) {
    console.error('今日のタスク取得中にエラーが発生しました:', error);

    // エラー発生時は現在の日付でフィルタリングして取得
    const today = new Date();
    return getTasks(api, {
      dueDate: formatDate(today),
      isCompleted: false
    });
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
 * 期限に関係なく、指定された親タスクの直下のサブタスクをすべて取得します
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
    // 全タスクを取得（期限でフィルタリングしない）
    const allTasks = await getTasks(api);

    // 親IDに一致するサブタスクをフィルタリング（期限条件なし）
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
 * 期限に関係なく、指定された親タスクのすべてのサブタスクを取得します
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
    // 直接のサブタスクを取得 (期限に関係なくすべて取得)
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
 * サブタスクは期限に関係なく全て取得します
 * @param api - TodoistApiインスタンス
 * @returns 階層構造化された今日期限のタスクとそのサブタスクの配列
 */
export async function getTodayTasksWithSubtasks(
  api: TodoistApi | any
): Promise<HierarchicalTask[]> {
  try {
    // まず全タスクを取得
    debug('Getting all tasks for today with subtasks');
    const allTasks = await getTasks(api);
    debug(`Retrieved ${allTasks.length} total tasks`);

    // 今日の日付を取得（YYYY-MM-DD形式）
    const today = new Date().toISOString().split('T')[0];
    debug(`Today's date: ${today}`);

    // 今日期限の親タスクIDを抽出
    const todayTaskIds = allTasks
      .filter(task => {
        // 親タスク（サブタスクではない）で、完了していなくて、今日が期限のもの
        const isParentTask = !(task.parentId || task.parent_id || task.parent);
        // より明示的なnullチェック
        const hasDueDate = typeof task.due === 'object' && task.due !== null && typeof task.due.date === 'string';
        const isDueToday = hasDueDate && task.due!.date === today;
        const isNotCompleted = !task.isCompleted;

        return isParentTask && isDueToday && isNotCompleted;
      })
      .map(task => task.id);

    debug(`Found ${todayTaskIds.length} parent tasks due today`);

    if (todayTaskIds.length === 0) {
      debug('No tasks due today, returning empty array');
      return [];
    }

    // 今日期限のタスクとそのサブタスクを含むリストを作成
    const tasksToProcess = allTasks.filter(task => {
      // 今日期限の親タスク自体を含める
      if (todayTaskIds.includes(task.id)) return true;

      // 今日期限の親タスクのサブタスクを含める（期限に関わらず全て）
      const taskParentId = task.parentId || task.parent_id || task.parent;

      // 親IDがある場合、それが今日期限の親タスクIDに含まれているか、
      // もしくは既に含まれるサブタスクの子タスクであるかを再帰的に確認
      if (taskParentId) {
        // 直接の親が今日期限のタスクである場合
        if (todayTaskIds.includes(taskParentId)) return true;

        // 間接的な親子関係を調べる（親の親...が今日期限のタスクである場合）
        let currentParentId = taskParentId;
        let ancestorTask;

        // 親をたどって今日期限のタスクに行き着くか確認
        while (currentParentId) {
          ancestorTask = allTasks.find(t => t.id === currentParentId);
          if (!ancestorTask) break;

          const ancestorParentId = ancestorTask.parentId || ancestorTask.parent_id || ancestorTask.parent;
          if (!ancestorParentId) break;

          if (todayTaskIds.includes(ancestorParentId)) return true;
          currentParentId = ancestorParentId;
        }
      }

      return false;
    });

    debug(`Processing ${tasksToProcess.length} tasks (including subtasks) for today's view`);

    // 階層構造を構築
    const hierarchicalTasks = buildTaskHierarchy(tasksToProcess)
      .filter(task => todayTaskIds.includes(task.id));

    debug(`Returning ${hierarchicalTasks.length} hierarchical tasks`);
    return hierarchicalTasks;
  } catch (error) {
    console.error('今日のタスクとサブタスク取得中にエラーが発生しました:', error);
    throw error;
  }
}