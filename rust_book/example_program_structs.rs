// To understand when we might want to use structs, let's write a program that calculates the area of
// rectangle. We'll start with single variables, and then refactor the program until we're using structs instead.

// The Debug trait enables us to print our struct in a way that is useful for developers so we can see its values
// while we're debugging our code.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(" The area of rectangle is: {} square pixels.", area(width1, height1));

    // Refactoring with tuples
    let rect1 = (30, 50);
    println!(" The area of rectangle using tuple is: {} square pixels.", area_with_tuples(rect1));

    // Refactoring with structs: adding more meaning
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(" The area of rectangle using structs: {} square pixels.", area_with_structs(&rect2));

    // It'd be nice to able to print an instance of Rectangle while we're debugging our program and see the values for all its fields.
    println!("rect2 is {:?}", rect2);
}

fn area(width:u32, height:u32) -> u32 {
    width * height
}

// Tuples don't name their elements, so our calculation has become more confusing because we have to index into
// the parts of the tuple.
fn area_with_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Our area function is now defined with one parameter, which we've named rectangle, whose type is an immutable borrow
// of a struct Rectangle instance. We want to borrow the struct rather than take ownership of it. This way, main retains
// its ownership and can continue using rect2, which is the reason we use the & in the function signature and where call
// the function.
fn area_with_structs(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}