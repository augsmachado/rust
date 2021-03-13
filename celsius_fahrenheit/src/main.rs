use std::io;

fn main() {
    println!("Temperature converter");


    let mut choice = String::new();

    println!("Please input your choice:\n1- Celsius to Fahrenheit\n2- Fahrenheit to Celsius");

    io::stdin().read_line(&mut choice).expect("Failed to read input");

    // Convert String to i32
    let t_choice: i32 = choice.trim().parse().expect("Failed to parse input");


    println!("Please input the temperature:");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .unwrap();
        
    let converter: f32 = temperature.trim().parse().unwrap();

    // Convert based on choice
    if t_choice == 1 {
        println!("Celsius: {}\nFahrenheit: {}", temperature,(converter * 1.8) + 32.0);
    } else if t_choice == 2 {
        println!("Fahrenheit: {}\n Celsius: {}", temperature,(converter - 32.0) / 1.8);
    }
}
