@echo off

mkdir "out"
cargo build --release
move "target\release\scs.exe" "out\"
cd "commands"

for /D %%i in (*) do (
    cd "%%i"
    cargo build --release

    if /I "%%i"=="mv" (
        move "target\release\move.exe" "..\..\out\"
    ) else (
        move "target\release\%%i.exe" "..\..\out\"
    )

    cd ..
)

cd ..