---
title: Clock Abstraction and Testing
status: accepted
updated: 2025-08-30
---

# Clock Abstraction and Testing

This app renders schedule prefixes and the header ESD based on the current local time. To keep UI logic testable and deterministic, a simple `Clock` abstraction is introduced.

## Design

- Module: `src/lib/clock.rs`
- API:
  - `trait Clock { fn now_minutes(&self) -> u16 }` — minutes since local midnight (0..=1439)
  - `struct SystemClock` — uses `chrono::Local` to read the OS local time
  - `fn system_now_minutes() -> u16` — convenience for non-DI paths

UI uses `system_now_minutes()` by default. For tests, `ui::draw_with_clock` accepts `&dyn Clock` to inject fixed times.

## Testing Approach

- Unit tests verify that `system_now_minutes()` returns local minutes using `chrono::Local`.
- Rendering tests use `ratatui::backend::TestBackend` and `ui::draw_with_clock` with a `FixedClock` to assert header content contains the expected ESD and totals.

## Rationale

- Avoids UTC-vs-local drift (previously used UNIX epoch modulo a day).
- Keeps domain time calculations decoupled from wall-clock source.
- Maintains determinism in tests by allowing fixed time injection.

## Notes

- Seconds accumulation for active tasks remains `Instant`-based and is TZ-agnostic.
- If future features need dates or time zones, consider layering a higher-level time provider that returns a full local datetime.

