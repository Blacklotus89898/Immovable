use std::io;

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    // Parse input
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut a = numbers[0];
    let mut b = numbers[1];

    let mut counter = 0;
    while a <= b {
        a *= 3;
        b *= 2;
        counter += 1;
    }

    
    println!("{}", counter);
}

