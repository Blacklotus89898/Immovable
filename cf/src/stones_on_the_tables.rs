use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Invalid input");
    
    input.clear(); // Clear the input buffer if you want to reuse it for another line
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s: String = input.trim().to_string();

    let mut result = 0i32;
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            result += 1;
        }
    }

    println!("{}", result);
}