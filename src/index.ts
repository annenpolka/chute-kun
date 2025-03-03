#!/usr/bin/env node
import dotenv from 'dotenv';
import * as path from 'path';
import {
  displayTasks,
  displayTasksHierarchy,
  FILTER_RESULT_FORMAT,
  HIERARCHY_FORMAT,
  TODAY_TASKS_FORMAT
} from './lib/formatters/taskFormatter';
import {
  buildTaskHierarchy,
  createTodoistApi,
  debug,
  getTasks,
  getTodayTasks,
  getTodayTasksWithSubtasks,
  HierarchicalTask,
  setDebugMode,
  TaskFilter
} from './lib/todoistClient';

// 環境変数を読み込む
dotenv.config({ path: path.resolve(process.cwd(), '.env') });

/**
 * コマンドライン引数を解析
 * @returns 解析結果
 */
function parseArgs(): { command: string; options: Record<string, string> } {
  const args = process.argv.slice(2);
  const command = args[0]?.startsWith('--') ? 'today' : args[0] || 'today';
  const options: Record<string, string> = {};

  for (let i = command === args[0] ? 1 : 0; i < args.length; i++) {
    const arg = args[i];
    if (arg.startsWith('--')) {
      const key = arg.slice(2);
      const value = args[i + 1] && !args[i + 1].startsWith('--') ? args[i + 1] : 'true';
      options[key] = value;
      if (value !== 'true') {
        i++;
      }
    }
  }

  return { command, options };
}

/**
 * メイン実行関数
 */
async function main() {
  try {
    const { command, options } = parseArgs();

    // デバッグモードの設定
    const isDebugMode = options.debug === 'true' || process.env.DEBUG_MODE === 'true';
    setDebugMode(isDebugMode);

    // アプリケーション起動メッセージ
    if (isDebugMode) {
      console.log('デバッグモード: 有効');

      // 環境変数のデバッグ情報
      debug('環境変数の状態:');
      debug('  プロセスID:', process.pid);
      debug('  環境:', process.env.APP_ENV || '未設定');
      debug('  カレントディレクトリ:', process.cwd());
      debug('  環境変数ファイルパス:', path.resolve(process.cwd(), '.env'));
      debug('  TODOIST_API_TOKEN が設定されているか:', process.env.TODOIST_API_TOKEN ? 'はい' : 'いいえ');
    }

    // コマンドラインからのAPIトークンを優先
    const apiToken = options.token || process.env.TODOIST_API_TOKEN;

    // Todoist APIクライアントを作成
    const api = createTodoistApi(apiToken);

    console.log('Todoistに接続中...');
    debug('APIトークン取得元:', options.token ? 'コマンドラインオプション' : '環境変数');

    switch (command) {
      case 'today':
        await showTodayTasks(api, options);
        break;
      case 'filter':
        await filterTasks(api, options);
        break;
      case 'help':
      default:
        showHelp();
        break;
    }

  } catch (error) {
    console.error('エラーが発生しました:', error);
    process.exit(1);
  }
}

/**
 * 今日のタスクを表示
 */
async function showTodayTasks(api: any, options: Record<string, string> = {}) {
  console.log('今日のタスクを取得中...');

  // 階層構造のサブタスクも含めて表示するかどうか
  const includeSubtasks = options['include-subtasks'] === 'true';
  const isDebugMode = options.debug === 'true' || process.env.DEBUG_MODE === 'true';
  let todayTasks;

  // デバッグ用に全タスクを取得して状況を確認
  if (isDebugMode) {
    const allTasks = await getTasks(api);
    console.log(`デバッグ: 全タスク数: ${allTasks.length}`);

    // 今日の日付を取得
    const today = new Date().toISOString().split('T')[0];
    console.log(`デバッグ: 今日の日付: ${today}`);

    // タスクの期限日を確認
    const tasksWithDueDate = allTasks.filter(task =>
      typeof task.due === 'object' && task.due !== null && typeof task.due.date === 'string'
    );
    console.log(`デバッグ: 期限日が設定されているタスク数: ${tasksWithDueDate.length}`);

    // 期限日を表示（最大5件）
    console.log('デバッグ: 期限日のサンプル（最大5件）:');
    tasksWithDueDate.slice(0, 5).forEach(task => {
      console.log(`  - "${task.content}" の期限日: ${task.due!.date}`);
    });
  }

  if (includeSubtasks) {
    // サブタスクを含めた階層構造で取得
    todayTasks = await getTodayTasksWithSubtasks(api);
    debug('取得した階層タスク数:', todayTasks.length);

    console.log(`今日のタスクとサブタスク:`);
    // 階層構造で表示
    displayTasksHierarchy(todayTasks, console.log, {
      ...TODAY_TASKS_FORMAT,
      ...HIERARCHY_FORMAT
    });
  } else {
    // 通常の今日のタスクを取得
    todayTasks = await getTodayTasks(api);
    debug('取得したタスク数:', todayTasks.length);

    if (todayTasks.length === 0) {
      console.log('今日期限のタスクはありません。');
      return;
    }

    console.log(`今日のタスク (${todayTasks.length}件):`);
    // 通常のフラット表示
    displayTasks(todayTasks, console.log, TODAY_TASKS_FORMAT);
  }
}

