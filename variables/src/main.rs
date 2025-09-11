//! # Rust Concepts Showcase
//!
//! This file demonstrates fundamental concepts in Rust,
//! including variables, data types, functions, and comments.

// A global constant. Its type MUST be annotated and it's always immutable.
const SECONDS_IN_MINUTE: u32 = 60;

/**
 * The main entry point of the program.
 * Most concepts are demonstrated within this function.
 */
fn main() {
    println!("--- Variables, Mutability, and Shadowing ---");

    // By default, variables are immutable.
    let an_immutable_variable = 5;
    println!("The immutable variable is: {}", an_immutable_variable);

    // You can make a variable mutable by adding `mut`.
    let mut a_mutable_variable = 5;
    println!("The mutable variable was: {}", a_mutable_variable);
    a_mutable_variable = 6; // This is allowed because it's mutable.
    println!("Now the mutable variable is: {}", a_mutable_variable);

    // Shadowing lets you declare a new variable with the same name.
    let x = 10;
    let x = x + 1; // The first `x` is shadowed by this new `x`.
    {
        // This shadowing only applies within this inner scope.
        let x = x * 2;
        println!("x in the inner scope is: {}", x);
    }
    println!("x is back to its outer scope value: {}", x);

    // Shadowing can also change the variable's type.
    let spaces = "   "; // `spaces` is a string slice (&str)
    let spaces = spaces.len(); // Now `spaces` is a number (usize).
    println!("There are {} spaces.", spaces);

    // --- Scalar and Compound Data Types ---
    println!("\n--- Data Types ---");

    // Scalar types represent a single value.
    let _an_integer: i32 = -50; // Explicit type annotation.
    let _an_unsigned_integer: u32 = 50; // Unsigned integer.
    let _large_number = 1_000_000; // `_` is a visual separator.
    let _float_64 = 2.0; // The default float type is f64.
    let _float_32: f32 = 3.0; // Explicitly an f32.
    let _is_learning_rust: bool = true; // Boolean type.
    let _an_emoji: char = '😻'; // Character type (Unicode).

    // Compound types group multiple values.
    // A tuple can hold values of different types.
    let my_tuple: (i32, f64, char) = (500, 6.4, 'a');
    let (val1, _val2, _val3) = my_tuple; // Destructuring the tuple.
    let first_element = my_tuple.0; // Accessing by index.
    println!("Destructured and indexed tuple values: {}, {}", val1, first_element);

    // An array must have elements of the same type and has a fixed length.
    let months = ["January", "February", "March"];
    let _numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Array with type and size.
    let same_values = [3; 5]; // Initializes an array with 5 elements, all set to 3.
    println!("First month: {}. A repeated value: {}", months[0], same_values[0]);

    // --- Functions, Statements, and Expressions ---
    println!("\n--- Functions and Expressions ---");

    // Calling a function. Function calls are expressions.
    another_function();
    print_labeled_measurement(5, 'h'); // Providing arguments.

    // The return value of a function can be used.
    let five = returns_five();
    let six = plus_one(five);
    println!("`plus_one(returns_five())` resulted in: {}", six);

    /*
       A block can be used as an expression.
       The last line `x + 1` does NOT have a semicolon, so its
       value is returned and bound to `y`.
    */
    let y = {
        let x = 3;
        x + 1 // No semicolon makes this an expression.
    };
    println!("The value of y from the expression block is: {y}");
}

/// This is an outer doc comment for the function below.
/// It takes no parameters and returns nothing.
fn another_function() {
    // This is a simple line comment.
    println!("`another_function` was called!");
}

/// Takes a value and a unit label and prints them.
/// Parameters require type annotations in Rust.
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

/// This function returns an i32. The return type is declared after `->`.
fn returns_five() -> i32 {
    5 // Implicitly returns 5 because it's the final expression.
}

/// This function takes an integer and returns that integer plus one.
fn plus_one(x: i32) -> i32 {
    x + 1 // An implicit return. Adding a semicolon here would cause an error.
}