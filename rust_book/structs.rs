// A struct, or structure, is a custom data type that lets you name and package together multiple related values that
// make up a meaningful group.
// Like tuples, the pieces of a struct can be different types. Unlike with tuples, you’ll name each piece of data so
// it’s clear what the values mean. As a result of these names, structs are more flexible than tuples: you don’t have
// to rely on the order of the data to specify or access the values of an instance.


// A struct’s name should describe the significance of the pieces of data being grouped together.
// Then, inside curly brackets, we define the names and types of the pieces of data, which we call fields.
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}



fn main() {

    // The struct definition is like a general template for the type, and instances fill in that template with particular
    // data to create values of the type.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field.
    user1.email = String::from("anotheremail@example.com");

    let _user2 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    // Creating instances from other instances with struct update syntax
    // The syntax .. specifies that the remaining fields not explicitly set should the same value as the fields in the given instance.
    let _user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    // Tuple structs have the added meaning the struct name provides but don't have names associated with their fields; rather, they
    // just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple be
    // a different type from other tuples, and naming each field as in a regular struct would be verbose or redundant.
    // Otherwise, tuple struct instances behave like tuples: you can destructure them into their individual pieces, you can use a .
    // followed by the index to access an individual value, and so on.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // You can also define structs that don't have any fields! These are called unit-like structs because they behave similarly to (),
    // the unit type. Unit-like structs can be useful in situations in which you need to implement a trait on some type but don't have
    // any data that want to store in the type itself.
    // In the first definition of User, we used the owned String type rather than the &str string slive type. This is deliberate choice
    // because we want instances of this struct to own all of its data and for that data to be valid for as long as the entire structs
    // is valid.
    // It's possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes. Lifetimes
    // ensure that the data referenced by a struct is valid for as long as the struct is.
}

// Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
// We can use the field init shorthand syntax to rewrite build_user so that it behaves exactly the same but doesn’t have
// the repetition of email and username
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
