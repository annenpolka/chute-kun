/**
 * サブタスクを取得するサンプルスクリプト
 *
 * このスクリプトは、Todoistからサブタスク情報を含むタスクを取得し、
 * 階層構造で表示します。
 */

import { createTodoistApi, getTasksWithSubtasks, HierarchicalTask } from './lib/todoistClient';

/**
 * タスクを階層構造で表示する関数
 * @param tasks - 階層構造化されたタスク配列
 * @param indent - インデントレベル（再帰呼び出し用）
 */
function displayTaskHierarchy(tasks: HierarchicalTask[], indent = 0): void {
  tasks.forEach(task => {
    // インデントを適用してタスク内容を表示
    const indentStr = ' '.repeat(indent * 2);
    console.log(`${indentStr}${task.isSubTask ? '└─' : '●'} ${task.content} (ID: ${task.id})`);

    // 期限日があれば表示
    if (task.due && task.due.date) {
      console.log(`${indentStr}  期限日: ${task.due.date}`);
    }

    // サブタスクがあれば再帰的に表示
    if (task.subTasks && task.subTasks.length > 0) {
      displayTaskHierarchy(task.subTasks, indent + 1);
    }
  });
}

/**
 * メイン関数
 */
async function main() {
  try {
    console.log('Todoistからサブタスク情報を取得しています...');

    // TodoistAPIクライアントを作成
    const api = createTodoistApi();

    // サブタスク情報を含むタスクを取得
    const tasksWithSubtasks = await getTasksWithSubtasks(api);

    console.log('\n=== 取得したタスク（階層構造） ===');
    console.log(`合計 ${tasksWithSubtasks.length} 個の親タスクが見つかりました\n`);

    // タスクを階層構造で表示
    displayTaskHierarchy(tasksWithSubtasks);

    // サブタスクの総数をカウント
    let subtaskCount = 0;
    const countSubtasks = (tasks: HierarchicalTask[]) => {
      tasks.forEach(task => {
        if (task.subTasks && task.subTasks.length > 0) {
          subtaskCount += task.subTasks.length;
          countSubtasks(task.subTasks);
        }
      });
    };

    countSubtasks(tasksWithSubtasks);
    console.log(`\n合計 ${subtaskCount} 個のサブタスクが見つかりました`);

  } catch (error) {
    console.error('エラーが発生しました:', error);
  }
}

// スクリプト実行
main();