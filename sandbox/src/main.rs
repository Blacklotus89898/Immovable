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

        passByRef(input);

        // passMut(input); // Uncommenting this will cause a compile-time error
        let mut input = String::from(input);
        passMut(&mut input);
        // passByValue(input); // Uncommenting this will cause a compile-time error
        passByValue(input.clone()); // Cloning to pass by value
        

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
