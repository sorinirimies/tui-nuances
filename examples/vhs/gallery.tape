# A VHS tape. See https://github.com/charmbracelet/vhs

# for running against the tui-nuances git repo https://github.com/sorinirimies/tui-nuances
Output "examples/vhs/output/gallery.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 700
Set FontSize 18
Set Padding 20
Set PlaybackSpeed 1

Hide
Type "clear && cargo run --release --example gallery"
Enter
Sleep 3s
Show

Sleep 1s
Type "n"
Sleep 2.5s
Type "n"
Sleep 2.5s
Type "n"
Sleep 2.5s
Type "n"
Sleep 2.5s
Type "n"
Sleep 2.5s

Hide
Type "q"
Sleep 0.3s
