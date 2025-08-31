# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Chute-kun is a TaskChute-style task management TUI application built in Rust. It implements the TaskChute methodology for time management and productivity, featuring a terminal user interface built with ratatui and crossterm.

## Architecture

The codebase follows a modular structure:

- `src/cli/main.rs` - Entry point and TUI event loop
- `src/lib/` - Core business logic:
  - `app.rs` - Main application state and key handling
  - `task.rs` - Task domain models (Task, DayPlan, TaskState)  
  - `ui.rs` - UI rendering and formatting
- `tests/` - Comprehensive integration tests for all functionality

### Key Domain Models

- **Task**: Core entity with title, estimate/actual time, and state (Planned/Active/Paused/Done)
- **DayPlan**: Collection of tasks with active task tracking and time calculations
- **App**: Main application state managing views (Past/Today/Future), selection, and UI interactions

## Development Commands

### Building and Running
- `cargo run` - Run the application
- `cargo build` - Build the project
- `cargo build --release` - Release build

### Testing (TDD Approach)
- `cargo test` - Run all tests
- `cargo test [test_name]` - Run specific test
- Tests are comprehensive integration tests covering app behavior, UI rendering, and task management

### Code Quality
- `cargo fmt --all` - Format code
- `cargo fmt --all -- --check` - Check formatting
- `cargo lint` - Run clippy with strict warnings (alias for `cargo clippy --workspace --all-targets --all-features -- -D warnings`)

## TDD Development Process

This project strictly follows Test-Driven Development:

1. **Red**: Write failing test for new functionality
2. **Green**: Write minimal code to make test pass  
3. **Refactor**: Improve code while keeping tests green

All features must be developed test-first. The test suite comprehensively covers app state transitions, UI rendering, time tracking, and keyboard interactions.

## Key Features

- **Time Tracking**: Second-precision tracking with minute accumulation
- **Task States**: Planned → Active → Paused → Done state transitions
- **Multi-View**: Past/Today/Future tabs (Tab/Shift+Tab navigation)
- **Task Management**: Add interrupts (i), reorder ([/]), estimate adjustment (e), postpone (p)
- **Vim-like Navigation**: j/k or arrow keys for selection
- **ESD Calculation**: Estimated completion times based on remaining work

## Configuration Files

- `rust-toolchain.toml` - Specifies stable Rust with rustfmt/clippy components
- `rustfmt.toml` - Code formatting configuration (100 char width, Unix newlines)
- `.cargo/config.toml` - Cargo aliases for linting and formatting
- All tools are configured for consistent formatting and strict linting

## Documentation

Extensive documentation in `docs/` including:
- `system-overview.md` - High-level architecture and goals
- `adr/` - Architectural decision records
- `processes/development-workflow.md` - TDD workflow and documentation practices
- `setup/linting.md` - Code quality tool configuration
- `features/` - Feature specifications and test lists

## Important Notes

- Follow TDD strictly - no production code without failing tests first
- UI tests verify exact formatting and layout
- Time tracking maintains partial seconds across pause/resume
- App state transitions are comprehensively tested
- All keyboard shortcuts and interactions must have corresponding tests
