Learn Rust
==========

This repo contains code related to my attempts to learn [Rust](https://www.rust-lang.org/).
Specifically it contains my solutions to [Project Euler](http://projecteuler.net/) problems in Rust,

This repo should be private to honor the ethos of Project Euler, but I'm too much of a
cheapskate to pay for a private github account, so I am relying on any readers who stumble
upon this repo to avoid looking at solutions to problems you have not already solved yourself.

## Usage

To print all solutions and the time to compute:
```
cargo run --release all
```

To print the solution and compute time for a specifc problem, for example 4:
```
cargo run --release 4
```

To check and see if the code for a specific problem yields the correct answer,
for example problem 3:
```
cargo run 3
```

To print diagnostics during development, first modify the function run_test() in main.rs, then:
```
cargo run
```

To run all the test routines:
```
cargo test
```

To run only the test routines for a single problem,
for example problem 2 (the string 002 is a unique selector for the tests in problem002.rs):
```
cargo test 002
```
