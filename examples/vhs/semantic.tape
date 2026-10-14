# A VHS tape. See https://github.com/charmbracelet/vhs

# for running against the tui-nuances git repo https://github.com/sorinirimies/tui-nuances
Output "examples/vhs/output/semantic.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 700
Set FontSize 22
Set Padding 20
Set PlaybackSpeed 1

Hide
Type "clear && cargo run --release --example semantic"
Enter
Sleep 5s
Show

Sleep 1s
Type "j"
Sleep 0.5s
Type "j"
Sleep 0.5s
Type "j"
Sleep 0.5s
Tab
Sleep 0.8s
Tab
Sleep 0.5s
Type "t"
Sleep 0.9s
Type "t"
Sleep 0.9s
Type "t"
Sleep 0.9s
Type "t"
Sleep 0.9s
Type "t"
Sleep 0.9s
Type "t"
Sleep 0.9s

Hide
Type "q"
Sleep 0.3s
