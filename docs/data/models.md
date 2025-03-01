# データモデル定義

*最終更新日: 2025-03-01*

## 概要

TaskChuteシステムは以下の主要データモデルを使用します。これらのモデルはTodoist APIとの連携、LLMによる予測、タイムブロック生成に必要な情報を格納します。

## エンティティ関係図

```
┌───────────────┐       ┌───────────────┐       ┌───────────────┐
│  Task         │       │  TimeBlock    │       │  TaskLog      │
│               │       │               │       │               │
│ id            │──┐    │ id            │       │ id            │
│ todoistId     │  │    │ taskId      ◄─┼───────┤ taskId        │
│ title         │  │    │ startTime     │       │ startTime     │
│ description   │  └───►│ endTime       │       │ endTime       │
│ projectId     │       │ duration      │       │ duration      │
│ priority      │       │ status        │       │ actualDuration│
│ estimatedTime │       │               │       │ status        │
└───────────────┘       └───────────────┘       └───────────────┘
        │                       │                       │
        │                       │                       │
        ▼                       ▼                       ▼
┌───────────────┐       ┌───────────────┐       ┌───────────────┐
│  Project      │       │  Schedule     │       │  Prediction   │
│               │       │               │       │               │
│ id            │       │ id            │       │ id            │
│ todoistId     │       │ date          │       │ taskId        │
│ name          │       │ timeBlockIds  │       │ estimatedTime │
│ color         │       │ status        │       │ confidence    │
└───────────────┘       └───────────────┘       │ factors       │
                                                └───────────────┘
```

## データモデル定義

### Task (タスク)
タスクは作業の基本単位であり、Todoistから取得します。

```typescript
interface Task {
  id: string;                  // 内部ID
  todoistId: string;           // Todoist上のタスクID
  title: string;               // タスク名
  description?: string;        // 説明
  projectId: string;           // 所属プロジェクトID
  sectionId?: string;          // セクションID
  labels?: string[];           // ラベル（配列）
  priority: number;            // 優先度 (1-4, 4が最高)
  estimatedTime?: number;      // 推定所要時間（分）
  dueDate?: string;            // 期限日
  isCompleted: boolean;        // 完了状態
  createdAt: string;           // 作成日時
  updatedAt: string;           // 更新日時
}
```

### TimeBlock (タイムブロック)
タイムブロックはタスクの実行予定時間枠を表します。

```typescript
interface TimeBlock {
  id: string;                  // ID
  taskId: string;              // 関連タスクID
  startTime: string;           // 開始時刻 (ISO形式)
  endTime: string;             // 終了時刻 (ISO形式)
  duration: number;            // 所要時間（分）
  status: TimeBlockStatus;     // 状態 (scheduled|in_progress|completed|cancelled)
  createdAt: string;           // 作成日時
  updatedAt: string;           // 更新日時
}

type TimeBlockStatus = 'scheduled' | 'in_progress' | 'completed' | 'cancelled';
```

### TaskLog (タスク実行ログ)
タスクの実際の実行記録を保存します。

```typescript
interface TaskLog {
  id: string;                  // ID
  taskId: string;              // 関連タスクID
  timeBlockId?: string;        // 関連タイムブロックID
  startTime: string;           // 実際の開始時刻
  endTime?: string;            // 実際の終了時刻
  duration?: number;           // 実際の所要時間（分）
  status: TaskStatus;          // 状態
  note?: string;               // メモ
  createdAt: string;           // 作成日時
  updatedAt: string;           // 更新日時
}

type TaskStatus = 'started' | 'paused' | 'resumed' | 'completed' | 'cancelled';
```

### Project (プロジェクト)
プロジェクトはタスクのグループです。

```typescript
interface Project {
  id: string;                  // 内部ID
  todoistId: string;           // Todoist上のプロジェクトID
  name: string;                // プロジェクト名
  color?: string;              // 色 (HEX)
  isArchived: boolean;         // アーカイブ状態
  createdAt: string;           // 作成日時
  updatedAt: string;           // 更新日時
}
```

