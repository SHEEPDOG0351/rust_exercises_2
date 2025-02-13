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

fn main() {
    
}