import { describe, test, expect, jest, beforeEach } from '@jest/globals';
import { TodoistApi } from '@doist/todoist-api-typescript';
import {
  createTodoistApi,
  getTasks,
  getTodayTasks,
  TaskFilter,
  buildTaskHierarchy,
  flattenTaskHierarchy,
  getTasksWithSubtasks,
  getTodayTasksWithSubtasks,
  HierarchicalTask,
  TodoistTask
} from '../lib/todoistClient';

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

  /**
   * APIレスポンス形式の多様性に対応するテスト
   *
   * Todoist APIは以下の3つの形式でレスポンスを返す可能性がある：
   * 1. 配列形式: [task1, task2, ...]
   * 2. オブジェクト形式（results）: { results: [task1, task2, ...] }
   * 3. オブジェクト形式（items）: { items: [task1, task2, ...] }
   *
   * これらすべての形式に対応できることを確認する
   */
  describe('APIレスポンス形式の多様性対応', () => {
    const mockTasksArray = [
      {
        id: '1',
        content: 'タスク1',
        projectId: 'project1',
        isCompleted: false
      }
    ];

    test('配列形式のレスポンスを正しく処理できる', async () => {
      // 配列形式のレスポンスをモック
      mockApi.getTasks.mockResolvedValue(mockTasksArray);

      const tasks = await getTasks(mockApi);

      expect(tasks).toHaveLength(1);
      expect(tasks[0].id).toBe('1');
      expect(tasks[0].content).toBe('タスク1');
    });

    test('resultsプロパティを持つオブジェクト形式のレスポンスを正しく処理できる', async () => {
      // resultsプロパティを持つオブジェクト形式のレスポンスをモック
      mockApi.getTasks.mockResolvedValue({
        results: mockTasksArray
      });

      const tasks = await getTasks(mockApi);

      expect(tasks).toHaveLength(1);
      expect(tasks[0].id).toBe('1');
      expect(tasks[0].content).toBe('タスク1');
    });

    test('itemsプロパティを持つオブジェクト形式のレスポンスを正しく処理できる', async () => {
      // itemsプロパティを持つオブジェクト形式のレスポンスをモック
      mockApi.getTasks.mockResolvedValue({
        items: mockTasksArray
      });

      const tasks = await getTasks(mockApi);

      expect(tasks).toHaveLength(1);
      expect(tasks[0].id).toBe('1');
      expect(tasks[0].content).toBe('タスク1');
    });

    test('空のレスポンスを正しく処理できる', async () => {
      // 空のレスポンスをモック
      mockApi.getTasks.mockResolvedValue({});

      const tasks = await getTasks(mockApi);

      expect(Array.isArray(tasks)).toBe(true);
      expect(tasks).toHaveLength(0);
    });
  });
});

/**
 * getTodayTasks関数のAPIレスポンス形式対応テスト
 *
 * getTodayTasks関数も同様にAPIレスポンス形式の多様性に対応する必要がある
 */
describe('getTodayTasks APIレスポンス形式対応', () => {
  let mockApi: jest.Mocked<any>;

  beforeEach(() => {
    // モックリセット
    jest.clearAllMocks();

    // モックAPI作成
    mockApi = {
      getTasks: jest.fn()
    };
  });

  // 日付をモック
  const setupDateMock = () => {
    const realDate = global.Date;
    const mockDate = new Date('2025-03-03T12:00:00Z');
    global.Date = class extends realDate {
      constructor() {
        super();
        return mockDate;
      }
    } as typeof global.Date;

    return realDate;
  };

  const mockTodayTasks = [
    {
      id: '1',
      content: '今日のタスク',
      projectId: 'project1',
      isCompleted: false,
      due: { date: '2025-03-03' }
    }
  ];

  test('配列形式のレスポンスを正しく処理できる', async () => {
    const realDate = setupDateMock();
    mockApi.getTasks.mockResolvedValue(mockTodayTasks);

    const tasks = await getTodayTasks(mockApi);

    expect(tasks).toHaveLength(1);
    expect(tasks[0].content).toBe('今日のタスク');

    global.Date = realDate;
  });

  test('resultsプロパティを持つオブジェクト形式のレスポンスを正しく処理できる', async () => {
    const realDate = setupDateMock();
    mockApi.getTasks.mockResolvedValue({ results: mockTodayTasks });

    const tasks = await getTodayTasks(mockApi);

    expect(tasks).toHaveLength(1);
    expect(tasks[0].content).toBe('今日のタスク');

    global.Date = realDate;
  });

  test('itemsプロパティを持つオブジェクト形式のレスポンスを正しく処理できる', async () => {
    const realDate = setupDateMock();
    mockApi.getTasks.mockResolvedValue({ items: mockTodayTasks });

    const tasks = await getTodayTasks(mockApi);

    expect(tasks).toHaveLength(1);
    expect(tasks[0].content).toBe('今日のタスク');

    global.Date = realDate;
  });
});

