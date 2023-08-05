#!/bin/bash

mkdir -p "out"
cargo build --release
mv "target/release/scs.exe" "out" 2>/dev/null
cd "commands"

for cmd in *; do
    (
        cd "$cmd"
        if [ "$cmd" = "mv" ]; then
            cmd="move"
        fi
        cargo build --release
        mv "target/release/$cmd.exe" "../../out" 2>/dev/null
    )
done