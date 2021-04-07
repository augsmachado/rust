#![allow(unused)]

use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    /*  
    When we call to panic! causes the error message will contain two lines. The first line will show our panic message
    and the place in our source code where the panic occurred: src/main.rs: 2:5 indicates that it's the second line, fifth
    character of our src/main.rs file.
    The filename and line number reported by the error message will be someone else's code where the panic! macro is called,
    not the line of our code that eventually led to the panic! call. We can use the backtrace of the functions the panic! call
    came from to figure out the part of our code that is causing the problem.
    The next note line tells us that we can set the RUST_BACKTRACE environment variable to get a backtrace of exactly what
    happened to cause the error. A backtrace is a list of all the functions that have been called to get to this point.
    */
    
    // panic!("crash and burn");

    /* Most errors aren't serious enough to require the program to stop entirely. Sometimes, when a function fails, it's for a
    reason that you can easily interpret and respond to. 
    The T and E are generic type parameters.
    */
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let f = File::open("hello.txt");

    /*  
    We need to add to the code to take different actions depending on the value File::open returns.
    What we want to do instead is take different actions for different failure reasons.
     */
     let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    
    /* 
    If the Result value is the Ok variant, unwrap() will return the value inside the Ok. If the Result is the Err variant, unwrap will
    call the panic! macro for us.
    Another method, expect(), which is similar to unwrap(), lets us also choose the panic! error message. Using expect() instead
    of unwrap() and providing good error messages can convey your intent and make tracking down the source of panic easier.
     */
}

/* 
The ? placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result
values. There is a difference between what the match expression from does and what the ? operator does: error values that have the ?
operator called on them go through the from function, defined in the From trait the standard library, which is used to convert errors
from one type into another. The ? operator can be used in functions that have a return type of Result, because it is defined to work
in the same way as the match expression.
 */
 fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}