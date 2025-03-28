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
    fn print<T: std::fmt::Debug>(x: T) {
        println!("{:?}", x);
    }

    print(5);
    print("Hello");

    // Ownership
    let s = String::from("Hello");
    let s1 = s;
    // println!("{}", s); // Error: value borrowed here after move
    // println!("{}", s1);
    println!("{}", s1.len());

    let s = String::from("Hello"); // need to redeclare s
    let s1 = s.clone(); // need to clone s
    println!("{}", s1);

    print_string(s); // s is moved and no longer accessible

    let n = 10;
    print_number(n); // n is copied because integers implement `Copy`


    // Ownership moved
    fn print_string(s: String) {
        println!("{}", s);
    }

    // Borrowing
    fn print_number(n: i32) {
        println!("{}", n);
    }

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

    // Option
    match divide2(10.0, 2.0) {
        Some(result) => println!("Result: {}", result),
        None => println!("Cannot divide by zero"),
    }

    // Option inside Result
    let result = option_inside_result(Some(10), Some(2));
    match result {
        Ok(Some(value)) => println!("Result: {}", value),
        Ok(None) => println!("Result is None"),
        Err(message) => println!("Error: {}", message),
    }

}

// Compiler is not one-pass, so function can be defined after main
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn divide2(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}
fn option_inside_result(a: Option<i32>, b: Option<i32>) -> Result<Option<i32>, String> {
    match (a, b) {
        (Some(a_val), Some(b_val)) => {
            if b_val == 0 {
                Err(String::from("Cannot divide by zero"))
            } else {
                Ok(Some(a_val / b_val))
            }
        }
        _ => Err(String::from("One or both options are None")),
    }
}

