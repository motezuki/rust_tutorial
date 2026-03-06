# Lesson 03: Data Types

## Objective
Learn about Rust's scalar and compound data types.

## Concepts Covered
- Scalar types: integers, floating-point numbers, booleans, characters
- Compound types: tuples and arrays
- Type annotations
- String slices

## Running the Lesson
```bash
cargo run
```

## Expected Output
```
Integer: 42
Float: 3.14
Boolean: true
Character: A, Emoji: 😀
Tuple: (500, 6.4, x)
Array: [1, 2, 3, 4, 5]
First element: 1
String slice: Hello, Rust!
```

## Key Points
- Rust has four primary scalar types: integers, floating-point, booleans, and characters
- Integers can be signed (i8, i16, i32, i64, i128) or unsigned (u8, u16, u32, u64, u128)
- Floating-point types are f32 and f64 (default)
- `char` type represents a Unicode scalar value (4 bytes)
- Tuples can group values of different types
- Arrays have fixed length and all elements must be the same type
