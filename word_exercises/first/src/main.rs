// Write a function vec_sum that computes the sum of all values of a Vec<i32>.

// fn main() {
//     let values_to_sum = vec![2, 4, 6, 8];

//     fn sum_of_values(vector: Vec<i32>) -> i32 {
//         let mut sum_var = 0;
//         for value in vector {
//             sum_var += value;
//         }
//         sum_var
//     }
//     println!("{}", sum_of_values(values_to_sum));
// }

// write a function 'vec_print' that takes a vector and prints all its elements. 

// fn main() {
//     let example_vec: Vec<i32> = vec![2, 4, 6, 8];

//     fn print_vector(vector: Vec<i32>) {
//         print!("{:?}", vector)
//     }

//     println!("{:?}", print_vector(example_vec));
// }

// for fun, I thought I would create another function which prints the numbers and doesn't require you to use debug print formatting:

// fn main() {
//     let example_vec: Vec<i32> = vec![2, 4, 6, 8];

//     fn print_vector(vector: Vec<i32>) -> i32 {
//         let mut sum_of_values: i32 = 0;
//         for value in vector {
//             sum_of_values += value
//         }
//         sum_of_values
//     }

//     println!("{}", print_vector(example_vec));
// }

// Write a function named is_even that takes a single integer as a parameter and returns a boolean value indicating whether the number is even. 
// Use an if expression inside your function. 
// In your main function, call is_even with a sample number and print the result using println!.

fn main() {
    let test_num: i32 = 2;
    let second_num: i32 = 1;

    fn is_even(num: i32) -> String {
        if num % 2 == 0 {
            format!("The number {} is even!", num)
        } else {
            format!("The number {} is odd!", num)
        }
    }
    println!("{}", is_even(test_num));
    println!("{}", is_even(second_num));
}

// Write a function called sum_array that accepts an array of five integers and returns the sum of its elements. 
// Use a for loop to iterate over the array elements. 
// In your main function, define an array with five integers, call your function, and print the resulting sum.

fn main() {
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];

    fn sum_array(arr: [i32; 5]) -> i32 {
        let mut sum_of_values: i32 = 0;
        for value in arr {
            sum_of_values += value
        }
        sum_of_values
    }
    println!("{}", sum_array(arr1));
}

// Write a small program that demonstrates variable shadowing. 
// Start by declaring an immutable variable with an initial value. 
// Then, shadow that variable by declaring a new variable with the same name that is derived from the original value (for example, by performing a calculation). 
// Continue shadowing it one more time with another expression. After each shadowing, print the current value of the variable using println! so you can observe the changes.

fn main() {
    let immutable_variable: i32 = 5;
    {
        let mut immutable_variable: i32 = 5;
        immutable_variable += 5;    
        println!("Shadowed Variable: {}", immutable_variable);
    };

    {
        let mut immutable_variable: i32 = 5;
        immutable_variable += 10;
        println!("Seocnd shadowed variable: {}", immutable_variable);
    };
    println!("And the regular variable's value is: {}", immutable_variable);
}