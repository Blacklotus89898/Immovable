use std::io::{self, Write};

fn main() {
    println!("Hello, world!");
    println!("Welcome to the Rust Terminal App!");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        println!("You entered: {}", input);
    }
}

// List current directory`

