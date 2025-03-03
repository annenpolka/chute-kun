import { TodoistApi } from '@doist/todoist-api-typescript';
import * as dotenv from 'dotenv';

dotenv.config();

async function main() {
  const api = new TodoistApi(process.env.TODOIST_API_TOKEN || '');
  console.log('親タスク「chute進める」のサブタスクを追加します...');

  try {
    // まず親タスクのIDを取得
    const tasksResponse = await api.getTasks({ filter: 'today' });
    // APIレスポンスが配列の場合と、オブジェクトの場合の両方に対応
    const tasks = Array.isArray(tasksResponse) ? tasksResponse : [];
    const parentTask = tasks.find(t => t.content === 'chute進める');

    if (parentTask) {
      // サブタスクを追加
      const newTask = await api.addTask({
        content: 'chuteサブタスク：コードの改善',
        parentId: parentTask.id,
        priority: 3
      });
      console.log('サブタスクを追加しました:', newTask.id);
    } else {
      console.log('親タスク「chute進める」が見つかりませんでした');
    }
  } catch (error) {
    console.error('エラーが発生しました:', error);
  }
}

main();