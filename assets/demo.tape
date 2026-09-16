# Real one-minute phases, played at 4x. Work is switched before completion.
Output target/recordings/demo.mp4
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
Set TypingSpeed 240ms
Set PlaybackSpeed 4
Env NO_COLOR ""
Env COLORTERM "truecolor"
Hide
Type@1ms "alias tomatui=./target/debug/tomatui; PS1='$ '; clear"
Enter
Sleep 500ms
Show
Sleep 2s
Type "tomatui"
Sleep 4s
Enter
Sleep 28s
Screenshot assets/work.png
Sleep 1s
Type "p"
Sleep 4s
Type "p"
Sleep 4s
Type "+"
Sleep 4s
Type "b"
Sleep 28s
Screenshot assets/break.png
Sleep 35s
Sleep 4s
Enter
Sleep 3s
Hide
# Skip the intervening phases to show the long break at session four.
Type "s"
Sleep 250ms
Type "s"
Sleep 250ms
Type "s"
Sleep 250ms
Type "s"
Sleep 250ms
Type "s"
Sleep 250ms
Show
Sleep 4s
Screenshot assets/long_break.png
Sleep 1s
Hide
Type "q"
Sleep 500ms
