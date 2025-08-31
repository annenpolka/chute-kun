# State Path Resolution (snapshot.toml)

This document defines how Chute‑kun resolves the snapshot save/load path.

- Field: `state_path` in `config.toml` (top level)
- Applies to: CLI/TUI (`chute`)

## Precedence

1. `config.toml` → `state_path` (highest)
2. CLI flag → `--state <file>`
3. Environment → `CHUTE_KUN_STATE`
4. Defaults (XDG‑style):
   - `$XDG_DATA_HOME/chute_kun/snapshot.toml`
   - `~/.local/share/chute_kun/snapshot.toml`
   - `dirs::data_dir()/chute_kun/snapshot.toml`

## Expansion Rules (config only)

- Supported in `state_path`:
  - Leading `~` → `$HOME`
  - Whitelisted `${VAR}`: `HOME`, `XDG_DATA_HOME`, `XDG_STATE_HOME`, `XDG_CONFIG_HOME`
- Unknown or unset `${VAR}`: the `state_path` is ignored (falls back to the next precedence level).
- The expanded path must be absolute; otherwise it is ignored.
- No shell execution is performed.

## Examples

```toml
# Absolute
state_path = "/data/chute/snapshot.toml"

# Whitelisted env
state_path = "${XDG_DATA_HOME}/chute_kun/snapshot.toml"

# Home expansion
state_path = "~/.local/share/chute_kun/snapshot.toml"
```

## Notes

- Only `config.toml`’s `state_path` supports expansion. CLI/env values are used as‑is.
- If you need a custom variable, export it to one of the whitelisted XDG vars or set `--state`/`CHUTE_KUN_STATE` instead.
