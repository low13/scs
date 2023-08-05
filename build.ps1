New-Item -ItemType Directory -Force "out"
cargo build --release
Move-Item "target\release\scs.exe" "out"
Set-Location "commands"

Get-ChildItem -Directory | ForEach-Object {
    $cmd = $_.Name

    Set-Location $cmd
    cargo build --release

    if ($cmd -eq "mv") {
        Move-Item "target\release\move.exe" "..\..\out"
    } else {
        Move-Item "target\release\$cmd.exe" "..\..\out"
    }
    Set-Location ..
}

Set-Location ..