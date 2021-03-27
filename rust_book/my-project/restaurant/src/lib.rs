#[cfg(test)]
mod tests {
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house {
    // The pub keyword on a module only lets code in its ancestor modules refer to it.
    // The add_to_waitlist function is private. The privacy rules apply to structs, enums, functions,
    // and methods as well as modules.
    // Let’s also make the add_to_waitlist function public by adding the pub keyword before its definition.
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// We used super so we’ll have fewer places to update code in the future if this code gets moved to a different module.
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    // We can also use pub to designate structs and enums as public, but there are a few extra details. If we use pub
    // before a struct definition, we make the struct public, but the struct’s fields will still be private. We can make
    // each field public or not on a case-by-case basis.
    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String,
    }

    // In contrast, if we make an enum public, all of its variants are then public. We only need the pub before the enum keyword
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// Adding use and a path in a scope is similar to creating a symbolic link in the filesystem. By adding
// use crate::front_of_house::hosting in the crate root, hosting is now a valid name in that scope, just as though the
// hosting module had been defined in the crate root. Paths brought into scope with use also check privacy, like any other paths.
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    // Our preference is to specify absolute paths because it’s more likely to move
    // code definitions and item calls independently of each other.
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn serve_order() {}

