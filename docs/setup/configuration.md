**概要**
- **場所**: `$XDG_CONFIG_HOME/chute_kun/config.toml`。未設定の場合、macOS/Linux では `~/.config/chute_kun/config.toml`（macOS も ~/Library ではなく ~/.config を使用）。`CHUTE_KUN_CONFIG` でパス上書き。
- **目的**: 1日の開始時刻やキーバインドをユーザー側で調整。
- **生成**: `chute_kun --init-config` で雛形を書き出し（既存ファイルは保持）。

**基本設定**
- **day_start**: 固定表示の開始時刻（`"HH:MM"`）。デフォルトは `"09:00"`。
- **keys**: 既定キーバインドの上書き。単一文字はそのまま、特殊キーは `Enter`/`Space`/`Tab`/`BackTab`/`Up`/`Down`、修飾は `Shift+...` 等。

**例: 既定の config.toml**
- day_start と各キーは必要なものだけ上書き可能。
- 複数割当は配列で指定。

```
# Chute_kun configuration
# 設定ファイルの場所: $XDG_CONFIG_HOME/chute_kun/config.toml （なければ ~/.config/chute_kun/config.toml）

day_start = "09:00"

[keys]
quit = "q"
add_task = "i"
add_interrupt = "I"
start_or_resume = "Enter"
finish_active = "Shift+Enter"
pause = "Space"
reorder_up = "["
reorder_down = "]"
estimate_plus = "e"
postpone = "p"
bring_to_today = "b"
view_next = "Tab"
view_prev = "BackTab"
select_up = ["Up", "k"]
select_down = ["Down", "j"]
```

**使い方**
- 初期化: `chute_kun --init-config`（または `CHUTE_KUN_CONFIG=/path/to/config.toml chute_kun --init-config`）。
- 実行時: ファイルが存在すれば自動読み込み。存在しない場合はデフォルト（09:00 と既定キー）。

**注意**
- 入力モード中の文字入力はテキスト編集が優先され、カスタムキーは適用されません（Enter/Esc/Backspace/文字）。
- `Shift+Enter` と `Enter` のように修飾の有無は区別されます。
