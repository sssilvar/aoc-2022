# Advent of Code 2022 (aoc-2022)

This repository contains my solutions for the Advent of Code 2022 challenges, implemented in Rust. https://adventofcode.com/2022/

## Project Structure

Each day of the Advent of Code is implemented as a separate Rust module under its own directory (e.g., `day_001`, `day_002`, etc.), with the following structure:

```
.
├── Cargo.lock
├── Cargo.toml                # Root Cargo file for the entire project
├── day_001/                  # Day 1 challenge solution
│   ├── Cargo.toml            # Cargo file for the binary target of day 1
│   ├── data/
│   │   └── input.txt         # Input data for the day 1 challenge
│   └── src/
│       ├── bin/
│       │   └── main.rs       # Main binary source file for day 1
│       └── lib.rs            # Optional library file for shared functions
.
.
.

├── template/                 # Template for generating new days
│   ├── Cargo.toml
│   ├── data/
│   │   └── input.txt
│   └── src/
│       ├── bin/
│       │   └── main.rs
│       └── lib.rs
└── target/                   # Compilation artifacts and build cache
```

Each day is structured as its own binary crate within the workspace, and all solutions are built and tested as separate binary targets. You can use the template provided under `template/` to generate a new day's module.

## Commands

Here are some useful `cargo` commands, with aliases included for convenience.

### Generating a New Day Module

To generate a new module for a specific day:

```bash
cargo gen day_00X
```

This will create a new module under the appropriate directory (e.g., `day_003`), with the basic structure set up. You can customize the template in the `template/` directory.

### Running Tests

To run tests for a specific day:

```bash
cargo test -p day_00X
```

This will execute all the tests available for that day’s solution, ensuring the code works correctly.

### Running the Solution

To run the solution for a specific day:

```bash
cargo run -p day_00X
```

This will execute the binary target associated with that day and print the output for the day’s challenge.

### Opening Documentation

To generate and open the project’s documentation in your default browser:

```bash
cargo guide
```

This will generate the documentation for the project using Rust’s built-in documentation tool (`cargo doc`) and open it in your browser.

## Example Workflow

For example, if you want to generate a module for Day 5, run its tests, and then run the solution, the commands would look like:

```bash
cargo gen day_005    # Generate module for Day 5
cargo test -p day_005  # Run tests for Day 5
cargo run -p day_005   # Run the solution for Day 5 (prints output)
```

## Contributions

Feel free to submit pull requests or issues if you'd like to improve or add to the solutions. I’m always open to discussions about optimization, better practices, or different approaches to solving the puzzles.
