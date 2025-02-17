mod front_of_house {
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