### Schedule (スケジュール)
1日のタイムブロックの集合です。

```typescript
interface Schedule {
  id: string;                  // ID
  date: string;                // 日付 (YYYY-MM-DD)
  timeBlockIds: string[];      // タイムブロックIDの配列
  status: ScheduleStatus;      // 状態
  workStartTime?: string;      // 仕事開始時間 (HH:MM)
  workEndTime?: string;        // 仕事終了時間 (HH:MM)
  breakTimes?: BreakTime[];    // 休憩時間の配列
  createdAt: string;           // 作成日時
  updatedAt: string;           // 更新日時
}

type ScheduleStatus = 'draft' | 'active' | 'completed';

interface BreakTime {
  startTime: string;           // 休憩開始時間 (HH:MM)
  endTime: string;             // 休憩終了時間 (HH:MM)
  duration: number;            // 所要時間（分）
  type: string;                // 休憩タイプ (lunch, coffee, etc.)
}
```

### Prediction (予測)
LLMによるタスク所要時間予測を保存します。

```typescript
interface Prediction {
  id: string;                  // ID
  taskId: string;              // 関連タスクID
  estimatedTime: number;       // 推定所要時間（分）
  confidence: number;          // 信頼度 (0.0-1.0)
  factors: PredictionFactor[]; // 予測に影響した要素
  model: string;               // 使用したモデル名
  createdAt: string;           // 作成日時
}

interface PredictionFactor {
  name: string;                // 要素名
  weight: number;              // 重み
  description?: string;        // 説明
}
```

### UserPreferences (ユーザー設定)
ユーザー固有の設定を保存します。

```typescript
interface UserPreferences {
  id: string;                  // ID
  todoistToken: string;        // Todoist APIトークン（暗号化）
  workDays: number[];          // 勤務日 (0-6, 0=日曜)
  workStartTime: string;       // 標準作業開始時間 (HH:MM)
  workEndTime: string;         // 標準作業終了時間 (HH:MM)
  breakPatterns: BreakPattern[]; // 標準休憩パターン
  productivityPattern?: ProductivityPattern; // 生産性パターン
  notificationEnabled: boolean; // 通知有効フラグ
  theme: string;               // UIテーマ
  updatedAt: string;           // 更新日時
}

interface BreakPattern {
  type: string;                // 休憩タイプ
  duration: number;            // 所要時間（分）
  frequency: number;           // 頻度（時間ごと）
}

interface ProductivityPattern {
  morningRating: number;       // 朝の生産性 (1-5)
  afternoonRating: number;     // 午後の生産性 (1-5)
  eveningRating: number;       // 夕方の生産性 (1-5)
  peakHours: string[];         // 最高生産性の時間帯 (HH:MM-HH:MM)
}
```

## スキーマ制約

1. **Task**:
   - `title`は必須、1-255文字
   - `priority`は1-4の整数
   - `estimatedTime`は0以上の数値

2. **TimeBlock**:
   - `startTime`は必須、ISO 8601形式
   - `endTime`は`startTime`より後
   - `duration`は0より大きい整数

3. **TaskLog**:
   - `startTime`は必須、ISO 8601形式
   - `endTime`が存在する場合は`startTime`より後
   - `status`が'completed'の場合は`endTime`が必須

4. **Schedule**:
   - `date`は必須、YYYY-MM-DD形式
   - 各`timeBlockIds`は重複しない
   - `workStartTime`と`workEndTime`はHH:MM形式

## データの永続化

1. **Todoist APIとの同期**:
   - タスク、プロジェクト、ラベルはTodoist APIと同期
   - コメント欄を利用して実績時間を記録

2. **ローカルストレージ**:
   - タイムブロック、予測、ユーザー設定はローカルに保存
   - IndexedDBまたはローカルファイルシステムを使用

3. **暗号化**:
   - Todoist APIトークンなどの認証情報は暗号化して保存
   - ユーザーデータはデバイス上で安全に管理