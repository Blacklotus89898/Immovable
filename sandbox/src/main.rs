use std::io::{self, Write};

fn main() {
    // Formatted print

    // Basic formatting
    println!("{} days", 31);

    // Positional arguments
    println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Bob");
    
    // Named arguments
    println!("{name} is {age} years old", name = "Alice", age = 30);

    println!("Base 10 number: {}", 42);
    println!("Base 2 number: {:b}", 42);
    println!("Base 8 number: {:o}", 42);
    println!("Base 16 number (hexdecimal): {:x}", 42);

    println!("{number:>5}", number=1); // Right align
    println!("{number:<5}", number=1); // Left align
    println!("{number:^5}", number=1); // Center align
    println!("{number:0>5}", number=1); // Pad with zeros

    println!("{number:0>widht$}", number=1, widht=5); // Pad with custom character



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
        println!("Length: {}", input.len());

        passByRef(input);

        // passMut(input); // Uncommenting this will cause a compile-time error
        let mut input = String::from(input);
        passMut(&mut input); // Mutable reference to the input string
        passByValue(input.clone()); // Cloning to pass by value
        // passByValue(input); // Uncommenting this will cause a compile-time error


        // For debugging
        println!("data type:{input:?}");
    }
}


fn passByRef(input: &str) {
    println!("You passed by reference: {}", input);

}

fn passMut(input: &mut String) {
    input.push_str(" - modified");
    println!("You passed by mutable reference: {}", input);
}

fn passByValue(input: String) {
    println!("You passed by value: {}", input);
}
