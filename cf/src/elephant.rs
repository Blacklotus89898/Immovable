use std::io;



pub fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    

    let x :i32 = input.trim().parse().unwrap();
    let res:i32 = (x + 4) / 5;
    
    println!("{}", res);
}