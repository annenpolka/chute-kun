# Configuration (`config.toml`)

Chute_kun loads configuration from the following path on startup and during UI rendering:

- `$CHUTE_KUN_CONFIG_DIR/config.toml` if set (useful in tests or portable setups)
- Otherwise `$XDG_CONFIG_HOME/chute_kun/config.toml`
- Otherwise `$HOME/.config/chute_kun/config.toml`

If the file is missing, sensible defaults are used.

## CLI Helpers

- `chute_kun --write-default-config` (alias: `init-config`): writes a default `config.toml` to the resolved config directory, without overwriting if it already exists. Prints the path and exits.

## Keys

You can customize keybindings. Values may be single characters (use `"space"` for space) or special names: `enter`, `shift+enter`, `tab`, `backtab`.

```toml
# Default start-of-day used for schedule display and ESD baseline.
start_of_day = "09:00"

[keybindings]
quit = "q"          # exit app
interrupt = "i"     # add + start 15m Interrupt
pause = "space"     # pause active task
inc_estimate = "e"  # +5m estimate on selected
postpone = "p"      # move selected to Tomorrow
select_up = "k"     # move cursor up
select_down = "j"   # move cursor down
reorder_up = "["    # move selected up
reorder_down = "]"  # move selected down
# Special bindings (default shown)
start = "enter"         # start/resume selected
finish = "shift+enter"  # finish active (and move to History)
view_next = "tab"       # switch view: Past -> Today -> Future
view_prev = "backtab"   # switch view backwards (aka Shift+Tab)
```

Notes:
- Arrow keys remain active regardless of custom `select_up`/`select_down`.
- The application reads the config when handling input and rendering, so edits take effect on next redraw.
