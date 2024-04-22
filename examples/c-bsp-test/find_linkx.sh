#!/bin/bash

# Check if the correct number of arguments is provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <start_path> <riscv_architecture>"
    exit 1
fi

# Extract the starting path and architecture arguments
start_path="$1"
architecture="$2"

# Check if the provided architecture is valid
if [ "$architecture" != "riscv64" ] && [ "$architecture" != "riscv32" ]; then
    echo "Invalid architecture. Please provide either 'riscv64' or 'riscv32'."
    exit 1
fi

# Find the corresponding link.x file based on the architecture
file=$(find "$start_path" -type f -name "link.x" | grep "$architecture" | head -1)

# Check if the file exists
if [ -z "$file" ]; then
    echo "No $architecture link.x file found in $start_path."
else
    echo "$(realpath "$file")"
fi

