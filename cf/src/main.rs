
pub mod elephant;
// use std::io::{self, Read};

fn main() {
    // // Read all input at once
    // let mut input = String::new();
    // io::stdin().read_to_string(&mut input).unwrap();

    // // Parse input into variables
    // let mut iter = input.split_whitespace();
    // let n: i32 = iter.next().unwrap().parse().unwrap();
    // let a: i32 = iter.next().unwrap().parse().unwrap();
    // let b: i32 = iter.next().unwrap().parse().unwrap();

    // println!("n: {}, a: {}, b: {}", n, a, b);

    elephant::main();
    
}

