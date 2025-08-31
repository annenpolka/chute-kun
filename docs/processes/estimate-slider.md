---
title: Estimate Slider UI
status: accepted
updated: 2025-08-31
---

Overview
- Task estimate input/editing is unified with a slider UI.
- Applies to both: (1) editing an existing task (e / right‑click), (2) new task creation after title.

Interaction
- Keyboard: Left/Right/Down/Up/j/k adjust by ±5m; Enter confirms; Esc cancels.
- Mouse: click or drag on the track to set minutes; click OK to confirm, Cancel to abort.
- Range/step: 0–240 minutes, step 5 (configurable in a future ADR).
 - Help Line: while the estimate popup is open, the bottom help shows only estimate-related hints (OK/Cancel, ±5m, ±1 day, slider/date clicks).

Visuals
- One‑line slider rendered as [====●····] with colored segments.
- Header line shows “Estimate: <Xm> — <title>”.
- Buttons are labeled “OK” / “Cancel” for consistency across estimate popups.

Notes
- New task flow is two‑step: Title → Estimate (slider). Empty estimate confirms to default (25m for normal, 15m for interrupt) if Enter is pressed directly.
- Dragging updates the value in real‑time; keyboard and mouse interoperate.

Testing
- See tests: app_estimate_slider_drag_test.rs, app_new_task_estimate_drag_test.rs.
