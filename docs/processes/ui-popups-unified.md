title: UI Popups Unification

# UI Popups Unification

Status: Adopted (2025-08-31)

Overview
- The main task list/table always remains visible. All interactive flows use centered popup overlays.
- Aims: zero context switches, predictable input model, consistent mouse/keyboard affordances.

In-Scope Popups
- Title Input (New/Interrupt): OK/Cancel. Enter confirms, Esc cancels.
- New-Task Estimate (Slider): Add/Cancel. Arrow keys/j/k adjust ±5m; slider is clickable/drag‑gable. Date line with weekday is shown; see "Date Picker".
- Estimate Edit (Slider): OK/Cancel. Same adjustment semantics as above. Date line with weekday is shown; see "Date Picker".
- Start Time (Slider): OK/Cancel. Open with Space. Arrow keys/j/k adjust ±5m; slider is clickable to set a fixed planned start time (per task). Applied time turns the Plan column cyan for that task.
- Command Palette: Run/Cancel. Input is typed while the popup is open; Enter or Run executes; Esc or Cancel closes without running.
- Delete Confirmation: Delete/Cancel. While open, header (Act seconds) is frozen.

Keyboard Semantics
- Enter: confirm (OK/Add/Run/Delete). Esc: cancel.
- Estimate adjustment: ←/→/j/k ±5m.
- Start Time adjustment: ←/→/j/k ±5m.
- New task defaults: Normal 25m, Interrupt 15m when estimate is not typed.
- Date Picker: `.` = +1 day, `,` = −1 day（clamped ≥ Today）.

Mouse Semantics
- Buttons: hover highlight; left click to activate. Slider tracks accept click/drag to set minutes.
- Date Picker: click `<` or `>` to step −/+1 day.
- Tabs, list selection, and other interactions remain available when no popup is open.

Rendering Rules
- Popups are centered and sized to content; buttons are horizontally centered within the inner box.
- The main content (task table or empty hint) does not change while popups are shown.
- Date line displays one of `Today (Wed)`, `Tomorrow (Thu)`, or `YYYY-MM-DD (Fri)`. Operation hints live in the help line, not inside the popup.

Help Line (scoped)
- While a popup is open, the bottom help shows only the hints relevant to that popup (context-specific), hiding unrelated global actions.
  - Confirm Delete: `Enter/y: delete`, `Esc/n: cancel`.
  - Estimate Edit: `Enter: OK`, `Esc: cancel`, `←/→/j/k: ±5m`, `.,: ±1 day`, `click slider`, `click < >`.
  - New Task Estimate: `Enter: add`, `Esc: cancel`, `.,: ±1 day`, `click slider`, `click < >`.
  - Command Palette: `Enter: run`, `Esc: cancel`, `type/backspace: edit`.
  - Title Input (New/Interrupt): `Enter: next`, `Esc: cancel`, `type/backspace: edit`.
- When no popup is open, the help shows the view-aware general shortcuts (navigation, start/pause, finish, reorder, etc.).

Tests (authoritative examples)
- Command popup mouse E2E: `tests/app_command_popup_mouse_buttons_test.rs`.
- Delete popup mouse E2E: `tests/app_delete_popup_mouse_buttons_test.rs`.
- Estimate editor/new-task estimate: `tests/ui_estimate_stepper_test.rs`, `tests/app_new_task_estimate_drag_test.rs`.
- Main list stays during delete confirm: `tests/ui_delete_prompt_test.rs`.
 - Popup-scoped help behavior: `tests/ui_help_popup_scoped_test.rs`.

Related Modules
- `src/lib/ui.rs`: popup geometry helpers and overlay renderers.
- `src/lib/app.rs`: input state, key/mouse handling while popups are open.
