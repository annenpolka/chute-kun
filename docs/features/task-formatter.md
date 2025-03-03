# タスク表示フォーマッタ / Task Formatter

*最終更新日 / Last Updated: 2025-03-03*

## 概要 / Overview

タスク表示フォーマッタは、TodoistのタスクデータをCLI上で見やすく表示するための機能です。さまざまな表示オプションをサポートし、異なる出力コンテキスト（今日のタスク一覧、検索結果など）に適した表示形式を提供します。

The Task Formatter provides a clean and consistent way to display Todoist task data in the CLI. It supports various display options and offers tailored output formats for different contexts (today's tasks, search results, etc.).

## 機能 / Features

- 完全にカスタマイズ可能な表示オプション
- 用途別のプリセット定義
- 優先度、完了状態、期限などの視覚的表現
- 関数型設計によるシンプルなインターフェース
- テスト駆動開発によって設計された堅牢な実装

## 使用方法 / Usage

### 基本的な使用法 / Basic Usage

```typescript
import { formatTask, displayTasks } from './lib/formatters/taskFormatter';

// 単一タスクのフォーマット
const formattedTask = formatTask(task, 0);
console.log(formattedTask);

// タスク配列の表示
displayTasks(tasks);
```

### カスタム表示オプション / Custom Display Options

```typescript
import { formatTask, FormatTaskOptions } from './lib/formatters/taskFormatter';

const options: FormatTaskOptions = {
  showStatus: true,
  showPriority: true,
  showDueDate: true,
  showIndex: true,
  prioritySymbol: "*",
  completedSymbol: "✅",
  uncompletedSymbol: "⬜",
  dueDateFormat: "due: %s",
  dueDateBrackets: ["(", ")"]
};

const formattedTask = formatTask(task, 0, options);
```

### プリセットの使用 / Using Presets

```typescript
import {
  displayTasks,
  TODAY_TASKS_FORMAT,
  FILTER_RESULT_FORMAT,
  COMPACT_FORMAT
} from './lib/formatters/taskFormatter';

// 今日のタスク表示用フォーマット
displayTasks(todayTasks, console.log, TODAY_TASKS_FORMAT);

// フィルタリング結果用フォーマット
displayTasks(filteredTasks, console.log, FILTER_RESULT_FORMAT);

// コンパクト表示用フォーマット
displayTasks(tasks, console.log, COMPACT_FORMAT);
```

## 設定オプション / Configuration Options

| オプション | 型 | デフォルト値 | 説明 |
|------------|------|--------------|------|
| `showStatus` | boolean | `true` | 完了状態を表示するか |
| `showPriority` | boolean | `true` | 優先度を表示するか |
| `showDueDate` | boolean | `true` | 期限を表示するか |
| `showIndex` | boolean | `true` | インデックス番号を表示するか |
| `indexOffset` | number | `1` | インデックス番号のオフセット |
| `prioritySymbol` | string | `"!"` | 優先度を表すシンボル |
| `completedSymbol` | string | `"✓"` | 完了タスクのシンボル |
| `uncompletedSymbol` | string | `"□"` | 未完了タスクのシンボル |
| `noPriorityPlaceholder` | string | `"-"` | 優先度なしの表示 |
| `dueDateFormat` | string | `"期限: %s"` | 期限表示のフォーマット |
| `dueDateBrackets` | [string, string] | `["[", "]"]` | 期限表示の括弧 |

## 定義済みプリセット / Predefined Presets

### 今日のタスク表示用 (TODAY_TASKS_FORMAT)

標準的な表示形式で、完了状態、優先度、タスク内容、期限を角括弧 `[]` で囲んで表示します。

```
1. □ [!!!] サンプルタスク [期限: 2025-03-03]
```

### フィルタリング結果用 (FILTER_RESULT_FORMAT)

検索結果表示用の形式で、期限を丸括弧 `()` で囲んで表示します。

```
1. □ [!!!] サンプルタスク (期限: 2025-03-03)
```

### コンパクト表示用 (COMPACT_FORMAT)

シンプルな表示形式で、完了状態を非表示にし、期限のラベルを短縮しています。

```
1. [!!!] サンプルタスク [due: 2025-03-03]
```

## 技術的詳細 / Technical Details

### アーキテクチャ / Architecture

タスクフォーマッタは関数型設計で実装されており、純粋関数として動作します。各関数は入力値に基づいて出力を生成し、副作用を持ちません。これにより、テスト容易性と再利用性が向上しています。

### コアコンポーネント / Core Components

1. **formatTask**: 単一タスクをフォーマットする関数
2. **displayTasks**: タスク配列を表示する関数
3. **FormatTaskOptions**: 表示オプションを定義するインターフェース
4. **プリセット定義**: 一般的な用途向けの設定セット

### パフォーマンス考慮事項 / Performance Considerations

- フォーマット処理は軽量で、大量のタスクを処理する場合でもパフォーマンス問題は発生しません
- 出力はテキストベースでメモリ効率が良く、リソース消費が最小限です

## 将来の拡張 / Future Enhancements

- カラー出力のサポート（ANSI カラーコードの使用）
- 階層表示の強化（インデントやツリー表示）
- ソート順オプションの追加
- グループ化オプションの追加
- 国際化対応（多言語メッセージ）

## テスト / Testing

タスクフォーマッタはテスト駆動開発（TDD）で設計されており、自動テストが実装の品質を保証します。テストスイートは以下のケースをカバーしています：

- デフォルトオプションでのフォーマット
- カスタムオプションでのフォーマット
- インデックスなしでのフォーマット
- 最小限の表示オプション
- displayTasks関数の動作

テストの実行方法：

```bash
npx jest src/tests/formatters/taskFormatter.test.ts
```

## 関連モジュール / Related Modules

- **todoistClient.ts**: Todoist APIとの通信
- **index.ts**: CLIエントリポイント