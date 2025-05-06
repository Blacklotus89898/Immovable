use std::io;



pub fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let mut hello: Vec<char> = "hello".chars().collect();

    for c in input.chars() {
        if c == hello[0] {
            hello.remove(0);
        }
        if hello.is_empty() {
            println!("YES");
            return;
        }
    }
    
    println!("NO");
    
}