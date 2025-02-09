// Define a struct named Book with the following fields:
    // title of type String
    // author of type String
    // pages of type u32
// In your main function, create an instance of Book and print its details using the println! macro.

// #[derive(Debug)]
// struct Book {
//     title: String,
//     author: String,
//     pages: u32
// }

// fn main() {
//     let book1: Book = Book {
//         title: String::from("Rust Book"),
//         author: String::from("Rust Devs"),
//         pages: 20
//     };

//     let book2: Book = Book {
//         title: String::from("Rust Book 2"),
//         author: String::from("Rust Devs"),
//         ..book1
//     };
//     println!("Book 1: {:?}\nBook 2: {:?}", book1, book2);
// }

// Define a struct named Rectangle with two fields: width and height, both of type u32.
// In main, create an instance of Rectangle.
// Then, create a new Rectangle instance that reuses the width from the first instance but provides a different height using the struct update syntax.
// Print both rectangles.

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32
// }
// fn main() {
//     let rect1: Rectangle = Rectangle {
//         width: 20,
//         height: 25
//     };

//     let rect2: Rectangle = Rectangle {
//         height: 30,
//         ..rect1
//     };

//     println!("First rect: {:?}\nSecond rect: {:?}", rect1, rect2);
// }

// Define a tuple struct named Color that holds three u8 values representing the red, green, and blue components.
// In main, create an instance of Color.
// Print each color component individually.

// #[derive(Debug)]
// struct Color(u8, u8, u8);

// fn main() {
//     let first_color: Color = Color(220, 250, 180);
//     let second_color: Color = Color(150, 180, 220);

//     let first_fields: [u8; 3] = [first_color.0, first_color.1, first_color.2];
//     let second_fields: [u8; 3] = [second_color.0, second_color.1, second_color.2];

//     let mut counter: usize = 0;
//     for _ in first_fields {
//         if counter < first_fields.len() {
//             println!("The field for the first color at index {} is: {}", counter, first_fields[counter]);
//             counter += 1
//         }
//     }
//     counter = 0;

//     for _ in second_fields {
//         if counter < second_fields.len() {
//             println!("The field for the second color at index {} is: {}", counter, second_fields[counter]);
//             counter += 1
//         }
//     }
// }

//
