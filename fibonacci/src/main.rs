use std::io;
use std::f64;

fn main() {
    
    println!("nth Fibonacci number using Binet's Simplified Formula");

    let mut nth_number = String::new();

    println!("Please input a nth Fibonacci number");  // must be a floating point number

    io::stdin().read_line(&mut nth_number).expect("Failed to read nth Fibonacci number");

    let mut nth_num : f64 = nth_number.trim().parse().expect("Failed to parse nth Fibonacci number");

    if nth_num < 0.0 { nth_num = nth_num * (-1.0)};

    // Binet's Simplified Formula
    let five :f64 = 5.0;
    let mut result: f64 = 1.0 + five.sqrt();
    result = result / 2.0;
    result = result.powf(nth_num);
    result = result/five.sqrt();


    println!(
        "Warning! The symbol [] means `round to the nearest integer`.\n
        nth Fibonacci number: [{}]", result
    );
}
