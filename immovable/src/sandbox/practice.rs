mod mod1;

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

}

// Compiler is not one-pass, so function can be defined after main
fn add(a: i32, b: i32) -> i32 {
    a + b
}