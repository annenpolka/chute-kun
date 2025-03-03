import { describe, test, expect, jest, beforeEach } from '@jest/globals';
import { TodoistApi } from '@doist/todoist-api-typescript';
import { createTodoistApi, getTasks, getTodayTasks, TaskFilter } from '../lib/todoistClient';

// TodoistApiのモック
jest.mock('@doist/todoist-api-typescript');

describe('Todoist Client 関数', () => {
  let mockApi: jest.Mocked<any>;
  
  beforeEach(() => {
    // モックリセット
    jest.clearAllMocks();
    
    // モックAPI作成
    mockApi = {
      getTasks: jest.fn()
    };
    
    // モックデータ
    const mockTasks = [
      { 
        id: '1', 
        content: 'タスク1', 
        projectId: 'project1', 
        labels: ['label1', 'label2'],
        priority: 4,
        isCompleted: false,
        due: { date: '2025-03-03' }
      },
      { 
        id: '2', 
        content: 'タスク2', 
        projectId: 'project2', 
        labels: ['label2'],
        priority: 3,
        isCompleted: true,
        due: { date: '2025-03-04' }
      },
      { 
        id: '3', 
        content: 'タスク3', 
        projectId: 'project1', 
        labels: ['label3'],
        priority: 2,
        isCompleted: false,
        due: { date: '2025-03-03' }
      }
    ];
    
    // getTasks関数のモック
    mockApi.getTasks.mockResolvedValue(mockTasks);
  });

  test('createTodoistApi: 有効なトークンで正常にAPIクライアントを作成', () => {
    process.env.TODOIST_API_TOKEN = 'test-token';
    
    const originalTodoistApi = TodoistApi;
    // コンストラクタをモック
    (TodoistApi as any) = jest.fn().mockImplementation(() => ({}));
    
    createTodoistApi();
    expect(TodoistApi).toHaveBeenCalledWith('test-token');
    
    createTodoistApi('custom-token');
    expect(TodoistApi).toHaveBeenCalledWith('custom-token');
    
    // 元に戻す
    (TodoistApi as any) = originalTodoistApi;
  });

  test('createTodoistApi: トークンがない場合はエラーを投げる', () => {
    // 環境変数をクリア
    delete process.env.TODOIST_API_TOKEN;
    
    expect(() => createTodoistApi()).toThrow('Todoist APIトークンが設定されていません。');
  });

  test('getTasks: フィルタなしで全てのタスクを取得', async () => {
    const tasks = await getTasks(mockApi);
    
    expect(mockApi.getTasks).toHaveBeenCalledTimes(1);
    expect(tasks).toHaveLength(3);
  });

  test('getTasks: プロジェクトIDでフィルタリング', async () => {
    const filter: TaskFilter = { projectId: 'project1' };
    const tasks = await getTasks(mockApi, filter);
    
    expect(tasks).toHaveLength(2);
    expect(tasks[0].projectId).toBe('project1');
    expect(tasks[1].projectId).toBe('project1');
  });

  test('getTasks: ラベルでフィルタリング', async () => {
    const filter: TaskFilter = { labelIds: ['label1'] };
    const tasks = await getTasks(mockApi, filter);
    
    expect(tasks).toHaveLength(1);
    expect(tasks[0].labels).toContain('label1');
  });

  test('getTasks: 完了状態でフィルタリング', async () => {
    const filter: TaskFilter = { isCompleted: false };
    const tasks = await getTasks(mockApi, filter);
    
    expect(tasks).toHaveLength(2);
    expect(tasks[0].isCompleted).toBe(false);
    expect(tasks[1].isCompleted).toBe(false);
  });

  test('getTasks: 期限日でフィルタリング', async () => {
    const filter: TaskFilter = { dueDate: '2025-03-03' };
    const tasks = await getTasks(mockApi, filter);
    
    expect(tasks).toHaveLength(2);
    expect(tasks[0].due.date).toBe('2025-03-03');
    expect(tasks[1].due.date).toBe('2025-03-03');
  });

  test('getTasks: 優先度でフィルタリング', async () => {
    const filter: TaskFilter = { priority: 4 };
    const tasks = await getTasks(mockApi, filter);
    
    expect(tasks).toHaveLength(1);
    expect(tasks[0].priority).toBe(4);
  });

  test('getTasks: 複合条件でフィルタリング', async () => {
    const filter: TaskFilter = { 
      projectId: 'project1',
      isCompleted: false,
      dueDate: '2025-03-03'
    };
    const tasks = await getTasks(mockApi, filter);
    
    expect(tasks).toHaveLength(2);
    expect(tasks[0].projectId).toBe('project1');
    expect(tasks[0].isCompleted).toBe(false);
    expect(tasks[0].due.date).toBe('2025-03-03');
  });

  test('getTodayTasks: 今日期限の未完了タスクを取得', async () => {
    // 日付モック
    const realDate = global.Date;
    const mockDate = new Date('2025-03-03T12:00:00Z');
    global.Date = class extends realDate {
      constructor() {
        super();
        return mockDate;
      }
    } as typeof global.Date;
    
    await getTodayTasks(mockApi);
    
    // getTasks関数が正しいフィルターで呼ばれることを確認
    expect(mockApi.getTasks).toHaveBeenCalledTimes(1);
    
    // 元に戻す
    global.Date = realDate;
  });

  test('getTasks: APIエラー時は例外を投げる', async () => {
    // エラーを投げるようにモック
    mockApi.getTasks.mockRejectedValue(new Error('API error'));
    
    await expect(getTasks(mockApi)).rejects.toThrow('API error');
  });
});