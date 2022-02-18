# 🦀`REULER`📏 

## Summary
Very [__Rusty__](https://www.rust-lang.org/fr) implementations of [Project Euler](https://projecteuler.net/) problem solutons.

Solving project euler problems in a (hopefully) idiomatic way.

## Features
- Dynamic problem loading (compiling only the requested problem)
- A list of solved problems in Rust
- Adding a new solution is as simple as creating a file in the problems folder

## Usage
Run this in the project directory (where the Cargo.toml file is located)
```shell
cargo run <problem_number>
```

Example with the third problem listed on [Project Euler](https://projecteuler.net/)
```shell
cargo run 3
```

```rust
>Opening problem (3)
Problem (3) "Largest prime factor" :
Output:
Factors = [71, 839, 1471, 6857] (Max = 6857)
```
## TODO
- Multiple solutions to one problem (`3.rs`, `3_a.rs`, `3_1.rs`, ...)
- Comment stuff :)
