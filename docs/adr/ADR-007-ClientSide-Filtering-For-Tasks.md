# ADR-007: 今日期限タスク取得のクライアント側フィルタリングへの移行

## ステータス

承認済み

_最終更新日: 2025-03-05_
_作成者: チームメンバー_

## コンテキスト

TaskChute アプリケーションでは、今日期限のタスクとそのサブタスクを効率的に取得・表示する必要があります。従来の実装では、Todoist API の「today」フィルターを使用して今日期限のタスクを取得し、その後個別にサブタスクを取得していましたが、この方法には以下の課題がありました：

1. 親タスクが今日期限でなくても、サブタスクが今日期限の場合に正しく表示されない
2. 複数回の API 呼び出しが必要となり、効率が悪い
3. タスク階層の構築処理が複雑になる

これらの課題を解決するためにタスク取得方法の改善が必要となりました。

## 検討された選択肢

### 選択肢 1: API 側のフィルターを利用（従来方式の維持）

- **概要**: Todoist API の「today」フィルターを使用して今日期限のタスクを取得し、別途サブタスクを取得する
- **利点**:
  - API サイドでフィルタリングするため、データ転送量が少ない
  - 既存の実装が利用できる
- **欠点**:
  - 親タスクが今日期限でない場合のサブタスク表示に対応できない
  - 複数回の API 呼び出しが必要となる
  - タスク階層の構築が複雑

### 選択肢 2: クライアント側フィルタリングへの移行（採用案）

- **概要**: 全タスクを一度に取得し、クライアント側で今日期限のタスクをフィルタリングする
- **利点**:
  - 親タスクの期限に関わらず、必要なサブタスクを柔軟に表示できる
  - API 呼び出し回数の削減によるパフォーマンス向上
  - タスク階層構築の単純化
- **欠点**:
  - データ転送量の増加
  - クライアント側の処理負荷の増加

### 選択肢 3: ハイブリッドアプローチ

- **概要**: 最初は API フィルターで今日期限のタスクを取得し、必要に応じて全タスクを取得する
- **利点**:
  - 初期ロード時のパフォーマンスを維持
  - 必要な場合のみ追加データを取得
- **欠点**:
  - 実装が複雑になる
  - 場合によっては 2 回の API 呼び出しが必要

## 決定

**選択された選択肢: クライアント側フィルタリングへの移行**

API のフィルタリング機能に依存せず、全タスクを一度に取得してからクライアント側でフィルタリングする方法を採用しました。この方法により、タスクの階層関係に関わらず、より柔軟に今日のタスクとそのサブタスクを表示できるようになります。

## 影響

### 肯定的な影響

- より完全なタスク階層の取得が可能に
- API 呼び出し回数の削減によるパフォーマンス向上
- タスクデータの一元管理が容易になる
- 柔軟なフィルタリング条件の適用が可能に

### 否定的な影響

- 初期データ転送量の増加
- クライアント側の処理負荷が増える可能性
- データ量が多い場合のメモリ使用量の増加

### 中立的な影響/トレードオフ

- タスク数が少ない場合はパフォーマンスが向上するが、非常に多い場合は劣化する可能性
- より複雑な階層関係の解析が可能になるが、実装が複雑化する

## 実装詳細

主な変更点は以下の通りです：

1. `getTodayTasks`関数: 全タスクを取得し、クライアント側でフィルタリング
2. `getTodayTasksWithSubtasks`関数: 全タスクから一度に階層構造を構築

```typescript
// タスク取得関数の改善例
export async function getTodayTasks(
  api: TodoistApi | any
): Promise<TodoistTask[]> {
  try {
    const allTasks = await getTasks(api);
    const today = new Date().toISOString().split('T')[0];

    // クライアント側でフィルタリング
    return allTasks.filter((task) => {
      if (task.isCompleted) return false;
      const hasDueDate =
        typeof task.due === 'object' &&
        task.due !== null &&
        typeof task.due.date === 'string';
      return hasDueDate && task.due.date === today;
    });
  } catch (error) {
    console.error('タスク取得エラー:', error);
    throw error;
  }
}
```

## 代替案と将来の可能性

- タスク数が著しく増加した場合は、ページネーションの導入や、選択肢 3 のハイブリッドアプローチへの移行を検討
- オフライン対応のためのデータキャッシュ戦略の導入
- API の機能強化に応じてサーバーサイドのフィルタリング機能を活用

## 関連する決定

- [ADR-005-Hierarchical-Task-Display](./ADR-005-Hierarchical-Task-Display.md)
- [ADR-001-TaskManagementAPI-Selection](./ADR-001-TaskManagementAPI-Selection.md)

## 参考資料

- [Todoist API Documentation](https://developer.todoist.com/rest/v2)
- [Client-side vs Server-side Filtering Best Practices](https://developer.mozilla.org/en-US/docs/Learn/JavaScript/Client-side_web_APIs/Fetching_data)
