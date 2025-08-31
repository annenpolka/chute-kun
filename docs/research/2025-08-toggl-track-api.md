# Toggl Track API — Research (as of 2025-08-31)

This note summarizes the current Toggl Track public APIs relevant for integrating Chute_kun’s session model with Toggl time entries. Links point to official docs.

## Overview
- **Base URL**: `https://api.track.toggl.com/api/v9` (Track API v9)
- **Auth**: HTTP Basic with the user’s API token as the username and the literal string `api_token` as the password. Example header: `Authorization: Basic <base64(api_token:api_token)>`. See official auth guide.
- **Formats**: JSON request/response; timestamps are ISO‑8601 (RFC‑3339) with timezone.

References
- Track API docs (official): https://engineering.toggl.com/docs/
- Auth guide: https://engineering.toggl.com/docs/#authentication

## Key Endpoints

### User and Workspace
- `GET /me` — Returns user profile and commonly used IDs (e.g., `default_workspace_id`).
- `GET /me/quota` — Returns per‑hour API quotas by organization and usage counters.

### Time Entries (core to integration)
- `GET /me/time_entries/current` — Currently running time entry, if any.
- `GET /me/time_entries?start_date=...&end_date=...` — List entries in a time range.
- `POST /workspaces/{wid}/time_entries` — Create a new time entry.
  - For a running entry, set `duration = -1` and provide `start`.
  - Recommended fields: `description`, `start`, `duration`, `project_id` (optional), `tags` (optional), `created_with`.
- `PATCH /workspaces/{wid}/time_entries/{id}` — Update an entry (e.g., finalize `stop`/`duration`, edit `description`, etc.).
- `PATCH /workspaces/{wid}/time_entries/{id}/stop` — Stop a running entry (server sets `stop` and `duration`).

### Projects/Tags (optional lookups)
- `GET /workspaces/{wid}/projects` — Query projects; use to map defaults.
- `GET /workspaces/{wid}/tags` — Query tags.

## Rate Limits and Quotas
- Toggl Track publishes per‑hour quotas by plan (Free/Starter/Premium) and enforcement details under `GET /me/quota` (returned at runtime). Historically, a safe practical pace is ~1 request/second during interactive use. Budget additional headroom for retries and backoff.
- Behavior on limit exceed: API responds `429 Too Many Requests`; retry with exponential backoff and jitter.

References
- API usage limits: https://engineering.toggl.com/docs/#get-me-quota

## Field Notes (Time Entry)
- Minimum fields to create a running entry:
  - `description`: task title string
  - `start`: ISO‑8601 timestamp (e.g., `2025-08-31T09:05:00+09:00`)
  - `duration`: `-1` to indicate “running”
  - `created_with`: e.g., `"chute_kun"` (please set an identifiable string)
- To finalize: either `PATCH /.../{id}/stop` or `PATCH /.../{id}` with explicit `stop` and positive `duration` in seconds.
- Time zones: send explicit offsets (local or UTC). Toggl stores and renders in the user’s locale.

## Reports API (optional)
- Aggregated analytics (not required for core sync) live under the separate Reports API v3. Consider later for daily/weekly summaries.

References
- Reports API overview: https://engineering.toggl.com/docs/reports

## Security
- Use environment variables for credentials (`TOGGL_API_TOKEN`). Never commit tokens.
- Prefer HTTPS only; avoid logging sensitive headers.

---

## Quick Examples

Create a running entry (curl):

```bash
curl -s -u "$TOGGL_API_TOKEN:api_token" \
  -H 'Content-Type: application/json' \
  -d '{
    "description": "Write spec: ESD logic",
    "start": "2025-08-31T10:12:00+09:00",
    "duration": -1,
    "created_with": "chute_kun"
  }' \
  https://api.track.toggl.com/api/v9/workspaces/$TOGGL_WORKSPACE_ID/time_entries
```

Stop it later:

```bash
curl -s -u "$TOGGL_API_TOKEN:api_token" -X PATCH \
  https://api.track.toggl.com/api/v9/workspaces/$TOGGL_WORKSPACE_ID/time_entries/$ENTRY_ID/stop
```

