use std::io;

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let n = input.trim().parse::<i32>().unwrap();
    
    input.clear();
    let mut numbers = Vec::new();

    // Collect exactly n * 3 numbers
    let mut total_numbers = 0;
    while total_numbers < n * 3 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let row_numbers: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        total_numbers += row_numbers.len() as i32;
        numbers.extend(row_numbers);
    }

    // Process the numbers row by row
    for j in 0..3 {
        let mut sum = 0;
        for i in 0..n {
            sum += numbers[(i * 3 + j) as usize];
        }
        if sum != 0 {
            println!("NO");
            return;
        }
    }

    println!("YES");
}
