import { createTodoistApi, getTodayTasks } from './lib/todoistClient';

/**
 * メイン実行関数
 */
async function main() {
  try {
    // Todoist APIクライアントを作成
    const api = createTodoistApi();
    
    console.log('今日のタスクを取得中...');
    
    // 今日のタスクを取得
    const todayTasks = await getTodayTasks(api);
    
    console.log(`今日のタスク (${todayTasks.length}件):`);
    
    // タスク一覧を表示
    todayTasks.forEach((task, index) => {
      const priority = '!'.repeat(task.priority) || '-';
      console.log(`${index + 1}. [${priority}] ${task.content}`);
    });
    
  } catch (error) {
    console.error('エラーが発生しました:', error);
    process.exit(1);
  }
}

// スクリプト実行
if (require.main === module) {
  main();
}

export { main };