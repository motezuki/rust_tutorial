# rust_tutorial
Repository of experiments in learning Rust programming

## Structure

This repository contains independent lessons for learning Rust programming. Each lesson is a standalone Cargo project that can be compiled and run independently.

### Lessons

The `lessons/` directory contains numbered lessons covering fundamental Rust concepts:

1. **[01-hello-world](lessons/01-hello-world/)** - Your first Rust program
   - Basic program structure
   - The `main` function
   - Using `println!` macro

2. **[02-variables](lessons/02-variables/)** - Variables and mutability
   - Immutable vs mutable variables
   - Variable shadowing
   - Constants

3. **[03-data-types](lessons/03-data-types/)** - Rust's type system
   - Scalar types (integers, floats, booleans, characters)
   - Compound types (tuples, arrays)
   - Type annotations

4. **[04-functions](lessons/04-functions/)** - Defining and using functions
   - Function parameters
   - Return values
   - Expressions vs statements

5. **[05-control-flow](lessons/05-control-flow/)** - Conditional logic and loops
   - `if` expressions
   - `loop`, `while`, and `for` loops
   - Loop control with `break` and `continue`

### Running a Lesson

Navigate to any lesson directory and run:

```bash
cd lessons/01-hello-world
cargo run
```

Each lesson includes:
- `Cargo.toml` - Project configuration
- `src/main.rs` - Lesson code
- `README.md` - Lesson explanation and learning objectives

## Getting Started

1. Make sure you have [Rust installed](https://www.rust-lang.org/tools/install)
2. Clone this repository
3. Navigate to any lesson directory
4. Run `cargo run` to see the lesson in action
5. Read the lesson's README.md for detailed explanations
6. Modify the code to experiment and learn!
