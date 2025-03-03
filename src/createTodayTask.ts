/**
 * 今日期限のタスクを作成するスクリプト
 *
 * このスクリプトは、Todoistに今日期限のタスクとそのサブタスクを作成します。
 * これにより、getTodayTasksWithSubtasks関数の動作を確認できます。
 */

import { createTodoistApi, TodoistTask } from './lib/todoistClient';

/**
 * メイン関数
 */
async function main() {
  try {
    console.log('Todoistに今日期限のタスクを作成しています...');

    // TodoistAPIクライアントを作成
    const api = createTodoistApi();

    // 今日の日付を取得 (YYYY-MM-DD形式)
    const today = new Date().toISOString().split('T')[0];
    console.log(`今日の日付: ${today}`);

    // 親タスクを作成
    const parentTask = await api.addTask({
      content: '今日のテストタスク（親）',
      description: 'これは親タスクです',
      dueDate: today
    });

    console.log(`親タスクを作成しました: ${parentTask.content} (ID: ${parentTask.id})`);

    // サブタスク1を作成（今日期限）
    const subTask1 = await api.addTask({
      content: '今日のテストタスク（サブ1）',
      description: 'これはサブタスク1です',
      parentId: parentTask.id,
      dueDate: today
    });

    console.log(`サブタスク1を作成しました: ${subTask1.content} (ID: ${subTask1.id})`);

    // サブタスク2を作成（期限なし）
    const subTask2 = await api.addTask({
      content: '期限なしのテストタスク（サブ2）',
      description: 'これはサブタスク2です',
      parentId: parentTask.id
    });

    console.log(`サブタスク2を作成しました: ${subTask2.content} (ID: ${subTask2.id})`);

    // 作成したタスクの詳細情報を取得
    console.log('\n作成したタスクの詳細情報:');

    const response = await api.getTasks();
    const allTasks = Array.isArray(response) ? response : (response.results || []);
    const createdTasks = allTasks.filter((task: TodoistTask) =>
      [parentTask.id, subTask1.id, subTask2.id].includes(task.id)
    );
    createdTasks.forEach((task: TodoistTask) => {
      console.log(`\nタスク: ${task.content} (ID: ${task.id})`);
      console.log(`説明: ${task.description}`);
      console.log(`親ID: ${task.parentId || 'なし'}`);
      console.log(`期限日: ${task.due?.date || '未設定'}`);
    });

    console.log('\n今日期限のタスクとサブタスクを作成しました。');
    console.log('fetchTodaySubtasks.tsを実行して、取得できるか確認してください。');

  } catch (error) {
    console.error('エラーが発生しました:', error);
  }
}

// スクリプト実行
main();