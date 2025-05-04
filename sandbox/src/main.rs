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

    // Debugging


    // Primitives
    let logical: bool = true;

    let fl: f64 = 1.0; // Float = 64 bits
    let fl2: f32 = 1.0; // Float = 32 bits
    let myInt: i32 = 1; // Integer = 32 bits
    let myInt2: i64 = 1; // Integer = 64 bits
    let myInt3: i8 = 1; // Integer = 8 bits
    let myInt4: i16 = 1; // Integer = 16 bits
    let myInt5: u32 = 1; // Unsigned Integer = 32 bits
    let myInt6: u64 = 1; // Unsigned Integer = 64 bits
    let myInt7: u8 = 1; // Unsigned Integer = 8 bits

    let c: char = 'a'; // Character
    let i = 5i32; // Integer, suffix annotation
    let mut myInfer = "sfdgs";

    let myArray: [i32; 5] = [1, 2, 3, 4, 5]; // Array
    let mySlice: &[i32] = &myArray[1..3]; // Slice

    let myTuple: (i32, f64, char) = (1, 2.0, 'a'); // Tuple
    let myTuple2 = (1, 2.0, 'a'); // Tuple with type inference

    let (a, b, c) = myTuple; // Destructuring tuple
    println!("a: {}, b: {}, c: {}", a, b, c);
    println!("myArray: {:?}", myArray); // Debug print

    let a = myTuple.0;

    println!("a: {}", a); // Accessing tuple element

    // Loops 
    let mut counter: i32 = 0;
    let loop_val = loop {
        println!("loopVal: {}", counter);
        counter += 1;
        if counter== 5 {
            break counter;
        }
    };

    println!("loopVal: {}", loop_val); // Accessing loop value

    let mut counter: i32 = 0;
    while counter < 4 {
        println!("counter: {}", counter);
        counter += 1;
    }

    for number in 1..5 {
        println!("number: {}", number);
    }
    for number in (1..5).rev() {
        println!("number: {}", number);
    }

    let mut myVec: Vec<i32> = vec![1, 2, 3, 4, 5]; // Vector

    for number in myVec.iter() {
        println!("number: {}", number);
    }

    


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
