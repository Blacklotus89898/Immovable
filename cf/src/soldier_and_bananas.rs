use std::io;

pub fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Parse space-separated input
    let values: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let k = values[0]; // First value
    let n = values[1]; // Second value
    let w = values[2]; // Third value

    let total_cost = k * w * (w + 1) / 2;
    let borrow = if total_cost > n { total_cost - n } else { 0 };

    println!("{}", borrow);
}