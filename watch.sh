#!/bin/bash

FILE_TO_WATCH="./src/day17/mod.rs"
COMMAND_TO_RUN="cargo run"

while true; do
    inotifywait -e modify "$FILE_TO_WATCH" && clear && $COMMAND_TO_RUN
done
