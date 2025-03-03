# 統合テスト実施ガイド

## 概要

本ドキュメントでは、Chute-kun プロジェクトにおける統合テストの実施手順と最良の実践方法を解説します。特に Todoist API との連携テストに焦点を当て、Kent Beck のテスト駆動開発（TDD）のサイクルに沿った形で進める方法を説明します。

## 最終更新日

2025-03-03

## 前提条件

- Node.js 18.x 以上がインストールされていること
- プロジェクトのパッケージが`npm install`でインストールされていること
- `.env`ファイルに有効な`TODOIST_API_TOKEN`が設定されていること

## 1. テスト駆動開発サイクルでの統合テスト

### 1.1 RED: 失敗するテストを書く

1. テスト対象の機能に対応する統合テストを作成する
2. 期待される動作を明確に定義する
3. テストを実行して意図的に失敗させる（まだ実装されていないため）

```bash
# 特定のテストファイルを実行
npm test -- src/tests/integration/todoistClient.integration.test.ts
```

### 1.2 GREEN: 最小限の実装でテストを通す

1. テストを通過させるために必要最小限のコードを実装する
2. 実装はきれいでなくても良い、まずはテストを通すことを優先
3. テスト実行で確認

```bash
# テストを実行して成功することを確認
npm test -- src/tests/integration/todoistClient.integration.test.ts
```

### 1.3 REFACTOR: コードを改善する

1. コードをリファクタリングして読みやすくする
2. テストが引き続き通ることを確認しながら進める
3. エラーハンドリングの改善やエッジケースの考慮を行う

```bash
# リファクタリング後も全テストが通ることを確認
npm test
```

## 2. 統合テストの実行方法

### 2.1 全テスト実行

```bash
npm test
```

### 2.2 特定の統合テストのみ実行

```bash
npm test -- src/tests/integration/todoistClient.integration.test.ts
```

### 2.3 特定のテストケースのみ実行

```bash
npm test -- -t "getTodayTasksWithSubtasks"
```

### 2.4 統合テストをスキップして実行

```bash
SKIP_INTEGRATION_TESTS=true npm test
```

## 3. 統合テスト作成時の注意点

### 3.1 API 認証情報の取り扱い

- `.env`ファイルに API 認証情報を保存する
- コード内に直接 API 認証情報を記述しない
- CI/CD 環境では環境変数として設定する

### 3.2 エラーハンドリング

- API 通信エラーを適切に処理する
- タイムアウト処理を実装する
- リトライメカニズムを検討する

```typescript
try {
  // API通信処理
  const result = await apiFunction();
  // 結果の検証
} catch (error) {
  console.error('エラー詳細:', error);
  // エラー処理
} finally {
  // クリーンアップ処理
}
```

### 3.3 テストデータの分離

- テスト用のデータセットを用意する
- テスト終了後は作成したデータを削除する
- 既存のデータに依存しないテスト設計

```typescript
// テスト用タスクの作成
const timestamp = Date.now();
const testTaskName = `テスト用タスク_${timestamp}`;
const testTask = await api.addTask({ content: testTaskName });

try {
  // テストコード
} finally {
  // テストデータのクリーンアップ
  await api.deleteTask(testTask.id);
}
```

### 3.4 非同期処理の取り扱い

- async/await パターンを使用する
- 適切な待機時間を設定する
- Promise の適切な取り扱い

```typescript
// 非同期待機を実装
const delay = (ms: number): Promise<void> => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};

// テスト内で使用
await delay(2000); // 2秒待機
```

## 4. デバッグとトラブルシューティング

### 4.1 ログ出力の活用

- 詳細なログでテスト実行状況を可視化する
- 重要なポイントにデバッグ出力を入れる

```typescript
console.log(`テストタスク作成: ID=${task.id}, Content=${task.content}`);
```

### 4.2 タイムアウト設定

- 長時間実行されるテストには適切なタイムアウト設定を行う

```typescript
// タイムアウト時間を60秒に設定
test('長時間かかるテスト', async () => {
  // テスト内容
}, 60000);
```

### 4.3 実行環境の違いへの対応

- 開発環境と CI 環境での動作の違いに注意
- 環境変数による制御を活用

## 5. 統合テストの保守

### 5.1 定期的なテスト実行

- CI/CD 環境でテストを自動実行する
- 定期的に手動でもテストを実行し、環境依存の問題がないか確認する

### 5.2 テストケースの更新

- API 仕様変更があった場合は、速やかにテストを更新する
- テストが脆弱すぎる場合は、より堅牢な実装に改善する

### 5.3 テスト結果の分析

- テストの失敗理由を分析し、システムの問題点を発見する
- 繰り返し発生する問題はパターン化して対策する

## 関連ドキュメント

- [Todoist API 統合テストの実装研究](../research/jest-integration-testing.md)
- [統合テストエラーハンドリング改善 ADR](../adr/ADR-006-Integration-Test-Error-Handling.md)
- [開発ワークフロー](./development-workflow.md)
