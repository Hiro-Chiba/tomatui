Output "/tmp/tomatui-long-break-preview.gif"
Set Shell "bash"
Set FontSize 32
Set FontFamily "FiraCode Nerd Font Mono"
Set Width 1600
Set Padding 44
Set Margin 48
Set MarginFill "#070B12"
Set BorderRadius 20
Set WindowBar Colorful
Set Framerate 30
Set CursorBlink false
Set Theme "Catppuccin Mocha"
Set Height 1000
Env NO_COLOR ""
Env COLORTERM "truecolor"
Hide
Type "./target/debug/tomatui start -w 25 -b 5 -l 15 -s 1 --on-end ask"
Enter
Sleep 500ms
Type "b"
Sleep 500ms
Show
Sleep 1s
Screenshot assets/long_break.png
Sleep 1s
Hide
Type "q"
