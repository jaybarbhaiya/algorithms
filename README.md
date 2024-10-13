# Introduction

This repository contains implementations of `Algorithms` in Rust.

The aim is to practice and learn about Rust.

## Implemented Algorithms

1. [Bubble Sort](https://www.w3schools.com/dsa/dsa_algo_bubblesort.php)

## Benchmark

- Commandline options:
  - checkout a branch
  
    ```console
    git checkout main
    ```

  - The below command creates a benchmark without overwriting existing ones.
  
    ```console
    cargo bench --bench bubble_sort -- --save-baseline <name>
    ```

  - Loading and comparing baselines   
  
    ```console
    cargo bench --bench bubble_sort -- --load-baseline new --baseline <name>
    ```
