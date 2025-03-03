#!/usr/bin/env node
import * as path from 'path';
import dotenv from 'dotenv';
import {
  createTodoistApi,
  getTodayTasks,
  getTasks,
  TaskFilter,
  setDebugMode,
  debug } from './lib/todoistClient';
import {
  formatTask,
  displayTasks,
  TODAY_TASKS_FORMAT,
  FILTER_RESULT_FORMAT,
  FormatTaskOptions
} from './lib/formatters/taskFormatter';

// 環境変数を読み込む
dotenv.config({ path: path.resolve(process.cwd(), '.env') });

/**
 * コマンドライン引数を解析
 * @returns 解析結果
 */
function parseArgs(): { command: string; options: Record<string, string> } {
  const args = process.argv.slice(2);
  let command = 'today'; // デフォルトコマンド
  const options: Record<string, string> = {};

  if (args.length > 0 && !args[0].startsWith('--')) {
    command = args[0];
  }

  // オプションの解析
  for (let i = 0; i < args.length; i++) {
    if (args[i].startsWith('--')) {
      const option = args[i].substring(2);
      if (i + 1 < args.length && !args[i + 1].startsWith('--')) {
        options[option] = args[i + 1];
        i++;
      } else {
        options[option] = 'true';
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

  // 今日のタスクを取得
  const todayTasks = await getTodayTasks(api);
  debug('取得したタスク数:', todayTasks.length);

  console.log(`今日のタスク (${todayTasks.length}件):`);
  
  // 今日のタスク表示用フォーマットを使用
  displayTasks(todayTasks, console.log, TODAY_TASKS_FORMAT);
}

// displayTasks関数は formatters/taskFormatter.ts に移動しました

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
    filter.priority = parseInt(options.priority);
  }

  // フィルタ条件を表示
  debug('使用されたオプション:', options);
  console.log('検索条件:', filter);

  // タスクを取得
  const tasks = await getTasks(api, filter);
  debug('取得したタスク数:', tasks.length);
  
  if (tasks.length > 0) {
    debug('最初のタスク例:', {
      id: tasks[0].id,
      content: tasks[0].content,
      projectId: tasks[0].projectId,
      due: tasks[0].due
    });
  }

  console.log(`検索結果 (${tasks.length}件):`);

  // フィルタリング結果用のフォーマットを使用
  displayTasks(tasks, console.log, FILTER_RESULT_FORMAT);
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

オプション (filterコマンド用):
  --project       プロジェクトIDで絞り込み
  --labels        ラベルで絞り込み (カンマ区切りで複数指定可)
  --due           期限日で絞り込み (YYYY-MM-DD形式)
  --completed     完了状態で絞り込み (true/false)
  --priority      優先度で絞り込み (1-4)

環境変数:
  TODOIST_API_TOKEN  TodoistのAPIトークン
  DEBUG_MODE         デバッグモードを有効化 (true/false)

例:
  npx ts-node src/index.ts today
  npx ts-node src/index.ts today --debug true
  npx ts-node src/index.ts filter --project project123 --due 2025-03-10
  npx ts-node src/index.ts filter --labels label1,label2 --completed false
  `);
}

// スクリプト実行
if (require.main === module) {
  main();
}

export { main };