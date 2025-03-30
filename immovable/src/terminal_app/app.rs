use std::io;

pub fn run_app() {
    println!("Welcome to My Terminal App!");

    loop {
        // Prompt for user input
        println!("Enter a command (type 'exit' to quit):");

        // Read user input from the terminal
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Trim input to remove newline characters
        let input = input.trim();

        // Check user command
        match input {
            "hello" => println!("Hello there! How can I assist you?"),
            "exit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Unknown command: {}", input),
        }
    }
}