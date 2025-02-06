# Rust Programming Exercises

This repository contains a series of Rust programming exercises designed to help you practice and master key concepts in the Rust programming language. Below are the instructions to set up your environment and the list of exercises.

---

## Setup Instructions

1. **Create a Rust File**  
   To get started, create a new Rust file for each exercise. For example:

   ```bash
   touch <filename>.rs
   ```

   Compile and run the file using the Rust compiler (`rustc`):

   ```bash
   rustc <filename>.rs
   ```

   This will generate an executable file named `<filename>` which you can run by typing `./<filename>` (or `fizzbuzz.exe` on Windows).

2. **Create a Cargo Project (Optional)**  
   If you prefer using Cargo (Rust's package manager and build system), initialize a new project:

   ```bash
   cargo new rust-exercises
   cd rust-exercises
   ```

   You can then place each exercise in the `src/bin/` directory as separate files (e.g., `fizzbuzz.rs`) and run them using:

   ```bash
   cargo run --bin fizzbuzz
   ```

---

## Exercises

### 1. FizzBuzz

**Task**: Print numbers from 1 to 100. For multiples of 3, print `"Fizz"`; for multiples of 5, print `"Buzz"`; for multiples of both, print `"FizzBuzz"`.  
**Concepts**: Loops (`for`), conditionals (`if/else`), modulus operator (`%`).

---

### 2. Reverse a String

**Task**: Take a string as input and return it reversed (e.g., `"hello"` → `"olleh"`).  
**Concepts**: String manipulation (`chars()`, `rev()`, `collect()`), iterators.

---

### 3. Factorial Calculator

**Task**: Compute the factorial of a number `n` (iteratively or recursively).  
**Concepts**: Loops vs. recursion, handling integer types (e.g., `u32` or `u64`).

---

### 4. Prime Number Checker

**Task**: Determine if a given number is prime. Optimize by checking divisors only up to √n.  
**Concepts**: Loops, math operations (`sqrt()`), edge cases (0, 1, 2).

---

### 5. Sum of Vector Elements

**Task**: Calculate the sum of all elements in a `Vec<i32>`.  
**Concepts**: Vectors, iterators, and the `sum()` method.

---

### 6. Guess the Number Game

**Task**: Generate a random number between 1 and 100 and let the user guess it with feedback ("too high" or "too low").  
**Concepts**: External crates (`rand`), input handling (`std::io`), loops, error handling (`Result`).

---

### 7. Fibonacci Sequence

**Task**: Generate the first `n` Fibonacci numbers (e.g., `0, 1, 1, 2, 3, 5...`).  
**Concepts**: Loops, mutable variables, tuples for swapping values.

---

### 8. Palindrome Checker

**Task**: Check if a string reads the same backward (ignoring case and non-alphanumeric characters, e.g., `"Racecar"`).  
**Concepts**: String methods (`to_lowercase()`, `chars()`), filtering, comparison.

---

### 9. Temperature Converter

**Task**: Convert Celsius to Fahrenheit and vice versa using functions (formulas: °F = °C * 1.8 + 32 and °C = (°F - 32) / 1.8).  
**Concepts**: Functions, floating-point arithmetic (`f32/f64`).

---

### 10. Find Maximum in a List

**Task**: Find the largest number in a `Vec<i32>`. Implement manually or use iterators.  
**Concepts**: Iterators (`iter()`), `max()` method, or manual comparison with loops.

---

Happy coding! 🚀
