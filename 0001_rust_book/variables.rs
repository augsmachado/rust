/*
    This code will fail because in Rust variables are immutable
    When a variable is immutable, once a value is bound to a name, yu can't
        change that value
*/

fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
