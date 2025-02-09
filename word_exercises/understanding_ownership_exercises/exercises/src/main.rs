// Write a function named calculate_length that takes a reference to a String (i.e., &String) and returns its length as a usize. 
// In your main function, create a String, pass a reference of it to your function, and print both the original string and its length afterward. 
// This exercise will show that borrowing a value does not transfer its ownership.

// fn main() {
//     fn calculate_length(string: &String) -> usize {
//         string.len()
//     }

//     let s: String = String::from("Hello, Rust!");
//     let len: usize = calculate_length(&s);
    
//     println!("The string '{}' has length {}", s, len);
// }

// Write a function named append_exclamation that takes a mutable reference to a String (i.e., &mut String) and appends an exclamation mark (!) to it.
// In your main function, create a mutable String, call your function passing a mutable reference, and print the modified string.

// fn main() {
//     fn append_exclamation(string: &mut String) -> &String {
//         string.push('!');
//         string
//     }

//     let mut test = String::from("Example String");
//     println!("{:?}", append_exclamation(&mut test));
// }

// Write a function called first_word that takes a string slice (&str) and returns a string slice representing the first word in the given string. 
// A word is defined as a sequence of characters until the first space. 
// Test your function in main by passing a String (converted to a slice) and printing the returned first word.

// fn main() {
//     fn first_word(s: &str) -> &str {
//         if let Some(index) = s.find(' ') {
//             &s[..index]
//         } else {
//             s
//         }
//     }

//     let test_string = String::from("Test String");
//     println!("{}", first_word(&test_string))
// }

// Write a function named print_and_sum that takes a reference to a Vec<i32> and does the following:
    // Prints each element of the vector.
    // Returns the sum of its elements.
// In main, create a vector, pass a reference to the function, print the sum, 
// then print the vector again to confirm that the vector is still accessible (i.e., ownership wasn’t moved).

// fn main() {
//     fn print_and_sum(vector: &Vec<i32>) -> String {
//         let mut sum = 0;
//         for element in vector {
//             println!("Current Num: {}", element);
//             sum += element
//         }
//         format!("Sum: {}", sum)
//     }

//     let test_vec: Vec<i32> = vec![2, 4, 6];
//     let result: String = print_and_sum(&test_vec);
//     println!("{}", result);
// }

// Write a function named sum_slice that accepts a slice of integers (&[i32]) and returns the sum of the elements in the slice. 
// In your main function, create an array of integers, then call your function on different slices of that array (for example, a slice from index 1 to 3),
// and print the results.

// fn main() {
//     fn sum_slice(s: &[i32]) -> i32 {
//         let mut total_sum: i32 = 0;
//         for num in s {
//             println!("Adding {} to the sum...", num);
//             total_sum += num;
//         }
//         println!("Finished calculating sum of slice");
//         total_sum
//     }

//     let test_numbers: [i32; 6] = [2, 5, 3, 4, 7, 8];
//     let first_slice: &[i32] = &test_numbers[0..3];
//     let second_slice: &[i32] = &test_numbers[3..];

//     let first_result: i32 = sum_slice(first_slice);
//     let second_result: i32 = sum_slice(second_slice);

//     println!("Sum of First Slice: {}\n Sum of Second Slice: {}", first_result, second_result);
// }

