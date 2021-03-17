// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
// Here’s a small programming problem: write a function that takes a string and returns the first word it finds in
// that string. If the function doesn’t find a space in the string, the whole string must be one word, so the
// entire string should be returned.

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let _word = first_word(&my_string[..]);
    println!("The value of the first_word is: {}", _word);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let _word = first_word(&my_string_literal[..]);
    println!("The value of the my_string_literal is: {}", _word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word(my_string_literal);
    println!("The value of the first_word my_string_literal is: {}", _word);
}