/**
 * 条件でタスクを絞り込んで表示
 */
async function filterTasks(api: any, options: Record<string, string>) {
  console.log('タスクをフィルタリング中...');

  const filter: TaskFilter = {};

  // オプションから検索条件を構築
  if (options.project) {
    filter.projectId = options.project;
  }
  if (options.labels) {
    filter.labelIds = options.labels.split(',');
  }
  if (options.due) {
    filter.dueDate = options.due;
  }
  if (options.completed !== undefined) {
    filter.isCompleted = options.completed === 'true';
  }
  if (options.priority) {
    filter.priority = parseInt(options.priority, 10);
  }

  // サブタスク関連のフィルタリングオプション
  const hasSubtasksFilter = options['has-subtasks'] === 'true';
  const isSubtaskFilter = options['is-subtask'] === 'true';
  const showHierarchy = options['show-hierarchy'] !== 'false'; // デフォルトで階層表示

  debug('検索条件:', filter);

  // タスクを取得
  const tasks = await getTasks(api, filter);
  debug('取得したタスク数:', tasks.length);

  // タスクを階層構造に変換
  const hierarchicalTasks = buildTaskHierarchy(tasks);

  // サブタスク関連のフィルタリング
  let filteredTasks = hierarchicalTasks;

  if (hasSubtasksFilter) {
    // サブタスクを持つタスクのみ表示
    filteredTasks = hierarchicalTasks.filter(task => task.subTasks && task.subTasks.length > 0);
  }

  if (isSubtaskFilter) {
    // サブタスクのみを抽出して表示
    const allSubtasks: HierarchicalTask[] = [];
    const extractSubtasks = (tasks: HierarchicalTask[]) => {
      tasks.forEach((task: HierarchicalTask) => {
        if (task.subTasks && task.subTasks.length > 0) {
          allSubtasks.push(...task.subTasks);
          extractSubtasks(task.subTasks);
        }
      });
    };

    extractSubtasks(hierarchicalTasks);
    filteredTasks = allSubtasks;
  }

  console.log(`検索結果 (${filteredTasks.length}件のルートタスク):`);

  // 表示方法を決定
  if (showHierarchy) {
    // 階層構造で表示
    displayTasksHierarchy(filteredTasks, console.log, {
      ...FILTER_RESULT_FORMAT,
      ...HIERARCHY_FORMAT
    });
  } else {
    // フラット表示
    displayTasks(filteredTasks, console.log, {
      ...FILTER_RESULT_FORMAT,
      flattenHierarchy: true
    });
  }
}

/**
 * ヘルプメッセージを表示
 */
function showHelp() {
  console.log(`
使い方: npx ts-node src/index.ts [コマンド] [オプション]

コマンド:
  today           今日期限の未完了タスクを表示 (デフォルト)
  filter          条件に合わせてタスクをフィルタリング
  help            このヘルプを表示

オプション (共通):
  --debug         デバッグ情報を表示 (true/false)
  --token         Todoist API トークンを直接指定

オプション (todayコマンド用):
  --include-subtasks  サブタスクも含めて表示 (true/false)

オプション (filterコマンド用):
  --project       プロジェクトIDで絞り込み
  --labels        ラベルで絞り込み (カンマ区切りで複数指定可)
  --due           期限日で絞り込み (YYYY-MM-DD形式)
  --completed     完了状態で絞り込み (true/false)
  --priority      優先度で絞り込み (1-4)
  --has-subtasks  サブタスクを持つタスクのみ表示 (true/false)
  --is-subtask    サブタスクのみ表示 (true/false)
  --show-hierarchy 階層構造で表示 (true/false、デフォルト: true)

環境変数:
  TODOIST_API_TOKEN  TodoistのAPIトークン
  DEBUG_MODE         デバッグモードを有効化 (true/false)

例:
  npx ts-node src/index.ts today
  npx ts-node src/index.ts today --include-subtasks true
  npx ts-node src/index.ts today --debug true
  npx ts-node src/index.ts filter --project project123 --due 2025-03-10
  npx ts-node src/index.ts filter --labels label1,label2 --completed false --show-hierarchy true
  `);
}

// スクリプト実行
if (require.main === module) {
  main();
}

export { main };
