# Sandbox Project

## Purpose
- Tutorial
- Prototype
- Debugging
- References

## Commands
```bash

cargo run # build and runs the main.rs in the project

cargo add # Adds the dependency/crates
```

## Cheat sheet
```rust

// Comments

// Formatted print

// Debug -- display

// Primitives
let my_int = 5i32; //i8, i16 .. i64, u64 unsighedm f32 floating
let my_char : char = 'r';
let my_bool = true;
// unit type
let my_unit = ();

// Compound Types

// Tuples
let my_tuple: (i32, f64, char) = (42, 3.14, 'x');
let (x, y, z) = my_tuple; // Destructuring
println!("Tuple values: {}, {}, {}", x, y, z);

// Arrays
let my_array: [i32; 3] = [1, 2, 3];
println!("Array element at index 0: {}", my_array[0]);



```
