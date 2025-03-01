# タスクマネージャーモジュール

*最終更新日: 2025-03-01*

## 責務
タスクマネージャーモジュールは、TaskChuteシステムの中核として、タスクの作成、更新、実行、完了処理、およびログ記録を担当します。Todoistとの連携を通じてタスクデータの永続化を行い、タスクシュート独自の機能をTodoistの基本機能に付加する役割を果たします。

## インターフェース

### 公開API
```typescript
interface TaskManager {
  // タスク管理基本操作
  createTask(taskData: TaskData): Promise<Task>;
  updateTask(taskId: string, taskData: Partial<TaskData>): Promise<Task>;
  deleteTask(taskId: string): Promise<void>;
  getTask(taskId: string): Promise<Task>;
  getTasks(filters?: TaskFilters): Promise<Task[]>;
  
  // タスクシュート固有機能
  startTask(taskId: string): Promise<TaskLog>;
  pauseTask(taskId: string): Promise<TaskLog>;
  resumeTask(taskId: string): Promise<TaskLog>;
  completeTask(taskId: string): Promise<Task>;
  
  // プラン機能
  createDailyPlan(date: Date, tasks: TaskData[]): Promise<Plan>;
  getDailyPlan(date: Date): Promise<Plan>;
  reorderPlanTasks(planId: string, taskIds: string[]): Promise<Plan>;
  
  // ルーチン機能
  createRoutine(routineData: RoutineData): Promise<Routine>;
  updateRoutine(routineId: string, routineData: Partial<RoutineData>): Promise<Routine>;
  getRoutines(): Promise<Routine[]>;
  generateRoutineTasks(date: Date): Promise<Task[]>;
}
```

### イベント
```typescript
// 発行するイベント
interface TaskEvents {
  'task:created': (task: Task) => void;
  'task:updated': (task: Task) => void;
  'task:deleted': (taskId: string) => void;
  'task:started': (taskLog: TaskLog) => void;
  'task:paused': (taskLog: TaskLog) => void;
  'task:resumed': (taskLog: TaskLog) => void;
  'task:completed': (task: Task) => void;
  'plan:created': (plan: Plan) => void;
  'plan:updated': (plan: Plan) => void;
  'routine:generated': (tasks: Task[]) => void;
}
```

### データ型定義
```typescript
interface TaskData {
  title: string;
  projectId: string;
  sectionId?: string;
  estimate?: number; // 見積時間（分）
  description?: string;
  priority?: 1 | 2 | 3 | 4; // 4が最高優先度
  dueDate?: string;
  labels?: string[];
}

interface Task extends TaskData {
  id: string;
  createdAt: string;
  actual?: number; // 実績時間（分）
  status: 'pending' | 'in_progress' | 'paused' | 'completed';
  logs?: TaskLog[];
}

interface TaskLog {
  id: string;
  taskId: string;
  action: 'start' | 'pause' | 'resume' | 'complete';
  timestamp: string;
  note?: string;
}

interface Plan {
  id: string;
  date: string;
  taskIds: string[]; // 実行順序を示す配列
  createdAt: string;
  updatedAt: string;
}

interface RoutineData {
  title: string;
  projectId: string;
  sectionId?: string;
  frequency: 'daily' | 'weekly' | 'monthly' | 'weekdays' | 'weekends';
  days?: number[]; // 曜日 (0-6) または 日付 (1-31)
  estimate?: number;
  priority?: 1 | 2 | 3 | 4;
  labels?: string[];
}

interface Routine extends RoutineData {
  id: string;
  createdAt: string;
  lastGenerated?: string;
}

interface TaskFilters {
  projectId?: string;
  sectionId?: string;
  status?: string;
  dueDate?: string;
  priority?: number;
  labels?: string[];
}
```

## 依存関係
- **Todoist API Adapter**: Todoist REST API v2との通信を担当
- **ユーザーモジュール**: ユーザー認証と権限確認
- **データベース**: ルーチン設定や実績データの保存
- **分析エンジン**: タスク実績データの提供

## 技術的制約
1. Todoistの構造制約:
   - 4段階の優先度（TaskChuteと一致）
   - プロジェクト/セクションの階層構造
   - セクション移動 = タスク状態変更の対応付け
   - タスク説明欄をログ記録に使用

2. パフォーマンス:
   - 日次プラン（〜50タスク）の読み込み: 500ms以内
   - タスク状態変更操作: 300ms以内

3. 同期:
   - オフライン操作のサポート
   - 再接続時の競合解決戦略

## 実装詳細

### Todoistとの概念マッピング
1. **プロジェクト**:
   - Todoistプロジェクト = TaskChuteプロジェクト

2. **セクション**:
   - Todoistセクション = TaskChuteモード/セクション
   - 特殊セクション:
     - `_pending`: 未開始タスク
     - `_in_progress`: 実行中タスク
     - `_paused`: 一時停止タスク
     - `_completed`: 完了タスク

3. **タスク**:
   - Todoistタスク = TaskChuteタスク
   - 特殊フィールド活用:
     - 説明欄: JSON形式でログデータ保存
     - ラベル: カテゴリおよび状態フラグ

### タスク実行ログ記録アルゴリズム
1. タスク開始時:
   - 開始時刻を記録
   - タスクを`_in_progress`セクションに移動
   - 説明欄にログエントリを追加

2. タスク一時停止時:
   - 一時停止時刻を記録
   - 実行時間を計算して累積
   - タスクを`_paused`セクションに移動

3. タスク再開時:
   - 再開時刻を記録
   - タスクを`_in_progress`セクションに戻す

4. タスク完了時:
   - 完了時刻を記録
   - 最終実行時間を計算して累積
   - タスクを`_completed`セクションに移動
   - 実績vs見積の比較データを保存

### ルーチンタスク生成処理
1. 日次スケジューラーが起動
2. 該当日付に生成すべきルーチンを取得
3. 各ルーチンをタスクに変換
4. 対応するプロジェクト/セクションにタスク作成
5. 生成履歴を更新