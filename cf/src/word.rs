use std::io;

pub fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    

    // iterate over the characters of the string
    // count the number of lowercase and uppercase letters

    /*
    let char_vec: Vec<char> = input.chars().collect();
for c in char_vec.iter() {
    println!("{}", c);
}

    let mut chars = input.chars();
while let Some(c) = chars.next() {
    println!("{}", c);
}

    for c in input.split("") {
    if !c.is_empty() {
        println!("{}", c);
    }
}
     */
    input = input.trim().to_string(); // Trim the input to remove newline characters

    let mut lower = 0;
    let len = input.len();
    for c in input.chars() {
        if c.is_lowercase() {
            lower += 1;
        }
    }

    if (len - 1) / 2 < lower {
        // convert to lowercase
        input = input.to_lowercase();
    } else {
        // convert to uppercase
        input = input.to_uppercase();
    }

    println!("{}", input);
}