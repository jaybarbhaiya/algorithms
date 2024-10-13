#!/bin/bash

# Get the current branch name
branch=$(git rev-parse --abbrev-ref HEAD)
echo "$branch"

# Run cargo bench -- --list and capture the output
output=$(cargo bench -- --list)

# Extract benchmark names from the output
benchmarks=$(echo "$output" | grep -oP '^[a-zA-Z0-9_]+(?=: benchmark)')

# Run each benchmark with --save-baseline
for bench in $benchmarks; do
    cargo bench --bench $bench -- --save-baseline $branch
done
