mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist(); // won't compile as hosting is a private module, a private modules code can only be accessed via it's children
    // And yes, this means parent modules of private children can't access their childrens code.
    // To change this, we can use the 'pub' keyword infront of a modules declaration to make it 'public'.
    // A public module allows it's code to be used by others.
    
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}