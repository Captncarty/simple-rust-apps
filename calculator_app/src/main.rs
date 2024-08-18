///imports
use std::io;

///main function
fn main() {
    loop {
        println!("Simple Calculator");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Exit\n");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let make_choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //Exit loop
        if make_choice == 5 {
            println!("Exiting...");
            break;
        }

        //taking first input
        println!("Enter first number: ");
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("Failed to read line"); //return choice or return msg
        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };

        //taking second input
        println!("Enter second number: ");
        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).expect("Failed to read line");
        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };

        // match make_choice {
        //     1 => println!("Result: {}", num1 + num2),
        //     2 => println!("Result: {}", num1 - num2),
        //     3 => println!("Result: {}", num1 * num2),
        //     4 => {
        //         if num2 == 0.0 {
        //             println!("Cannot divide by zero");
        //         } else {
        //             println!("Result: {}", num1 / num2);
        //         }
        //     }
        //     _ => println!("Invalid choice"),
        // }

        match make_choice {
            1 => {
                let result = num1 + num2;
                println!("Result: {}\n", result);
            }
            2 => {
                //fixing edge cases and negatives using if
                if num1 > num2 {
                    let result = num1 - num2;
                    println!("Result: {}\n", result);
                } else {
                    let result = num2 - num1;
                    println!("Result: {}\n", result);
                }
            }
            3 => {
                let result = num1 * num2;
                println!("Result: {}\n", result);
            }
            4 => {
                if num2 == 0.0 {
                    println!("Zero division error\n");
                } else {
                    let result = num1 / num2;
                    println!("Result: {}\n", result);
                }
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 5");
            }
        }
    }
}