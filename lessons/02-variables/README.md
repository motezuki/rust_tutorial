# Lesson 02: Variables

## Objective
Understand variables, mutability, shadowing, and constants in Rust.

## Concepts Covered
- Immutable variables (default)
- Mutable variables with `mut` keyword
- Variable shadowing
- Constants with `const` keyword

## Running the Lesson
```bash
cargo run
```

## Expected Output
```
The value of x is: 5
The value of y is: 10
The new value of y is: 15
The value of z is: 20
The shadowed value of z is: 25
The maximum points are: 100000
```

## Key Points
- Variables are immutable by default in Rust
- Use `mut` keyword to make a variable mutable
- Shadowing allows reusing variable names with different types or values
- Constants must have type annotations and are always immutable
- Constants use UPPER_SNAKE_CASE naming convention
