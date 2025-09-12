//! # Rust Control Flow Showcase
//!
//! This file demonstrates the use of `if`, `else if`, and `else`
//! expressions for controlling the flow of execution in Rust.

fn main() {
    println!("## Part 1: Basic `if-else` Expressions ##");

    let number = 3;

    // An `if` expression allows you to branch your code.
    // The block of code is executed if the condition is true.
    if number < 5 {
        println!("The condition `number < 5` was true."); // This will print.
    } else {
        println!("The condition `number < 5` was false.");
    }

    // In Rust, the condition for an `if` expression MUST be a bool.
    // Unlike some languages, Rust will not automatically convert non-Booleans.
    // For example, the following code would cause a compile error:
    /*
        if number {
            // ERROR: mismatched types, expected `bool`, found integer
        }
    */

    // To check if a number is non-zero, you must be explicit.
    if number != 0 {
        println!("The number was something other than zero.");
    }

    println!("\n-------------------------------------------------\n");
    println!("## Part 2: Handling Multiple Conditions with `else if` ##");

    let another_number = 6;

    // You can handle multiple conditions using `else if`.
    // Rust executes the block for the *first* true condition it finds
    // and then skips the rest.
    if another_number % 4 == 0 {
        println!("The number {} is divisible by 4.", another_number);
    } else if another_number % 3 == 0 {
        // This block will be executed because 6 % 3 == 0 is the first true condition.
        println!("The number {} is divisible by 3.", another_number);
    } else if another_number % 2 == 0 {
        println!("The number {} is divisible by 2.", another_number);
    } else {
        println!("The number {} is not divisible by 4, 3, or 2.", another_number);
    }

    println!("\n-------------------------------------------------\n");
    println!("## Part 3: Using `if` in a `let` Statement ##");

    let condition = true;

    // Because `if` is an expression, you can use it on the right side of a `let` statement.
    let assigned_number = if condition { 5 } else { 6 };
    println!("The value assigned from the `if` expression is: {}", assigned_number);

    // IMPORTANT: The values from each arm of the `if` expression must be the same type.
    // Variables must have a single, known type at compile time.
    // The following code would fail to compile because the `if` and `else`
    // arms have incompatible types (integer and &str).
    /*
        let invalid_assignment = if condition { 5 } else { "six" };
        // ERROR: `if` and `else` have incompatible types
    */
}