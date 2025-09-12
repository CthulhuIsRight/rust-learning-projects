//! # Rust Repetition Showcase with Practical Example
//!
//! This file demonstrates Rust's loop constructs (`loop`, `while`, `for`)
//! and includes a practical example of calculating a large Fibonacci number.
//!
//! NOTE: To run this code, you must add the required dependencies to `Cargo.toml`.

// Imports needed for the Fibonacci example
use std::time::Instant;
use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Computes the nth Fibonacci number iteratively using BigUint.
/// This function is defined outside of main() so it can be called from within it.
fn fibonacci_big(n: u64) -> BigUint {
    if n == 0 {
        return Zero::zero();
    } else if n == 1 {
        return One::one();
    }

    let mut a: BigUint = Zero::zero();
    let mut b: BigUint = One::one();

    // The for loop here is a core part of the algorithm
    for _ in 2..=n {
        let c = &a + &b;
        a = b;
        b = c;
    }
    b
}

fn main() {
    println!("## Part 1: The `loop` Keyword ##");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2;
        }
    };
    println!("The result returned from the loop is: {}", result);

    // ---
    println!("\n-------------------------------------------------\n");
    println!("## Part 2: Loop Labels for Nested Loops ##");

    let mut count = 0;
    'counting_up: loop {
        println!("Outer loop count = {count}");
        let mut remaining = 10;
        loop {
            println!("  Inner loop remaining = {remaining}");
            if remaining == 9 {
                break; // Exits the inner loop
            }
            if count == 2 {
                break 'counting_up; // Exits the outer 'counting_up loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count after labeled break = {count}");

    // ---
    println!("\n-------------------------------------------------\n");
    println!("## Part 3: Conditional Loops with `while` ##");

    let mut countdown = 3;
    while countdown != 0 {
        println!("{countdown}!");
        countdown -= 1;
    }
    println!("LIFTOFF!!!");

    // ---
    println!("\n-------------------------------------------------\n");
    println!("## Part 4: Looping Through Collections with `for` ##");

    let a = [10, 20, 30, 40, 50];
    println!("Iterating over an array with a `for` loop:");
    for element in a {
        println!("  The value is: {element}");
    }

    println!("Countdown using a `for` loop and a reversed Range:");
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    // ---
    println!("\n-------------------------------------------------\n");
    println!("## Part 5: Practical Example - Fibonacci Sequence ##");

    // Using a smaller N so the example runs instantly.
    let n = 500u64;
    println!("Calculating the {}th Fibonacci number...", n);

    let start = Instant::now();
    let fib_n = fibonacci_big(n);
    let duration = start.elapsed();

    println!("Fibonacci({}) computed in {:?}", n, duration);

    // Print only the last 20 digits to avoid flooding the terminal.
    let fib_str = fib_n.to_str_radix(10);
    let last_digits = if fib_str.len() > 20 {
        &fib_str[fib_str.len() - 20..]
    } else {
        &fib_str[..]
    };
    println!("Last 20 digits: ...{}", last_digits);
}