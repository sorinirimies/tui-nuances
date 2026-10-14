# A VHS tape. See https://github.com/charmbracelet/vhs

# for running against the tui-nuances git repo https://github.com/sorinirimies/tui-nuances
Output "examples/vhs/output/switcher.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 700
Set FontSize 22
Set Padding 20
Set PlaybackSpeed 1

Hide
Type "cargo run --release --example switcher"
Enter
Sleep 5s
Show

Sleep 1s
Type "n"
Sleep 0.8s
Type "n"
Sleep 0.8s
Type "n"
Sleep 0.8s
Type "n"
Sleep 0.8s
Type "c"
Sleep 1.2s
Type "c"
Sleep 1.2s
Type "c"
Sleep 1.2s
Type "c"
Sleep 1s
Type "p"
Sleep 0.8s
Type "p"
Sleep 1s

Hide
Type "q"
Sleep 0.3s
