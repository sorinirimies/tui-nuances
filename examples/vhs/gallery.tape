# A VHS tape. See https://github.com/charmbracelet/vhs

# for running against the tui-nuances git repo https://github.com/sorinirimies/tui-nuances
Output "examples/vhs/output/gallery.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 700
Set FontSize 22
Set Padding 20
Set PlaybackSpeed 1

Hide
Type "clear && cargo run --release --example gallery"
Enter
Sleep 5s
Show

Sleep 1s
Type "t"
Sleep 0.8s
Type "t"
Sleep 0.8s
Type "t"
Sleep 0.8s
Type "t"
Sleep 0.8s
Type "t"
Sleep 0.8s
Type "t"
Sleep 0.8s
Type "t"
Sleep 0.8s
Type "t"
Sleep 0.8s
Type "T"
Sleep 0.8s
Type "T"
Sleep 1s

Hide
Type "q"
Sleep 0.3s
