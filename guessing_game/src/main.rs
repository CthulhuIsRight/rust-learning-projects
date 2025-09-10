use std::cmp::Ordering;
use std::io;
use rand::random_range;

fn main() {
    println!("Guess the number!");
    let secret_number = random_range(1..=100);
    // println!("The secret number is: {secret_number}"); // Uncomment for debugging only
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess_trimmed = guess.trim();
        if guess_trimmed.eq_ignore_ascii_case("quit") {
            println!("You have chosen to quit. Farewell!");
            break;
        }
        let guess: u32 = match guess_trimmed.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number or 'quit' to exit!");
                continue;
            }
        };
        println!("You guessed {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}