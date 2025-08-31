# Task Categories (v1)

Last updated: 2025-08-31

Purpose: Provide a minimal, built-in way to classify tasks and color-code them in the TUI for quick visual scanning.

## Minimal Set
- Work: professional work-related tasks.
- Home: household/family/personal chores.
- Hobby: leisure and self-driven activities.
- General: default fallback when no category is chosen.

Rationale: Start small to avoid friction. These cover most daily contexts while keeping UI noise minimal. We can expand later if real usage demands.

## UI Representation
- Each task row shows a colored dot right after the state icon:
  - Work: Blue dot
  - Home: Yellow dot
  - Hobby: Magenta dot
  - General: White dot
- The dot appears in all views; row highlight only affects background color.

## Keyboard
- `c`: Cycle category for the currently selected task: General → Work → Home → Hobby → General.
 - When the Category Picker is open, use Up/Down (or j/k) to choose and Enter to apply (Esc to cancel).

## Persistence
- Category is stored on each `Task` in the snapshot (TOML). Missing field in older snapshots defaults to `General` for backward compatibility.

## Out of Scope (v1)
- Custom category names or colors via config.
- Filtering/sorting by category.
- Editing category during new-task input flow.

## Mouse
- Left‑click the colored dot to cycle the category.
- Right‑click the colored dot to open the Category Picker overlay (navigate with Up/Down, Enter=apply, Esc=cancel).

These can be added incrementally once the v1 proves useful.
