#[allow(dead_code)]

// Define an Enum called Direction with four variants: North, East, South, and West.

// #[derive(Debug)]
// enum Direction {
//     North,
//     East,
//     South,
//     West
// }

// fn main() {
//     let north: Direction = Direction::North;
//     let east: Direction = Direction::East;
//     let south: Direction = Direction::South;
//     let west: Direction = Direction::West;

//     println!("North: {:?}, East: {:?}, South: {:?}, West: {:?}", north, east, south, west)
// }

// Define an enum called IpAddr with two variants:
// V4 that stores four u8 values (representing an IPv4 address)
// V6 that stores a String (representing an IPv6 address)
// In your main function, create one instance of each variant and print them.

// #[derive(Debug)]
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String)
// }

// fn main() {
//     let v4_instance: IpAddr = IpAddr::V4(4, 2, 3, 1);
//     let v6_instance: IpAddr = IpAddr::V6(String::from("Ip Address"));
//     println!("V4 Instance: {:?}\nV6 Instance: {:?}", v4_instance, v6_instance)
// }

// Define an enum called Coin with variants: Penny, Nickel, Dime, and Quarter.
// Write a function named value_in_cents that accepts a Coin and returns its value in cents as a u32 using a match expression.
// In your main function, create an instance of each coin variant and print its value in cents.

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter
// }

// fn main() {
//     fn value_in_cents(coin: Coin) -> u32 {
//         match coin {
//             Coin::Dime => 10,
//             Coin::Nickel => 5,
//             Coin::Penny => 1,
//             Coin::Quarter  => 25,
//         }
//     }

//     let penny: Coin = Coin::Penny;
//     let nickel: Coin = Coin::Nickel;
//     let dime: Coin = Coin::Dime;
//     let quarter: Coin = Coin::Quarter;

//     println!("Penny: {}\nNickel: {}\nDime: {}\nQuarter: {}", value_in_cents(penny), value_in_cents(nickel), value_in_cents(dime), value_in_cents(quarter))
// } // above prints each coins value using the value+in_cents function. Since these values are u32's (because that's what the function returns) I don't need debug trait to print them.

// Define an enum called Message with the following variants:
// Quit
// Move { x: i32, y: i32 } (an anonymous struct-like variant)
// Write(String)
// ChangeColor(i32, i32, i32) (a tuple-like variant)
// Implement an impl block for Message that includes a method call(&self) which prints a different message depending on the variant.
// In main, create one instance of each variant and call the call method on them.

// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move(i32, i32),
//     Write(String),
//     ChangeColor(i32, i32, i32)
// }

// fn main() {
//     impl  Message {
//         fn call(&self) {
//             match self {
//                 Message::Quit => println!("Quit Message recieved"),
//                 Message::ChangeColor(r, g , b ) => println!("Color value recieved"),
//                 Message::Write(s) => println!("Write recieved"),
//                 Message::Move(x,y ) => println!("Move value recieved")
//             }
//         }
//     }

//     let quit_message = Message::Quit.call();
//     let change_color_message = Message::ChangeColor(120, 150, 180).call();
//     let write_message = Message::Write(String::from("Message")).call();
//     let move_message = Message::Move(20, 40).call();

//     // Note: Since we declare the var's above with the method call immediately, 
//     // the method call gets activated upon declaration, thus all of these values getting printed to the console automatically
// }

// Write a function called double_or_zero that accepts an Option<i32>.
// If the input is Some(value), return the value multiplied by 2.
// If the input is None, return 0.
// In your main function, test the function with Some(10) and None, printing the results.

// fn main() {
//     fn double_or_zero(number: Option<i32>) -> i32 { // use option when it's valid for the user not to provide a value for a param. Like using a default value for a param in python
//         match number {
//             Some(number) => number * 2,
//             None => 0
//         }
//     }

//     let some_value: i32 = double_or_zero(Some(10));
//     let none_input: Option<i32> = None;

//     println!("Function call with Some(10) as argument: {}\nFunction call with none as input: {:?}", some_value, none_input)
// }

// Exercise 1: Handling Option with if let
// Task:
// Write a function called print_username that accepts an Option<&str>.

// If the value is Some(name), print "Username is: <name>".
// If the value is None, print "No username provided."
// Use an if let statement to destructure the Option in your function.

// fn main() {

//     fn print_username(condition: Option<&str>) {
//         if let Some(value) = condition {
//             println!("Username is: {}", value)
//         } else {
//             print!("No username provided")
//         }
//     }

//     let test_var1: Option<&str> = Some("Sheepdog0331");
//     let test_var2: Option<&str> = None;

//     print_username(test_var1);
//     print_username(test_var2);
// }

// Define an enum named Operation with the following variants:

// Add(i32, i32)
// Subtract(i32, i32)
// Multiply(i32, i32)
// Write a function called perform_addition that takes an Operation value and uses an if let statement to check if the operation is an Add variant.

// If it is, compute the sum of the two numbers and print "Sum: <result>".
// Otherwise, print "Not an addition operation."

// enum Operation {
//     Add(i32, i32),
//     Subtract(i32, i32),
//     Multiply(i32, i32)
// }

// fn main() {

//     fn perform_addition(condition: Operation) {
//         if let Operation::Add(x, y) = condition {
//             let sum: i32 = x + y;
//             println!("Sum: {}", sum)
//         } else {
//             print!("Not an addition operation")
//         }
//     }

//     let test_var1: Operation = Operation::Add(2, 2);
//     let test_var2: Operation = Operation::Multiply(4, 4);

//     perform_addition(test_var1);
//     perform_addition(test_var2);
// }

// Define an enum called Message with these variants:

// Quit
// Move { x: i32, y: i32 }
// Write(String)
// ChangeColor(i32, i32, i32)
// In your main function, create a variable of type Message with one of the variants (choose any). 
// Then, use an if let statement to check if the message is of the Write variant:
    // If it is, print "Message: <contents>".
    // Otherwise, do nothing (or you can print "Not a Write message" if you prefer).

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32)
// }

// fn main() {
//     let test_var1: Message = Message::Write(String::from("You're correct"));

//     if let Message::Write(value) = test_var1 {
//         println!("Message: {}", value)
//     } else {
        
//     }
// }