# Toggl Track Setup

This guide shows how to prepare credentials for the optional Toggl Track sync.

## 1) Get your API Token
1. Open Toggl Track web → Profile → API Token
2. Copy the token (keep it secret)

## 2) Create a local `.env`
1. Copy `.env.example` to `.env`
2. Fill `TOGGL_API_TOKEN=...`
3. Optionally set `TOGGL_WORKSPACE_ID`, `TOGGL_PROJECT_ID_DEFAULT`, `TOGGL_TAGS_DEFAULT`

Do not commit `.env` files. Tokens must not be stored in `snapshot.toml`.

## 3) Verify connectivity (manual)
Using curl:

```bash
source .env
curl -s -u "$TOGGL_API_TOKEN:api_token" https://api.track.toggl.com/api/v9/me | jq .
```

You should see your profile JSON (including `default_workspace_id`).

## Notes
- The app will pace requests to respect API limits. If you experiment with scripts, prefer ≤ 1 req/sec and backoff on 429.
- If your organization enforces quotas, `GET /api/v9/me/quota` reveals current limits and counters.

