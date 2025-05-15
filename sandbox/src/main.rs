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
    let fl: f32 = 1.0f32; // Suffix annotation
    let fl: f64 = 1.0f64; // Suffix annotation
    let fl2: f32 = 1.0; // Float = 32 bits
    let myInt: i32 = 1; // Integer = 32 bits
    let myInt2: i64 = 1; // Integer = 64 bits
    let myInt3: i8 = 1; // Integer = 8 bits
    let myInt4: i16 = 1; // Integer = 16 bits
    let myInt5: u32 = 1; // Unsigned Integer = 32 bits
    let myInt6: u64 = 1; // Unsigned Integer = 64 bits
    let myInt7: u8 = 1; // Unsigned Integer = 8 bits

    // casting
    let myInt8: i32 = 1;
    let myInt9: i64 = myInt8 as i64; // Casting to 64 bits
    let myInt10: i32 = myInt9 as i32; // Casting to 32 bits

    type NanoSecond = u64; // Type alias
    let x: NanoSecond = 5; // Type alias

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


    // Structs -- custome data types
    #[derive(Debug)]
    struct Person {
        name: String,
        age: i32,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("Person: {} is {} years old", person.name, person.age);
    println!("Person: {:#?}", person); // Debug print
    println!("Person: {:?}", person); // Debug print

    impl Person {
        fn rename(&mut self, name: String) {
            self.name = name;
        }
        fn birthday(&mut self) {
            self.age += 1;
        }
    }

    let mut Alice = Person {
        name: String::from("Alice"),
        age: 30,
    };

    Alice.rename(String::from("Bob"));
    println!("Person: {} is {} years old", Alice.name, Alice.age);
    Alice.birthday();
    println!("Person: {} is {} years old", Alice.name, Alice.age);

    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        // Method to create a new Point
        fn new(x: f64, y: f64) -> Point {
            Point { x, y }
        }

        // Method for length
        fn distance(&self, other: &Point) -> f64 {
            ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
        }
    }

    let point = Point { x: 1.0, y: 2.0 };
    let another_point = Point::new(23.0, 234.0);
    let distance = point.distance(&another_point);
    println!("Distance: {}", distance);

    let point = Point { x: 1.0, ..point }; // Struct update syntax, by copying the value of x from point

    // Enums
    #[derive(Debug)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }


    // Vectors and Slices
    let xs = vec![1, 2, 3, 4, 5]; //vectir
    let ys = [6, 7, 8, 9, 10]; //array
    let zs = &xs[1..3]; //slice

    println!("xs: {:?}", xs);
    println!("ys: {:?}", ys);
    println!("zs: {:?}", zs);
    println!("xs[0]: {}", xs[0]);

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(&x) => println!("xs[{}]: {}", i, x),
            None => println!("xs[{}]: None", i),
        }
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
