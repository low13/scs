#!/bin/bash

mkdir -p "out/commands"
cargo build --release
mv "target/release/scs.exe" "out"
cd "commands"

for cmd in *; do
    (
        cd "$cmd"
        if [ "$cmd" = "mv" ]; then
            cmd="move"
        fi
        cargo build --release
        mv "target/release/$cmd.exe" "../../out/commands"
    )
done