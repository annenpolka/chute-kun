# CLAUDE.md for chute_kun project

## プロジェクト概要
このリポジトリはTaskChute（タスクシュート）手法を実装したタスク管理CLIツールのコードとドキュメントを含んでいます。TaskChuteは大橋悦夫氏によって開発された、計画、記録、ルーティン管理に焦点を当てたタスク管理手法です。

## Project Description
This repository contains documentation and implementation of TaskChute methodology, a task management system developed by Etsuo Ohashi focused on planning, logging, and routine management.

## Development Guidelines

### Development Approach
- **Test-Driven Development (TDD)**: Follow Kent Beck's practices with Red-Green-Refactor cycle
- **Simple Design**: Prefer simplicity and avoid premature optimization 
- **Incremental Design**: Evolve design alongside code rather than detailed upfront planning
- **Continuous Testing**: All features should have automated tests
- **Context Clarification**: When requirements or context is unclear, always ask the user for clarification before proceeding
- **Document-Driven Development**: Documentation is an integral part of the development process, not an afterthought
- **Post-Commit Documentation Hook**: Every code change requires corresponding documentation updates
- **Disciplined Workflow**: Follow the structured workflow defined in `docs/processes/development-workflow.md`

### File Organization
- Maintain clear separation between documentation files and implementation code
- Use descriptive filenames that reflect content purpose
- Store all documentation in the `docs/` directory following the structure defined below
- Keep tests alongside implementation code (e.g., `src/feature.ts` and `src/feature.test.ts`)

### Documentation Style
- Write documentation in Markdown format
- Use Japanese as primary language with English translations where appropriate
- Include practical examples for implementation concepts

### Code Style
- Comment code thoroughly in both Japanese and English
- Follow consistent indentation (2 spaces recommended)
- Use meaningful variable names that reflect TaskChute terminology
- Follow functional programming principles where appropriate

## 開発コマンド / Build/Test Commands

### テスト関連 / Testing
- **単体テスト / Unit Tests**: `npm test`
- **テストウォッチモード / Test Watch Mode**: `npm test -- --watch` (TDDワークフロー用)
- **テストカバレッジ / Test Coverage**: `npm test -- --coverage`

### ビルド・実行 / Build & Run
- **ビルド / Build**: `npm run build`
- **開発モード実行 / Development Server**: `npm run dev`
- **CLIモード実行 / CLI Mode**: `npm run cli`

### コード品質 / Code Quality
- **リント / Lint**: `npm run lint`
- **フォーマット / Format**: `npm run format`

## ドキュメント管理要件 / Documentation Management Requirements

### 1. ドキュメント構造概要 / Document Structure Overview
- 完全なドキュメントガイドと構造は `docs/README.md` を参照
- ドキュメントは論理的なカテゴリで整理され、明確なナビゲーションを提供
- 各ドキュメントは特定の目的と対象読者を持つ
- 最新のドキュメント構造：
  ```
  docs/
  ├── README.md                # ドキュメント全体の目次
  ├── system-overview.md       # システム全体の概要
  ├── adr/                     # アーキテクチャ決定記録
  ├── api/                     # API仕様ドキュメント
  ├── data/                    # データモデル定義
  ├── decisions/               # 意思決定記録
  ├── features/                # 機能仕様書
  ├── modules/                 # モジュール仕様
  ├── planning/                # 計画・設計ドキュメント
  ├── processes/               # 開発プロセス
  ├── research/                # 調査研究資料
  └── setup/                   # セットアップ・構成情報
  ```

### 2. Architecture Decision Records (ADRs)
- Create ADRs for all framework selections, design patterns, data model changes, API designs
- Store in `docs/adr/ADR-{number}-{title}.md`
- Include status, date, context, decision, rationale, consequences, and related documents
- Never contradict existing ADRs without updating their status

### 3. System Documentation
- System overview: `docs/system-overview.md` (components, roles, dependencies)
- Module specifications: `docs/modules/{module-name}.md` (responsibilities, interfaces, dependencies)
- Data models: `docs/data/models.md` (entity diagrams, schemas, constraints)
- API specifications: `docs/api/{module-name}.md` (interfaces, functions, parameters, examples)
- Feature specifications: `docs/features/{feature-name}.md` (detailed feature requirements)

### 4. Development Process Documentation
- Development workflow: `docs/processes/development-workflow.md` (document-driven development)
- Commit strategy: `docs/processes/commit-strategy.md` (commit conventions)
- Decision log: `docs/processes/decisions-log.md` (for minor decisions)

### 5. Setup and Configuration
- Implementation status: `docs/setup/implementation-status.md` (current progress and next steps)
- Environment variables: `docs/setup/environment-variables.md` (configuration settings)

### 6. Planning Documents
- Development plan: `docs/planning/development-plan.md` (roadmap and milestones)
- CLINE configuration: `docs/planning/cline-config.md` (Claude CLI configuration)

### 7. Documentation Maintenance Rules
- Reference relevant documentation before any code changes
- Maintain consistent terminology across all documents
- Update documentation simultaneously with code changes (documentation as post-commit hook)
- Resolve contradictions immediately
- Mark planned features explicitly
- Include last modified date on all documents

## 実装メモ / Implementation Notes

TaskChute手法は以下の3つの中核機能に焦点を当てています：

1. **計画 / Plan**: 日々のタスクを実行順序で整理
   - 優先度と所要時間に基づくタスクの最適配置
   - タイムブロック生成による効率的な時間配分

2. **記録 / Log**: タスク実行時間と結果の記録
   - 開始・終了時刻の記録
   - 実績と予測の比較分析
   - 振り返りと改善のためのデータ収集

3. **ルーティン / Routine**: 効率化のための定期的なタスク管理
   - 繰り返しタスクの自動計画
   - 習慣化のための仕組み作り
   - 定期的なパターン認識と効率化

すべての実装はこれらの原則に則り、上記で定義された構造に従って徹底的にドキュメント化する必要があります。

## 外部ライブラリ使用ガイドライン / External Library Usage Guidelines

1. **型定義の確認**: ライブラリの使用前に型定義ファイル(.d.ts)を確認する
2. **API応答処理**: API応答の形式変更に堅牢な実装を心がける
3. **バージョン管理**: 依存ライブラリのバージョンを明示的に管理する
4. **互換性テスト**: ライブラリ更新時には明示的に互換性テストを行う

詳細は `docs/processes/development-workflow.md` の「外部ライブラリ・API利用方針」セクションを参照してください。