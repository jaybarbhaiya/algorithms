# Makefile for running all benchmarks with Criterion and saving baselines

# Get the current branch name
BRANCH_NAME := $(shell git rev-parse --abbrev-ref HEAD)

# Get all benchmark names
BENCHMARKS := $(shell cargo bench -- --list | grep -oP '^[a-zA-Z0-9_]+(?=: benchmark)')

# Define a phony target to run all benchmarks
.PHONY: run-benchmarks

run-benchmarks:
	@for bench in $(BENCHMARKS); do \
		cargo bench --bench $$bench -- --save-baseline $(BRANCH_NAME); \
	done
