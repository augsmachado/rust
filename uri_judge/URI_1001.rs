// Read 2 numbers and sum them
use std::io;

fn sum_numbers(num_a: &f64, num_b: &f64) {
    println!("X = {}", (num_a + num_b));
}

fn main() {
    println!("Input two numbers to sum!");
    
    println!("First number:");
    let mut a = String::new();
    println!("Seconde number:");
    let mut b = String::new();

    io::stdin().read_line(&mut a).expect("Failed to read line");
    io::stdin().read_line(&mut b).expect("Failed to read line");

    let num_a: f64 = a.trim().parse::<f64>().unwrap();
    let num_b: f64 = b.trim().parse::<f64>().unwrap();

    sum_numbers(&num_a, &num_b);
}