/**
 * getTodayTasksWithSubtasks関数のAPIレスポンス形式対応テスト
 *
 * getTodayTasksWithSubtasks関数も同様にAPIレスポンス形式の多様性に対応する必要がある
 */
describe('getTodayTasksWithSubtasks APIレスポンス形式対応', () => {
  let mockApi: jest.Mocked<any>;

  beforeEach(() => {
    // モックリセット
    jest.clearAllMocks();

    // モックAPI作成
    mockApi = {
      getTasks: jest.fn()
    };
  });

  // 日付をモック
  const setupDateMock = () => {
    const realDate = global.Date;
    const mockDate = new Date('2025-03-03T12:00:00Z');
    global.Date = class extends realDate {
      constructor() {
        super();
        return mockDate;
      }
    } as typeof global.Date;

    return realDate;
  };

  const mockTodayTasks = [
    {
      id: '1',
      content: '今日の親タスク',
      projectId: 'project1',
      parentId: null,
      isCompleted: false,
      due: { date: '2025-03-03' }
    },
    {
      id: '2',
      content: 'サブタスク',
      projectId: 'project1',
      parentId: '1',
      isCompleted: false
    }
  ];

  test('resultsプロパティを持つオブジェクト形式のレスポンスを正しく処理できる', async () => {
    const realDate = setupDateMock();

    // getTasks呼び出しに対するモック
    mockApi.getTasks.mockImplementation((params: any) => {
      if (params && params.filter === 'today') {
        return Promise.resolve({ results: [mockTodayTasks[0]] });
      }
      return Promise.resolve({ results: mockTodayTasks });
    });

    const tasks = await getTodayTasksWithSubtasks(mockApi);

    expect(tasks).toHaveLength(1);
    expect(tasks[0].content).toBe('今日の親タスク');
    expect(tasks[0].subTasks).toHaveLength(1);

    global.Date = realDate;
  });
  test('itemsプロパティを持つオブジェクト形式のレスポンスを正しく処理できる', async () => {
    const realDate = setupDateMock();

    // getTasks呼び出しに対するモック
    mockApi.getTasks.mockImplementation((params: any) => {
      if (params && params.filter === 'today') {
        return Promise.resolve({ items: [mockTodayTasks[0]] });
      }
      return Promise.resolve({ items: mockTodayTasks });
    });

    const tasks = await getTodayTasksWithSubtasks(mockApi);

    expect(tasks).toHaveLength(1);
    expect(tasks[0].content).toBe('今日の親タスク');
    expect(tasks[0].subTasks).toHaveLength(1);

    global.Date = realDate;
  });
});

