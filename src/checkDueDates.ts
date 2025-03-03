/**
 * タスクの期限日を確認するスクリプト
 *
 * このスクリプトは、Todoistから全タスクを取得し、
 * 期限日の設定状況を確認します。
 */

import { createTodoistApi, getTasks, TodoistTask } from './lib/todoistClient';

/**
 * メイン関数
 */
async function main() {
  try {
    console.log('Todoistからタスクの期限日情報を取得しています...');

    // TodoistAPIクライアントを作成
    const api = createTodoistApi();

    // 全タスクを取得
    const allTasks = await getTasks(api);

    console.log(`\n=== 全タスク数: ${allTasks.length} ===\n`);

    // 期限日が設定されているタスクをカウント
    const tasksWithDueDate = allTasks.filter(task => task.due && task.due.date);
    console.log(`期限日が設定されているタスク数: ${tasksWithDueDate.length}`);

    // 今日の日付を取得 (YYYY-MM-DD形式)
    const today = new Date().toISOString().split('T')[0];
    console.log(`今日の日付: ${today}`);

    // 今日期限のタスクをカウント
    const todayTasks = tasksWithDueDate.filter(task => task.due.date === today);
    console.log(`今日期限のタスク数: ${todayTasks.length}`);

    // 期限日の形式を確認
    console.log('\n=== 期限日の形式サンプル ===');
    if (tasksWithDueDate.length > 0) {
      const sampleTask = tasksWithDueDate[0];
      console.log(`タスク: ${sampleTask.content}`);
      console.log(`期限日: ${sampleTask.due.date}`);
      console.log(`期限日のデータ型: ${typeof sampleTask.due.date}`);
      console.log(`期限日オブジェクト全体: ${JSON.stringify(sampleTask.due, null, 2)}`);
    } else {
      console.log('期限日が設定されているタスクがありません');
    }

    // 全タスクの期限日を表示
    console.log('\n=== 全タスクの期限日 ===');
    allTasks.forEach(task => {
      console.log(`タスク: ${task.content}`);
      if (task.due && task.due.date) {
        console.log(`  期限日: ${task.due.date}`);
      } else {
        console.log('  期限日: 未設定');
      }
    });

  } catch (error) {
    console.error('エラーが発生しました:', error);
  }
}

// スクリプト実行
main();