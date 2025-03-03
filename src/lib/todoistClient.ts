import { TodoistApi } from '@doist/todoist-api-typescript';
import dotenv from 'dotenv';

// 環境変数の読み込み
dotenv.config();

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
 * Todoist APIクライアントを作成
 * @param apiToken - Todoist APIトークン（省略時は環境変数から取得）
 * @returns TodoistApiインスタンス
 */
export function createTodoistApi(apiToken?: string): TodoistApi {
  const token = apiToken || process.env.TODOIST_API_TOKEN;
  if (!token) {
    throw new Error('Todoist APIトークンが設定されていません。');
  }
  return new TodoistApi(token);
}

/**
 * 条件に基づいてタスクを取得
 * @param api - TodoistApiインスタンス
 * @param filter - タスク検索条件
 * @returns 条件に合致するタスクの配列
 */
export async function getTasks(api: TodoistApi | any, filter?: TaskFilter): Promise<any[]> {
  try {
    const tasks = await api.getTasks();
    
    if (!filter) {
      return tasks;
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
    });
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
export async function getTodayTasks(api: TodoistApi | any): Promise<any[]> {
  const today = new Date().toISOString().split('T')[0]; // YYYY-MM-DD形式
  return getTasks(api, { dueDate: today, isCompleted: false });
}