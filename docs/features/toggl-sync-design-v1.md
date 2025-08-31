# Toggl Track Sync — Design v1 (proposal)

Status: draft (research-backed). Scope is an initial, minimal, push‑only sync from Chute_kun to Toggl Track.

## Goals
- Record each Chute_kun session as a Toggl Track Time Entry.
- Keep Chute_kun as the source of truth for task structure; Toggl holds the time log.
- Be resilient to being offline; reconcile later.

Non‑goals (v1)
- No pull/merge of edits done in Toggl UI.
- No project/tag auto‑classification (basic defaults only).

## Data Model Mapping

Chute_kun domain (current):
- `Task { title, estimate_min, actual_min, actual_carry_sec, started_at_min, finished_at_min, sessions: Vec<Session>, state, done_ymd }`
- `Session { start_min: u16, end_min: Option<u16> }` (minutes since local midnight)
- `DayPlan { tasks: Vec<Task> }` per day; storage snapshot splits `today` / `future` / `past`.

Toggl Track (Time Entry):
- Fields used: `id`, `description`, `start`, `stop`, `duration`, `project_id?`, `tags?`, `created_with`.

Mapping (1 session ↔ 1 time entry)
- `Task.title` → `time_entry.description`
- `Session.start_min` → `time_entry.start` (local date + minutes → ISO‑8601)
- `Session.end_min` → `time_entry.stop` (if present)
- Running session → `duration = -1` until stopped
- Optional: default `project_id`/`tags` via environment

Notes
- Chute_kun tracks minutes; Toggl stores seconds. For backfills we compute `duration = (end_min - start_min) * 60`. We ignore `actual_carry_sec` granularity per session in v1 (can refine later).

## API Usage (Track API v9)
- Create running entry: `POST /workspaces/{wid}/time_entries` with `{ description, start, duration: -1, created_with: "chute_kun" }`.
- Stop running entry: `PATCH /workspaces/{wid}/time_entries/{id}/stop`.
- Update fixed entry: `PATCH /workspaces/{wid}/time_entries/{id}` (for retroactive sessions).
- Discover IDs: `GET /me` → `default_workspace_id`. Optionally list `projects`/`tags`.
See `docs/research/2025-08-toggl-track-api.md` for details.

## Configuration
- `TOGGL_API_TOKEN` (required): personal API token.
- `TOGGL_WORKSPACE_ID` (optional): default workspace. If unset, use `GET /me`.
- `TOGGL_PROJECT_ID_DEFAULT` (optional): numeric project ID applied to new entries.
- `TOGGL_TAGS_DEFAULT` (optional): comma‑separated tag names.
- `TOGGL_SYNC_MODE` (optional): `off` | `push` (default `push`).

Secrets policy: credentials via env only. Do not write tokens to snapshots or logs.

## Sync Lifecycle (v1)

Events → API calls
- Start/Resume a task → create running time entry; store returned `id`.
- Pause/Finish a task → stop the running entry for the last session; persist.
- Backfill (e.g., editing sessions) → create/update entries with explicit `start`/`stop`.

Idempotency & Storage
- Extend session persistence to remember external IDs without polluting `SnapshotV1`:
  - Sidecar store: `.local/share/chute_kun/sync/toggl.json` with map keys `(ymd, task_index, session_index)` → `time_entry_id`.
  - Advantages: avoids changing `SnapshotV1` schema; easy to clear/rebuild.
- Alternative (later): add `Option<String> toggl_id` to `Session` if we formalize external sync in the snapshot.

Offline/Retry
- Queue operations if HTTP fails; retry with exponential backoff and jitter.
- On recovery: for any running local session without `toggl_id`, create; for finished sessions without `toggl_id`, create fixed entry with `start/stop`.
- If server already has an entry (duplicate), we detect via description+start collision heuristics or via a local dedupe cache. In v1, rely on local sidecar to prevent duplicates.

Rate Limiting
- Target ≤ 1 req/sec average during interactive use; batch backfills with pacing.
- Handle `429` with exponential backoff.

## Minimal Module Plan (not implemented yet)
- `src/lib/sync/toggl.rs` (HTTP client + DTOs)
  - `start_session(task_title, start_ts, opts) -> Result<EntryId>`
  - `stop_session(entry_id) -> Result<()>`
  - `upsert_fixed_session(task_title, start_ts, stop_ts, opts) -> Result<EntryId>`
- `src/lib/sync/state.rs` (sidecar store for `(ymd, task_idx, session_idx) -> toggl_id`)
- Inject via command‑mode actions or automatic hooks in `App` state transitions.

## Edge Cases
- App crash while running: on next launch, if local has active session but Toggl doesn’t, create running entry with `start` from local session start.
- Timezone change across a day: serialize `start/stop` with explicit offset captured at event time.
- Manual edits in Toggl: out of scope (push‑only). We keep local authoritative.

## Testing Strategy
- Unit: timestamp conversion (local minutes ↔ ISO‑8601), sidecar keying, retry/backoff.
- Integration (behind feature flag): mock HTTP (e.g., `httpmock`) for POST/PATCH/GET flows.
- Property tests: idempotency of replaying queued operations.

## Open Questions (later phases)
- Two‑way sync and conflict resolution.
- Rich mapping (projects/tags by title prefix or rules).
- Session‑level seconds tracking in the domain model.

