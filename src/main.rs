mod users;

use std::io::{self, BufRead};
use crate::users::User;

fn main() {
    // Create a locked handle to standard input for efficient reading
    let reader = io::stdin().lock();
    let mut users: Vec<User> = Vec::new();

    println!("Enter user data in the format 'Name Age'. Type 'exit' to finish.");

    for line in reader.lines() {
        match line {
            Ok(ref input) => {
                let trimmed = input.trim();

                // Check if the user wants to exit
                if trimmed.eq_ignore_ascii_case("exit") {
                    break;
                }

                // Split the input into parts based on whitespace
                let parts: Vec<&str> = trimmed.split_whitespace().collect();

                // Ensure that exactly two parts are provided: name and age
                if parts.len() != 2 {
                    println!("Invalid input format. Please enter data as 'Name Age'.");
                    continue;
                }

                let name = parts[0].to_string();

                // Attempt to parse the age as an i32
                let age = parts[1].parse::<i32>().unwrap_or_else(|_| {
                    println!("Invalid age provided. Setting age to 0.");
                    0
                });

                // Create a new User object and add it to the vector
                let user = User::new(name, age);
                users.push(user);

                // Optionally limit the number of users to 5
                if users.len() >= 2 {
                    println!("Reached the maximum number of users (2).");
                    print!("\x1B[2J\x1B[1;1H");
                    break;
                }
            }
            Err(error) => println!("Error reading line: {}", error),
        }
    }

    // Display the collected user data
    for user in &users {
        println!("You are {} && You are {} years old!", user.name, user.age);
    }
}

