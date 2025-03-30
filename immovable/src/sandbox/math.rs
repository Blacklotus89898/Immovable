// Function templates

// pub fn functionName(a: i32, b: i32) -> i32 {
//     a + b
// }

/// Fibonaccci function
pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

/// Factorial function
pub fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

/// Function to check if a number is prime
pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// Function to calculate the greatest common divisor (GCD) of two numbers
pub fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

/// Function to calculate the least common multiple (LCM) of two numbers
pub fn lcm(a: u32, b: u32) -> u32 {
    (a * b) / gcd(a, b)
}

/// Function to calculate the power of a number
pub fn power(mut base: u32, exponent: u32) -> u32 {
    if exponent == 0 {
        return 1;
    } else {
        for _ in 1..exponent {
            base *= base;
        }
        return base;
    }
}
