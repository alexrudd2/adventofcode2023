#!/bin/bash

# Check if an argument is provided
if [ "$#" -ne 1 ]; then
    echo "Enter a day!"
    exit 1
fi

day="$1"

FILE_TO_WATCH="./src/day${day}/mod.rs"
COMMAND_TO_RUN="cargo run ${day}"

while true; do
    inotifywait -e modify "$FILE_TO_WATCH" && $COMMAND_TO_RUN
done
