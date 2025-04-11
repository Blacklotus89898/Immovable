use rand::Rng;
use std::io;

fn main() {
    println!("Hello, world!");
    println!("Hey what is your name?");
    // Declare varable to store name
    let mut name = String::new();

    // Read name from user into the variable
    io::stdin().read_line(&mut name).unwrap();

    // Trim the newline character from the input
    let name = name.trim();

    // Print a welcome message using the name
    println!("Welcome to the guessing game, {name}!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess a number");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
        // Convert the guess to an integer
    let mut guess: i32 = guess.trim().parse().unwrap();

    while guess != secret_number {
        println!("Try again");
        let mut guess_input = String::new();
        io::stdin().read_line(&mut guess_input).unwrap();
        // Convert the guess to an integer
        guess = guess_input.trim().parse().unwrap();

        if (guess < secret_number) {
            println!("Too low!");
        } else {
            println!("Too high!");
        }
    }
    println!("You guessed it!");
}
