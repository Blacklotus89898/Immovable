use rand::Rng;
use std::io;

fn main() {
    println!("Hello, world!");
    println!("Hey, what is your name?");
    
    // Declare variable to store name
    let mut name = String::new();

    // Read name from user into the variable
    io::stdin().read_line(&mut name).expect("Failed to read input");
    
    // Trim the newline character from the input
    let name = name.trim();

    // Print a welcome message using the name
    println!("Welcome to the guessing game, {name}!");

    // Generate a random secret number
    let secret_number = rand::rng().random_range(1..=100);

    println!("Guess a number between 1 and 100");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");

        // Convert the guess to an integer with error handling
        match guess.trim().parse::<i32>() {
            Ok(num) => {
                match num.cmp(&secret_number) {
                    std::cmp::Ordering::Less => println!("Too low!"),
                    std::cmp::Ordering::Greater => println!("Too high!"),
                    std::cmp::Ordering::Equal => {
                        println!("You guessed it! The secret number was {secret_number}.");
                        break;
                    }
                }

            }
            Err(_) => {
                println!("Please enter a valid number.");
            }
        }
    }
}
