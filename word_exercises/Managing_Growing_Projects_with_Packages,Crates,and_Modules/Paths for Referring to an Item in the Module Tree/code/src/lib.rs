mod front_of_house { // By default, modules are private upon declaration. This means that only other modules within the same hierarchial position within this file can use it. No one is allowed to change the code / values within private modules.
    pub mod hosting { // you have to add pub to the mod and the function within for the code to be completely readbale / usable by other files.
        // Using just pub on mod makes the module readable to its ancestors only, by making the function pub as well, you make the code within the mod readable by everyone
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist(); // won't compile as hosting is a private module, a private modules code can only be accessed via it's children
    // And yes, this means parent modules of private children can't access their childrens code.
    // To change this, we can use the 'pub' keyword infront of a modules declaration to make it 'public', then doing the same for the function within that module.
    // A public module allows it's code to be used by others.
    
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

mod back_of_house { // By default, modules are private upon declaration. This means that only other modules within the same hierarchial position within this file can use it. No one is allowed to change the code / values within private modules.
    pub struct Breakfast { // pub needed here for the properties within Breakfast to be accessible to everyone
        pub toast: String, // makes the toast field public, thus allows the function below (summer) to change it's value based on what's given to the functions param
        seasonal_fruit: String, // in structs, you need to put pub before every field that you want public. Notice here how there isn't a pub keyword, thus a private field
    }

    impl Breakfast { // you need an impl block because when a struct has a pprivate field, you need to create a function which acts as a constructor which tells the compiler what the value for that private field is
        // the rest of the public fields should have a parameter that represents the value that should be inserted into that field
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() { // this allows the function to be used in different files
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

mod back_of_house {
    pub enum Appetizer { // in contrast with structs, if an enum as pub before it's declaration (like it does here) then all fields are public within it
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}