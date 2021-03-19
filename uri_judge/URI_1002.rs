// Circunference area

use std::io;
use std::f64::consts::PI;

fn circunference(radius: &String) {
    let mut radius: f64 = radius.trim().parse::<f64>().unwrap();
    radius = radius.powf(2.0);

    println!("Circunference area: {}", radius * PI)
}

fn main() {
    println!("Input the radius of the circle: ");

    let mut radius = String::new();
    io::stdin().read_line(&mut radius).expect("Failed to read line");

    circunference(&radius);
}