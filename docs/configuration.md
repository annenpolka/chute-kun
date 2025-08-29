# Configuration (config.toml)

Chute_kun reads an optional TOML config from the XDG config directory:

- Path: `$XDG_CONFIG_HOME/chute_kun/config.toml` (fallback: `$HOME/.config/chute_kun/config.toml`)

## Options

- `start_of_day` (string HH:MM): When set, task list time prefixes start from this minute instead of the current time. The ESD header still uses the real current time.
- `[keys]` table: Override key bindings by action name. Values accept characters (e.g., `'q'`, `']'`) or names like `"enter"`, `"space"`, `"tab"`, `"backtab"`, `"up"`, `"down"`. `"shift+enter"` is supported.

### Supported actions

- `quit`
- `interrupt`
- `start_resume`
- `pause`
- `reorder_down`
- `reorder_up`
- `increase_estimate`
- `postpone`
- `next_view`
- `prev_view`
- `select_up`
- `select_down`
- `finish_active`

## Example

```toml
start_of_day = "09:00"

[keys]
quit = 'x'
interrupt = 'i'
start_resume = 'enter'
pause = 'space'
reorder_down = ']'
reorder_up = '['
increase_estimate = 'e'
postpone = 'p'
next_view = 'tab'
prev_view = 'backtab'
select_up = 'k'
select_down = 'j'
finish_active = 'shift+enter'
```

