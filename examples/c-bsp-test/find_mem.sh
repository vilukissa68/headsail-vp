#!/bin/bash

# Check if the correct number of arguments is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <start_path>"
    exit 1
fi

# Extract the starting path argument
start_path="$1"

# Find the mem_hpc file
file=$(find "$start_path" -type f -name "mem_hpc.x" | head -n 1)
# Check if the file exists
if [ -z "$file" ]; then
    echo "No mem_hpc.x file found in $start_path."
else
    echo "$(realpath "$file")"
fi
