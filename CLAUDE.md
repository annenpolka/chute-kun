# CLAUDE.md for chute_kun project

## Project Description
This repository contains documentation and implementation of TaskChute methodology, a task management system developed by Etsuo Ohashi focused on planning, logging, and routine management.

## Development Guidelines

### Development Approach
- **Test-Driven Development (TDD)**: Follow Kent Beck's practices with Red-Green-Refactor cycle
- **Simple Design**: Prefer simplicity and avoid premature optimization 
- **Incremental Design**: Evolve design alongside code rather than detailed upfront planning
- **Continuous Testing**: All features should have automated tests
- **Context Clarification**: When requirements or context is unclear, always ask the user for clarification before proceeding

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

## Build/Test Commands
- **Unit Tests**: `npm test` (or `yarn test`)
- **Test Watch Mode**: `npm test -- --watch` (for TDD workflow)
- **Test Coverage**: `npm test -- --coverage`
- **Build**: `npm run build`
- **Development Server**: `npm run dev`

## Documentation Management Requirements

### 1. Architecture Decision Records (ADRs)
- Create ADRs for all framework selections, design patterns, data model changes, API designs
- Store in `docs/adr/ADR-{number}-{title}.md`
- Include status, date, context, decision, rationale, consequences, and related documents
- Never contradict existing ADRs without updating their status

### 2. System Documentation
- System overview: `docs/system-overview.md` (components, roles, dependencies)
- Module specifications: `docs/modules/{module-name}.md` (responsibilities, interfaces, dependencies)
- Data models: `docs/data/models.md` (entity diagrams, schemas, constraints)
- API specifications: `docs/api/spec.md` (endpoints, request/response formats, error codes)
- Decision log: `docs/decisions/log.md` (for minor decisions)

### 3. Documentation Maintenance Rules
- Reference relevant documentation before any code changes
- Maintain consistent terminology across all documents
- Update documentation simultaneously with code changes
- Resolve contradictions immediately
- Mark planned features explicitly
- Include last modified date on all documents

## Implementation Notes
The TaskChute methodology centers on three core functions:
1. Plan: Organizing daily tasks in execution order
2. Log: Recording task execution times and results
3. Routine: Managing recurring tasks for efficiency

Any implementation must align with these principles and be documented thoroughly using the structure defined above.