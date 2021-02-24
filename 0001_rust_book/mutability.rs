/*
    Variables are immutable only by default.
    You can make them mutable by adding mut in front of the variable name. In
        addition to allowing this value to change, mut conveys intent to future
        readers of the code by indicating that other parts of the code will be
        changing this variable's value.
*/

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
