---
title: Unify Action Labels to OK/Cancel in Popups
status: accepted
date: 2025-08-31
---

Context
- Popup actions were inconsistently labeled (e.g., "Add" in title input, "OK" in other dialogs).
- Inconsistent labels confuse users and complicate help text and tests.

Decision
- All confirmation buttons in popups use the label "OK"; negative action uses "Cancel".
- Applies to:
  - Title input popup (New Task): OK / Cancel
  - Estimate editor popup (existing task): OK / Cancel
  - New task estimate popup (post-title): OK / Cancel

Consequences
- UI and help strings reflect Enter=OK / Esc=Cancel uniformly.
- Mouse hitboxes are computed from the label widths; tests use hitboxes, not text, to stay robust.
- Backward compatibility: internal event names (e.g., `InputAdd`) are kept to avoid churn in code/tests. Only labels changed.

Alternatives Considered
- Keep "Add" only for creation contexts: rejected due to inconsistency and extra cognitive load.

Migration / Backward Compatibility Policy
- No config or keymap changes. Behavior is unchanged; only labels and help strings changed.
- Tests referencing labels were updated (function names/comments). Public CLI/TUI usage is unaffected beyond wording.

