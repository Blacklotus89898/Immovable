use std::io;

pub fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut n = numbers[0];
    let mut k = numbers[1];

    while k > 0 {
        if n % 10 == 0 {
            n /= 10;
        } else {
            n -= 1;
        }
        k -= 1;
    }

    println!("{}", n);


}