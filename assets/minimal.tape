# Run from the repository root after cargo build --release --locked.
# Requires VHS and its recording dependencies, including FFmpeg.
# Explicit durations keep the demo independent of saved timer preferences.
Output assets/minimal.gif

Set Shell "bash"
Set FontSize 20
Set Width 900
Set Height 170
Set Padding 20
Set Framerate 10
Set TypingSpeed 35ms
Set Theme "Catppuccin Mocha"

Hide
Type "export PS1='$ ' PS2='> '; export PATH=$PWD/target/release:$PATH; clear"
Enter
Sleep 500ms
Show
Type "tomatui start -m -w 25 -b 5 -l 15 -s 4"
Enter
Sleep 3s
Type "p"
Sleep 2s
Type "p"
Sleep 3s
Type "q"
Sleep 1s
