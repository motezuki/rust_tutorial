# Lesson 04: Functions

## Objective
Learn how to define and use functions in Rust.

## Concepts Covered
- Function declaration with `fn` keyword
- Function parameters with type annotations
- Return types with `->` syntax
- Implicit returns (expressions without semicolons)
- Statements vs expressions
- Block expressions

## Running the Lesson
```bash
cargo run
```

## Expected Output
```
=== Functions Demo ===
Hello from a function!
Hello, Alice!
5 + 3 = 8
Calculate result: 25
Value from block expression: 4
```

## Key Points
- Functions are declared with `fn` keyword
- Parameter types must be explicitly declared
- Return type is specified after `->`
- The last expression in a function is automatically returned (no `return` keyword needed)
- Expressions evaluate to a value; statements do not
- Use semicolon to turn an expression into a statement
- Rust uses snake_case for function names
