# Fictional fixture rendered by the same code as tomatui stats history.
# Run cargo build --locked --example stats_preview first.
Output "/tmp/tomatui-stats-preview.gif"
Set Shell "bash"
Set FontSize 20
Set Width 950
Set Height 450
Set Padding 24
Set Theme "Catppuccin Mocha"
Set CursorBlink false
Env NO_COLOR ""
Env COLORTERM "truecolor"
Hide
Type "PS1='$ '; clear; ./target/debug/examples/stats_preview"
Enter
Sleep 1s
Show
Sleep 1s
Screenshot assets/stats.png
Sleep 1s
