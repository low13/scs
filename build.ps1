mkdir -Force "out/commands"
cargo build --release
Move-Item "target\release\scs.exe" "out"
cd "commands"

Get-ChildItem -Directory | ForEach-Object {
    $cmd = $_.Name

    cd $cmd
    cargo build --release

    if ($cmd -eq "mv") {
        Move-Item "target\release\move.exe" "..\..\out\commands"
    } else {
        Move-Item "target\release\$cmd.exe" "..\..\out\commands"
    }
    cd ..
}