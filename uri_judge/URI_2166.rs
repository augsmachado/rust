// The method of periodic continued fractions is one of the many ways to calculate the square root of a natural number.
// This method uses as denominator a repetition for fractions. This repetition can be done by a fixed number of times.
// For example, by repeating 2 times the continued fraction to calculate the square root of 2
// Your task is to calculate the approximate value of square root of 2 given the number N of repetitions.

//use std::io;

fn main() {
    println!("The number of repetitions must be between zero and 100:");

    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read number");

    let n: f64 = number.trim().parse::<f64>().unwrap();

    if n < 0.0 {
        println!("Square root is negative. Dont is possible to calculate a value in real numbers");
    } if n == 0.0 {
        println!("Square root is: {}", (n + 1.0));
    } if n > 0.0 {
        let mut index = 1.0;
        let mut aux = 2.0;

        while index < n {
            let root = 2.0 + (1.0/ aux);
            aux = root;

            index += 1.0;
        }

        println!("Square root of 2 is: {}", (1.0 +(1.0/aux)));
    }
}