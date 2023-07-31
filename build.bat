@echo off

mkdir "out\commands"
cargo build --release
move "target\release\scs.exe" "out\"
cd "commands"

for /D %%i in (*) do (
    cd "%%i"
    cargo build --release

    if /I "%%i"=="mv" (
        move "target\release\move.exe" "..\..\out\commands\"
    ) else (
        move "target\release\%%i.exe" "..\..\out\commands\"
    )

    cd ..
)