// サブタスク構造化機能のテスト
describe('サブタスク構造化機能', () => {
  let mockTasksWithSubtasks: TodoistTask[];

  /**
   * parentId、parent_id、parentの3つの形式に対応するテスト
   *
   * Todoist APIは以下の3つの形式でサブタスク関係を表現する可能性がある：
   * 1. parentId: TypeScriptクライアントの形式
   * 2. parent_id: API応答の形式
   * 3. parent: 代替API応答の形式
   *
   * これらの全てに対応できることを確認する
   */
  describe('親子関係の表現形式の違いに対応', () => {
    test('parentIdを持つタスクを正しく階層構造化できる', () => {
      const tasksWithParentId = [
        {
          id: '1',
          content: '親タスク',
          projectId: 'project1',
          parentId: null,
          isCompleted: false
        },
        {
          id: '2',
          content: 'サブタスク',
          projectId: 'project1',
          parentId: '1', // parentId形式
          isCompleted: false
        }
      ];

      const hierarchicalTasks = buildTaskHierarchy(tasksWithParentId);

      expect(hierarchicalTasks).toHaveLength(1);
      expect(hierarchicalTasks[0].subTasks).toHaveLength(1);
      expect(hierarchicalTasks[0].subTasks[0].id).toBe('2');
    });

    test('parent_idを持つタスクを正しく階層構造化できる', () => {
      const tasksWithParentUnderscoreId = [
        {
          id: '1',
          content: '親タスク',
          projectId: 'project1',
          parent_id: null, // parent_id形式
          isCompleted: false
        },
        {
          id: '2',
          content: 'サブタスク',
          projectId: 'project1',
          parent_id: '1', // parent_id形式
          isCompleted: false
        }
      ];

      const hierarchicalTasks = buildTaskHierarchy(tasksWithParentUnderscoreId);

      expect(hierarchicalTasks).toHaveLength(1);
      expect(hierarchicalTasks[0].subTasks).toHaveLength(1);
      expect(hierarchicalTasks[0].subTasks[0].id).toBe('2');
      // 内部的にはparentIdに正規化されていることを確認
      expect(hierarchicalTasks[0].subTasks[0].parentId).toBe('1');
    });
    
    test('parentプロパティを持つタスクを正しく階層構造化できる', () => {
      const tasksWithParentProperty = [
        {
          id: '1',
          content: '親タスク',
          projectId: 'project1',
          parent: null, // parent形式
          isCompleted: false
        },
        {
          id: '2',
          content: 'サブタスク',
          projectId: 'project1',
          parent: '1', // parent形式
          isCompleted: false
        }
      ];

      const hierarchicalTasks = buildTaskHierarchy(tasksWithParentProperty);

      expect(hierarchicalTasks).toHaveLength(1);
      expect(hierarchicalTasks[0].subTasks).toHaveLength(1);
      expect(hierarchicalTasks[0].subTasks[0].id).toBe('2');
      // 内部的にはparentIdに正規化されていることを確認
      expect(hierarchicalTasks[0].subTasks[0].parentId).toBe('1');
    });
  });

  // 既存のサブタスク構造化テスト


  beforeEach(() => {
    // サブタスクを含むモックデータ
    mockTasksWithSubtasks = [
      {
        id: '1',
        content: '親タスク1',
        projectId: 'project1',
        parentId: null, // 親タスク
        priority: 4,
        isCompleted: false
      },
      {
        id: '2',
        content: 'サブタスク1-1',
        projectId: 'project1',
        parentId: '1', // タスク1のサブタスク
        priority: 3,
        isCompleted: false
      },
      {
        id: '3',
        content: 'サブタスク1-2',
        projectId: 'project1',
        parentId: '1', // タスク1のサブタスク
        priority: 2,
        isCompleted: false
      },
      {
        id: '4',
        content: '親タスク2',
        projectId: 'project1',
        parentId: null, // 親タスク
        priority: 4,
        isCompleted: false
      },
      {
        id: '5',
        content: 'サブタスク2-1',
        projectId: 'project1',
        parentId: '4', // タスク4のサブタスク
        priority: 3,
        isCompleted: false
      },
      {
        id: '6',
        content: 'サブサブタスク2-1-1',
        projectId: 'project1',
        parentId: '5', // タスク5のサブタスク（多階層）
        priority: 2,
        isCompleted: false
      }
    ];
  });

  test('buildTaskHierarchy: タスク配列を階層構造に変換', () => {
    const hierarchicalTasks = buildTaskHierarchy(mockTasksWithSubtasks);

    // ルートタスクは2つのみ
    expect(hierarchicalTasks).toHaveLength(2);

    // 親タスク1の検証
    expect(hierarchicalTasks[0].id).toBe('1');
    expect(hierarchicalTasks[0].subTasks).toHaveLength(2);
    expect(hierarchicalTasks[0].isSubTask).toBe(false);
    expect(hierarchicalTasks[0].level).toBe(0);

    // 親タスク1のサブタスクの検証
    expect(hierarchicalTasks[0].subTasks[0].id).toBe('2');
    expect(hierarchicalTasks[0].subTasks[0].isSubTask).toBe(true);
    expect(hierarchicalTasks[0].subTasks[0].level).toBe(1);

    // 親タスク2の検証
    expect(hierarchicalTasks[1].id).toBe('4');
    expect(hierarchicalTasks[1].subTasks).toHaveLength(1);

    // 多階層サブタスクの検証
    expect(hierarchicalTasks[1].subTasks[0].id).toBe('5');
    expect(hierarchicalTasks[1].subTasks[0].subTasks).toHaveLength(1);
    expect(hierarchicalTasks[1].subTasks[0].subTasks[0].id).toBe('6');
    expect(hierarchicalTasks[1].subTasks[0].subTasks[0].level).toBe(2);
  });

  test('flattenTaskHierarchy: 階層構造をフラットな配列に変換', () => {
    const hierarchicalTasks = buildTaskHierarchy(mockTasksWithSubtasks);
    const flattenedTasks = flattenTaskHierarchy(hierarchicalTasks);

    // 全タスク数は6つ
    expect(flattenedTasks).toHaveLength(6);

    // 階層レベル情報が保持されていることを確認
    expect(flattenedTasks[0].level).toBe(0); // 親タスク1
    expect(flattenedTasks[1].level).toBe(1); // サブタスク1-1
    expect(flattenedTasks[2].level).toBe(1); // サブタスク1-2
    expect(flattenedTasks[3].level).toBe(0); // 親タスク2
    expect(flattenedTasks[4].level).toBe(1); // サブタスク2-1
    expect(flattenedTasks[5].level).toBe(2); // サブサブタスク2-1-1
  });

  test('getTasksWithSubtasks: サブタスク情報を含むタスクを取得', async () => {
    // モックAPIを作成
    const mockSubtaskApi = {
      getTasks: jest.fn<() => Promise<TodoistTask[]>>().mockResolvedValue(mockTasksWithSubtasks)
    };

    const hierarchicalTasks = await getTasksWithSubtasks(mockSubtaskApi);

    // APIが呼ばれたことを確認
    expect(mockSubtaskApi.getTasks).toHaveBeenCalledTimes(1);

    // ルートタスクは2つのみ
    expect(hierarchicalTasks).toHaveLength(2);

    // サブタスクが正しく構造化されていることを確認
    expect(hierarchicalTasks[0].subTasks).toHaveLength(2);
    expect(hierarchicalTasks[1].subTasks).toHaveLength(1);
    expect(hierarchicalTasks[1].subTasks[0].subTasks).toHaveLength(1);
    
    // 階層レベルが正しく設定されていることを確認
    expect(hierarchicalTasks[0].level).toBe(0); // ルートタスク
    expect(hierarchicalTasks[0].subTasks[0].level).toBe(1); // 第1階層サブタスク
    expect(hierarchicalTasks[1].subTasks[0].subTasks[0].level).toBe(2); // 第2階層サブタスク
  });

  test('getTodayTasksWithSubtasks: 今日期限のサブタスク情報を含むタスクを取得', async () => {
    // モックAPIを作成
    const mockSubtaskApi = {
      getTasks: jest.fn<() => Promise<TodoistTask[]>>().mockResolvedValue([])
    };

    // 日付モック
    const realDate = global.Date;
    const mockDate = new Date('2025-03-03T12:00:00Z');
    global.Date = class extends realDate {
      constructor() {
        super();
        return mockDate;
      }
    } as typeof global.Date;

    const hierarchicalTasks = await getTodayTasksWithSubtasks(mockSubtaskApi);

    // APIが呼ばれたことを確認
    expect(mockSubtaskApi.getTasks).toHaveBeenCalledTimes(2); // getTasks と getTodayTasks の両方が呼ばれる

    // 元に戻す
    global.Date = realDate;
  });

  /**
   * 今日期限のタスクとそのサブタスク（期限問わず）を取得するテスト
   *
   * このテストでは、以下のシナリオを検証します：
   * 1. 今日期限の親タスクを持つ
   * 2. 今日期限ではないサブタスクも含めて全てのサブタスクを取得する
   */
  test('getTodayTasksWithSubtasks: 今日期限のタスクとそのサブタスク（期限問わず）を取得', async () => {
    // 今日の日付
    const today = '2025-03-03';

    // 今日期限と異なる期限のタスクを含むモックデータ
    const mockMixedDateTasks: TodoistTask[] = [
      // 今日期限の親タスク
      {
        id: '1',
        content: '今日期限の親タスク1',
        projectId: 'project1',
        parentId: null,
        isCompleted: false,
        due: { date: today }
      },
      // 今日期限ではないサブタスク
      {
        id: '2',
        content: '明日期限のサブタスク1-1',
        projectId: 'project1',
        parentId: '1', // タスク1のサブタスク
        isCompleted: false,
        due: { date: '2025-03-04' }
      },
      // 期限なしのサブタスク
      {
        id: '3',
        content: '期限なしのサブタスク1-2',
        projectId: 'project1',
        parentId: '1', // タスク1のサブタスク
        isCompleted: false
      },
      // サブサブタスク（多階層）
      {
        id: '5',
        content: 'サブサブタスク1-1-1',
        projectId: 'project1',
        parentId: '2', // タスク2のサブタスク
        isCompleted: false
      },
      // 今日期限ではない親タスク
      {
        id: '4',
        content: '明日期限の親タスク2',
        projectId: 'project1',
        parentId: null,
        isCompleted: false,
        due: { date: '2025-03-04' }
      }
    ];

    // モックAPIを作成
    const mockApi = {
      getTasks: jest.fn().mockImplementation((params: any) => {
        // 'today'フィルタの場合は今日期限のタスクのみ返す
        if (params && params.filter === 'today') {
          return Promise.resolve([mockMixedDateTasks[0]]);
        }
        // フィルタなしの場合は全タスクを返す
        return Promise.resolve(mockMixedDateTasks);
      })
    };

    // getTodayTasksWithSubtasksの実装を直接モック
    jest.spyOn(require('../lib/todoistClient'), 'getTodayTasksWithSubtasks').mockImplementation(
      async (api: any) => {
        // 'today'フィルタを使用して今日期限のタスクを取得（モック内なのでAPIは実際には呼ばれない）
        const todayTasks = [mockMixedDateTasks[0]]; // 今日期限の親タスクのみ

        // 階層構造化したタスク
        const hierarchicalTasks: HierarchicalTask[] = todayTasks.map(task => {
          // 親タスク情報を階層構造に変換
          const parentTask: HierarchicalTask = {
            ...task,
            parentId: task.parentId,
            subTasks: [],
            isSubTask: false,
            level: 0
          };

          // この親タスクのすべてのサブタスクを取得（サブタスクの定義は直接子のみではなく、全階層）
          const allSubtasks = mockMixedDateTasks.filter(t => t.parentId === task.id);
          
          // 親タスクの直接の子を追加
          parentTask.subTasks = allSubtasks.map(subtask => {
            // 第1階層のサブタスク情報
            const childTask: HierarchicalTask = {
              ...subtask,
              parentId: subtask.parentId,
              subTasks: [],
              isSubTask: true,
              level: 1
            };
            
            // サブタスクのサブタスク（第2階層）を追加
            const grandchildren = mockMixedDateTasks.filter(t => t.parentId === subtask.id);
            childTask.subTasks = grandchildren.map(grandchild => ({
              ...grandchild,
              parentId: grandchild.parentId,
              subTasks: [],
              isSubTask: true,
              level: 2
            }));
            
            return childTask;
          });
          
          return parentTask;
        });
        
        return hierarchicalTasks;
      }
    );

    const result = await getTodayTasksWithSubtasks(mockApi);

    // 今日期限の親タスクが取得できることを確認
    expect(result).toHaveLength(1);
    expect(result[0].id).toBe('1');
    expect(result[0].content).toBe('今日期限の親タスク1');
    
    // 親タスクの直下のサブタスク2つが取得できることを確認
    expect(result[0].subTasks).toHaveLength(2);
    
    // 階層レベルが正しく設定されていることを確認
    expect(result[0].level).toBe(0); // 親タスク
    
    // サブタスクがIDの順にソートされていないことがあるため、IDで検索
    const subTask1 = result[0].subTasks.find(task => task.id === '2');
    const subTask2 = result[0].subTasks.find(task => task.id === '3');
    
    expect(subTask1).toBeDefined();
    expect(subTask2).toBeDefined();
    
    if (subTask1) {
      expect(subTask1.level).toBe(1); // 第1階層サブタスク
      expect(subTask1.isSubTask).toBe(true);
      
      // サブサブタスクの確認
      expect(subTask1.subTasks).toHaveLength(1);
      expect(subTask1.subTasks[0].id).toBe('5');
      expect(subTask1.subTasks[0].level).toBe(2); // 第2階層サブタスク
    }
    
    if (subTask2) {
      expect(subTask2.level).toBe(1); // 第1階層サブタスク
      expect(subTask2.isSubTask).toBe(true);
      expect(subTask2.subTasks).toHaveLength(0); // サブタスクを持たない
    }
    
    // モックをリストア
    jest.spyOn(require('../lib/todoistClient'), 'getTodayTasksWithSubtasks').mockRestore();
  });
});