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
finish_active = ["Shift+Enter", "f"]
pause = "Space"
delete = "x"
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
- 変更（予定基準時間）: `chute_kun --set-day-start HH:MM` または `chute_kun --set-day-start HHMM`
  - `CHUTE_KUN_CONFIG` が設定されていればそのパスを、なければ既定の config.toml を作成/更新します。
- 実行時: ファイルが存在すれば自動読み込み。存在しない場合はデフォルト（09:00 と既定キー）。

**TUI での変更（コマンドパレット）**
- `:` でコマンドパレットを開き、`base HH:MM` または `base HHMM` を入力して Enter。
  - 例: `base 10:30` / `base 1030`
  - 変更は `config.toml` にも保存され、次回起動以降も有効です。

**注意**
- 入力モード中の文字入力はテキスト編集が優先され、カスタムキーは適用されません（Enter/Esc/Backspace/文字）。
- `Shift+Enter` と `Enter` のように修飾の有無は区別されます。

**ヘルプ行への反映**
- 画面下部のヘルプは `keys` セクションの設定をそのまま表示に反映します。
  - 複数割り当ては `/` 区切り（例: `Shift+Enter/f: finish`）。
  - 修飾キーは `Shift`/`Ctrl`/`Alt` の順で表示。
- `Ctrl+<letter>` は設定の大文字小文字を区別せず、表示は小文字（例: `Ctrl+d`）。
- `BackTab` は表示上 `Shift+Tab` としてレンダリングされます（設定では `BackTab` と `Shift+Tab` のどちらも指定可能）。
- Delete も `keys.delete` で変更可能（例: `delete = "Ctrl+d"`）。
