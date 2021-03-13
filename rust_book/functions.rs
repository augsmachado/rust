/*
    fn keyword allows you declare new functions.
    Rust code uses snake case as conventional style for function and variable names. In snake case,
        all letters are lowercase and underscores separate words.
*/
fn another_function() {
    println!("Another function!");
}

/*
    In those languages, you can write x = y = 6 and have both x and y have the value 6; that is not the
        case in Rust.
    Expressions evaluate to something and make up most of the rest of code thar you'll write in Rust. Calling
        a function is an expression. Calling a macro is an expression. The block that we use to create new scopes {},
        is an expression.
    Note he x + 1 line without a semicolon t the end, which is unlike most of the lines you've seen so far. Expressions
        not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement,
        which will then not return a value. Keep this in mind as you explore function return values and expressions next.
*/
fn expression() {
    let _x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

/*
    Functions can return values to the code that calls them. We don’t name return values, but we do declare their type after
        an arrow (->). In Rust, the return value of the function is synonymous with the value of the final expression in the
        block of the body of a function. You can return early from a function by using the return keyword and specifying a value,
        but most functions return the last expression implicitly

*/
fn five() -> i32 {
    5
}



fn main() {
    println!("Hello World!");

    another_function();

    expression();

    let x = five();
    println!("The value of x is: {}", x);
}