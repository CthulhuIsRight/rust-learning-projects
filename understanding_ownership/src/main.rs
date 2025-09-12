//! # Rust Ownership Concepts Demonstration
//!
//! This program demonstrates the core ownership rules from the provided notes,
//! including scope, the move semantics of heap-allocated data like `String`,
//! and the copy semantics of stack-allocated data.

// The main function where we will demonstrate most concepts.
fn main() {
    println!("## Part 1: Scope ##");
    // A scope is the range within which a variable is valid.
    {
        // s is not valid here, it's not yet declared.
        let s = String::from("hello"); // s is valid from this point forward.
        println!("Inside the inner scope, s = '{}'", s);
    } // This scope is now over, and s is no longer valid.
      // println!("{}", s); // <-- This would cause a compile error.

    println!("\n-------------------------------------------------\n");

    println!("## Part 2: The `Move` Semantic ##");
    // For heap-allocated data like `String`, assignment transfers ownership.
    // This is known as a "move".
    let s1 = String::from("hello"); // s1 comes into scope.
    let s2 = s1; // s1's value is moved into s2.
                 // s1 is no longer considered valid to prevent double-free errors.

    println!("s2 holds the value: '{}'. s1 is no longer valid.", s2);
    // println!("Trying to use s1 fails: {}", s1); // <-- This would cause a "borrow of moved value" error.

    println!("\n-------------------------------------------------\n");

    println!("## Part 3: The `Clone` Trait for Deep Copies ##");
    // If we want a deep copy of heap data, we must explicitly call `clone`.
    let s3 = String::from("world");
    let s4 = s3.clone(); // `clone` creates a full copy of the heap data.

    println!("s3 = '{}', s4 = '{}'. Both are valid.", s3, s4);

    println!("\n-------------------------------------------------\n");

    println!("## Part 4: Stack-Only Data and the `Copy` Trait ##");
    // Types with a known size at compile time are stored entirely on the stack.
    // They implement the `Copy` trait, so they are copied, not moved.
    let x = 5;
    let y = x; // `x` is copied into `y`. No move occurs.

    println!("x = {}, y = {}. Both are valid because integers implement the `Copy` trait.", x, y);

    // Tuples also implement `Copy` if all their elements implement `Copy`.
    let my_tuple = (10i32, true);
    let another_tuple = my_tuple;
    println!("The first element of `another_tuple` is {}", another_tuple.0);

    println!("\n-------------------------------------------------\n");

    println!("## Part 5: Ownership and Functions ##");
    // Passing a variable to a function will move or copy, just as assignment does.
    let s_func = String::from("take me");
    takes_ownership(s_func); // `s_func`'s value moves into the function and is no longer valid here.
                             // println!("{}", s_func); // <-- This would error!

    let x_func = 10;
    makes_copy(x_func); // `x_func` is copied; it's still valid to use it here.
    println!("We can still use x_func after `makes_copy`: {}", x_func);

    println!("\n-------------------------------------------------\n");

    println!("## Part 6: Return Values and Scope ##");
    // Returning values can also transfer ownership.
    let returned_s1 = gives_ownership(); // `gives_ownership` moves its return value into `returned_s1`.
    let returned_s2 = String::from("give and take");
    let returned_s3 = takes_and_gives_back(returned_s2); // `returned_s2` is moved into the function, which moves its return value into `returned_s3`.

    println!("Ownership was transferred: s1='{}', s3='{}'", returned_s1, returned_s3);

    println!("\n-------------------------------------------------\n");
    
    println!("## Part 7: Tedious Ownership Transfer ##");
    // It can be annoying to have to return ownership just to use a value again.
    let string_to_measure = String::from("how long am I?");
    let (measured_string, length) = calculate_length(string_to_measure);
    println!("The length of '{}' is {}. Ownership was returned.", measured_string, length);

} // All variables go out of scope here and `drop` is called on those that still own their data.

// --- Helper Functions ---

/// This function takes ownership of a String because `String` does not implement `Copy`.
fn takes_ownership(some_string: String) {
    println!("Inside `takes_ownership`: {}", some_string);
} // `some_string` goes out of scope, `drop` is called, and the memory is freed.

/// This function makes a copy of an integer because `i32` implements `Copy`.
fn makes_copy(some_integer: i32) {
    println!("Inside `makes_copy`: {}", some_integer);
} // `some_integer` goes out of scope. Nothing special happens.

/// This function moves its return value to the function that calls it.
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // `some_string` is returned, and ownership is moved.
}

/// This function takes a String and returns it, moving ownership back out.
fn takes_and_gives_back(a_string: String) -> String {
    a_string // `a_string` is returned, moving ownership.
}

/// A function that demonstrates returning ownership alongside another value in a tuple.
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}