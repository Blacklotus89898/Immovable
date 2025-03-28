mod mod1;
mod person;

fn main() {
    // Printing
    println!("Hello, world!");
    
    // Variables and Mutability
    let x = 5;
    let mut y = 10;
    println!("x: {}, y: {}", x, y);
    y = 15;
    println!("x: {}, y: {}", x, y);

    // Primitive Data Types
    let a: i32 = 5;
    let b: f64 = 5.0;
    let c: bool = true;
    let d: char = 'a';
    let e: [i32; 5] = [1, 2, 3, 4, 5]; // Array
    let f: (i32, f64, bool) = (5, 5.0, true); // Tuple
    let g: [i32; 5] = [0; 5]; // Array with default value
    println!("a: {}, b: {}, c: {}, d: {}, e: {:?}, f: {:?}, g: {:?}", a, b, c, d, e, f, g);

    // Other Data Types
    let h: &i32 = &a; // Reference
    let i: *const i32 = &a; // Raw Pointer
    let j: Box<i32> = Box::new(a); // Box Pointer
    println!("h: {}, i: {:p}, j: {}", h, i, j);
    
    let k: Option<i32> = Some(a); // Option
    let l: Result<i32, i32> = Ok(a); // Result
    println!("k: {:?}, l: {:?}", k, l);

    let mut dict: std::collections::HashMap<&str, i32> = std::collections::HashMap::new(); // HashMap
    dict.insert("key1", 1);
    dict.insert("key2", 2);
    dict.insert("key3", 3);
    let list = vec![1, 2, 3, 4, 5]; // Vector
    println!("dict: {:?}, list: {:?}", dict, list);

    // Control Flow
    if a == 5 {
        println!("a is 5");
    } else if a == 6 {
        println!("a is 6");
    } else {
        println!("a is not 5");
    }

    // Match
    let result = match a {
        5 => "a is 5",
        6 => "a is 6",
        _ => "a is not 5",
    };
    println!("{}", result);

    // Loop
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            break;
        }
    }

    // While
    while count < 10 {
        count += 1;
    }

    // For from 0 to 5
    for i in 0..5 {
        println!("{}", i);
    }

    // Functions
    let sum = add(5, 5);
    println!("Sum: {}", sum);

    // Modules
    mod1::say_hello();

    // Structs
    let p = person::Person {
        name: String::from("Alice"),
        age: 25,
    };
    println!("{} is {} years old", p.name, p.age);

    // Enum
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    fn move_player(dir: Direction) {
        match dir {
            Direction::Up => println!("Moving Up"),
            Direction::Down => println!("Moving Down"),
            Direction::Left => println!("Moving Left"),
            Direction::Right => println!("Moving Right"),
        }
    }

    move_player(Direction::Up);
    move_player(Direction::Down);

    // Traits -- similar to interfaces in other languages
    trait Greet {
        fn greet(&self);
    }

    impl Greet for person::Person {
        fn greet(&self) {
            println!("Hello, my name is {}", self.name);
        }
    }

    p.greet();

    // Generics
    fn print<T>(x: T) {
        println!("{:?}", x);
    }

    print(5);
    print("Hello");

    // Lifetimes
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let s1 = String::from("abcd");
    let s2 = String::from("xyz");
    let result = longest(s1.as_str(), s2.as_str());
    println!("The longest string is {}", result);

    // Error Handling
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            return Err(String::from("Cannot divide by zero"));
        }
        Ok(a / b)
    }

    // Similar to try-catch
    let result = divide(10, 2);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(message) => println!("Error: {}", message),
    }

}

// Compiler is not one-pass, so function can be defined after main
fn add(a: i32, b: i32) -> i32 {
    a + b
}