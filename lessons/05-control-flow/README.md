# Lesson 05: Control Flow

## Objective
Learn about conditional statements and loops in Rust.

## Concepts Covered
- `if`, `else if`, and `else` expressions
- Using `if` in `let` statements
- `loop` with `break` and return values
- `while` loops
- `for` loops with arrays and ranges

## Running the Lesson
```bash
cargo run
```

## Expected Output
```
=== Control Flow Demo ===

7 is greater than 5
Value from if expression: 5

Loop example:
Loop result: 6

While loop example:
3!
2!
1!
Liftoff!

For loop example:
Value: 10
Value: 20
Value: 30
Value: 40
Value: 50

For loop with range:
Number: 1
Number: 2
Number: 3
```

## Key Points
- `if` is an expression, not a statement, so it can return a value
- All branches of an `if` expression must return the same type
- `loop` creates an infinite loop; use `break` to exit
- `break` can return a value from a loop
- `while` loops run while a condition is true
- `for` loops iterate over collections or ranges
- Ranges are written as `start..end` (exclusive) or `start..=end` (inclusive)
