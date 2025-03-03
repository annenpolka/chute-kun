import { TodoistTask } from '../todoistClient';

/**
 * タスク表示フォーマットの設定オプション
 */
export interface FormatTaskOptions {
  showStatus?: boolean;         // 完了状態を表示するか
  showPriority?: boolean;       // 優先度を表示するか
  showDueDate?: boolean;        // 期限を表示するか
  showIndex?: boolean;          // インデックス番号を表示するか
  indexOffset?: number;         // インデックス番号のオフセット（デフォルト: 1から開始）
  prioritySymbol?: string;      // 優先度を表すシンボル（デフォルト: "!"）
  completedSymbol?: string;     // 完了タスクのシンボル（デフォルト: "✓"）
  uncompletedSymbol?: string;   // 未完了タスクのシンボル（デフォルト: "□"）
  noPriorityPlaceholder?: string; // 優先度なしの表示（デフォルト: "-"）
  dueDateFormat?: string;       // 期限表示のフォーマット（デフォルト: "期限: %s"）
  dueDateBrackets?: [string, string]; // 期限表示の括弧（デフォルト: ["[", "]"]）
}

/**
 * デフォルトのフォーマットオプション
 */
const DEFAULT_OPTIONS: Required<FormatTaskOptions> = {
  showStatus: true,
  showPriority: true,
  showDueDate: true,
  showIndex: true,
  indexOffset: 1,
  prioritySymbol: "!",
  completedSymbol: "✓",
  uncompletedSymbol: "□",
  noPriorityPlaceholder: "-",
  dueDateFormat: "期限: %s",
  dueDateBrackets: ["[", "]"]
};

/**
 * タスクを文字列形式にフォーマット
 * @param task - フォーマットするタスク
 * @param index - タスクのインデックス
 * @param customOptions - カスタムオプション
 * @returns フォーマットされたタスク文字列
 */
export function formatTask(
  task: TodoistTask, 
  index: number, 
  customOptions: Partial<FormatTaskOptions> = {}
): string {
  // デフォルトオプションとカスタムオプションをマージ
  const options: Required<FormatTaskOptions> = {
    ...DEFAULT_OPTIONS,
    ...customOptions
  };
  
  const parts: string[] = [];
  
  // インデックス表示
  if (options.showIndex) {
    parts.push(`${index + options.indexOffset}.`);
  }
  
  // 完了状態表示
  if (options.showStatus) {
    const statusMarker = task.isCompleted ? 
      options.completedSymbol : 
      options.uncompletedSymbol;
    parts.push(statusMarker);
  }
  
  // 優先度表示
  if (options.showPriority) {
    const priorityStr = task.priority ? 
      options.prioritySymbol.repeat(task.priority) : 
      options.noPriorityPlaceholder;
    parts.push(`[${priorityStr}]`);
  }
  
  // タスク内容
  parts.push(task.content);
  
  // 期限表示
  if (options.showDueDate && task.due) {
    const dueDateStr = options.dueDateFormat.replace('%s', task.due.date);
    parts.push(`${options.dueDateBrackets[0]}${dueDateStr}${options.dueDateBrackets[1]}`);
  }
  
  return parts.join(' ');
}

/**
 * タスク配列を表示する
 * @param tasks - 表示するタスク配列
 * @param logger - 出力関数（デフォルト: console.log）
 * @param options - フォーマットオプション
 */
export function displayTasks(
  tasks: TodoistTask[], 
  logger: (message: string) => void = console.log,
  options: Partial<FormatTaskOptions> = {}
): void {
  tasks.forEach((task, index) => {
    logger(formatTask(task, index, options));
  });
}

/**
 * フィルタリング結果用のカスタムフォーマットオプション
 */
export const FILTER_RESULT_FORMAT: Partial<FormatTaskOptions> = {
  dueDateBrackets: ["(", ")"],
};

/**
 * 今日のタスク表示用のカスタムフォーマットオプション
 */
export const TODAY_TASKS_FORMAT: Partial<FormatTaskOptions> = {
  // デフォルトと同じ（上書きする場合はここに追加）
};

/**
 * コンパクト表示用のフォーマットオプション
 */
export const COMPACT_FORMAT: Partial<FormatTaskOptions> = {
  showStatus: false,
  dueDateFormat: "due: %s"
};