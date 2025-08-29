# Configuration (`config.toml`)

Chute_kun loads configuration from the following path on startup and during UI rendering:

- `$CHUTE_KUN_CONFIG_DIR/config.toml` if set (useful in tests or portable setups)
- Otherwise `$XDG_CONFIG_HOME/chute_kun/config.toml`
- Otherwise `$HOME/.config/chute_kun/config.toml`

If the file is missing, sensible defaults are used.

## CLI Helpers

- `chute_kun --write-default-config` (alias: `init-config`): writes a default `config.toml` to the resolved config directory, without overwriting if it already exists. Prints the path and exits.

## Keys

You can customize a subset of keybindings. Values are single characters; use `"space"` (or `"SPC"`) for the space key.

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
```

Notes:
- Arrow keys, `Tab`/`Shift-Tab`, and `Enter`/`Shift+Enter` are currently fixed.
- The application reads the config when handling input and rendering, so edits take effect on next redraw.
