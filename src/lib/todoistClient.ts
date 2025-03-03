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
export async function getTasks(api: TodoistApi | any, filter?: TaskFilter): Promise<any[]> {
  try {
    const response = await api.getTasks();
    
    // APIの戻り値を確認
    console.log('API response type:', typeof response);
    
    // 新しいTodoist APIではレスポンスがオブジェクトで、resultsプロパティに配列がある
    const tasks = Array.isArray(response) ? response : (response.results || []);
    
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
  try {
    // 新APIではdueストリングでフィルタできるため、直接取得を試みる
    const response = await api.getTasks({ filter: 'today' });
    console.log('Using direct today filter');
    
    // APIの戻り値を正規化
    const tasks = Array.isArray(response) ? response : (response.results || []);
    
    // 完了済みタスクを除外
    return tasks.filter((task: any) => !task.isCompleted);
  } catch (error) {
    // 直接フィルタが失敗した場合は従来の方法を使用
    console.log('Fallback to date filtering');
    const today = new Date().toISOString().split('T')[0]; // YYYY-MM-DD形式
    return getTasks(api, { dueDate: today, isCompleted: false });
  }
}