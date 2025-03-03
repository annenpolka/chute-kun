import { describe, test, expect, jest } from '@jest/globals';
import { TodoistTask } from '../../lib/todoistClient';
import { formatTask, FormatTaskOptions, displayTasks } from '../../lib/formatters/taskFormatter';

describe('タスクフォーマット関数', () => {
  // テスト用のサンプルタスク
  const sampleTask: TodoistTask = {
    id: '1',
    content: 'サンプルタスク',
    projectId: 'project1',
    parentId: null,
    isCompleted: false,
    priority: 3,
    due: { date: '2025-03-03' }
  };

  const completedTask: TodoistTask = {
    id: '2',
    content: '完了済みタスク',
    projectId: 'project1',
    parentId: null,
    isCompleted: true,
    priority: 1,
    due: { date: '2025-03-02' }
  };

  const taskWithoutDue: TodoistTask = {
    id: '3',
    content: '期限なしタスク',
    projectId: 'project1',
    parentId: null,
    isCompleted: false,
    priority: 4,
  };

  const taskWithoutPriority: TodoistTask = {
    id: '4',
    content: '優先度なしタスク',
    projectId: 'project1',
    parentId: null,
    isCompleted: false,
    due: { date: '2025-03-04' }
  };

  test('デフォルトオプションでのフォーマット', () => {
    // 未完了タスク（優先度・期限あり）
    expect(formatTask(sampleTask, 0)).toBe('1. □ [!!!] サンプルタスク [期限: 2025-03-03]');
    
    // 完了済みタスク
    expect(formatTask(completedTask, 1)).toBe('2. ✓ [!] 完了済みタスク [期限: 2025-03-02]');
    
    // 期限なしタスク
    expect(formatTask(taskWithoutDue, 2)).toBe('3. □ [!!!!] 期限なしタスク');
    
    // 優先度なしタスク
    expect(formatTask(taskWithoutPriority, 3)).toBe('4. □ [-] 優先度なしタスク [期限: 2025-03-04]');
  });

  test('カスタムオプションでのフォーマット', () => {
    const options: FormatTaskOptions = {
      showStatus: false,
      prioritySymbol: '*',
      dueDateBrackets: ['(', ')'],
      dueDateFormat: 'due: %s'
    };
    
    expect(formatTask(sampleTask, 0, options)).toBe('1. [***] サンプルタスク (due: 2025-03-03)');
    
    // 完了状態を表示しないオプション
    expect(formatTask(completedTask, 1, options)).toBe('2. [*] 完了済みタスク (due: 2025-03-02)');
  });

  test('インデックスなしでのフォーマット', () => {
    const options: FormatTaskOptions = {
      showIndex: false
    };
    
    expect(formatTask(sampleTask, 0, options)).toBe('□ [!!!] サンプルタスク [期限: 2025-03-03]');
  });

  test('最小限の表示オプション', () => {
    const options: FormatTaskOptions = {
      showStatus: false,
      showPriority: false,
      showDueDate: false,
      showIndex: false
    };
    
    expect(formatTask(sampleTask, 0, options)).toBe('サンプルタスク');
  });

  test('displayTasks関数のテスト', () => {
    // モック出力関数
    const mockLogger = jest.fn();
    const tasks = [sampleTask, completedTask, taskWithoutDue];
    
    displayTasks(tasks, mockLogger);
    
    // 各タスクごとに1回ずつ呼ばれることを確認
    expect(mockLogger).toHaveBeenCalledTimes(3);
    expect(mockLogger).toHaveBeenNthCalledWith(1, '1. □ [!!!] サンプルタスク [期限: 2025-03-03]');
    expect(mockLogger).toHaveBeenNthCalledWith(2, '2. ✓ [!] 完了済みタスク [期限: 2025-03-02]');
    expect(mockLogger).toHaveBeenNthCalledWith(3, '3. □ [!!!!] 期限なしタスク');
  });
});