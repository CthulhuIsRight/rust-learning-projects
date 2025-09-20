// The #[derive(Debug)] attribute automatically implements the Debug trait
// for the Rectangle struct, which allows us to print it for debugging.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// This function calculates the area of a rectangle.
// It takes an immutable reference (&) to a Rectangle instance,
// so it borrows the struct instead of taking ownership.
fn area(rectangle: &Rectangle) -> u32 {
    // Access the fields of the struct using dot notation
    // and return the calculated area.
    rectangle.width * rectangle.height
}

fn main() {
    // Create an instance of the Rectangle struct.
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Print the contents of the struct instance for debugging.
    // The {:#?} specifier formats it to be easy to read.
    println!("rect1 is {:#?}", rect1);

    // Call the area function with a reference to our instance
    // and print the result.
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}