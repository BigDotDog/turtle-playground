// Number Guessing Game in Rust
// This project teaches functions, modules, and basic input/output

use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

// A module for handling user input
mod user_input {
    use std::io::{self, Write};
    
    // Function to get a number from the user
    pub fn get_number() -> u32 {
        loop {
            // Print a prompt
            print!("Enter your guess: ");
            // Make sure the prompt is displayed
            io::stdout().flush().expect("Failed to flush stdout");
            
            // Create a new string to store the input
            let mut guess = String::new();
            
            // Read a line from the user
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            
            // Try to convert the input to a number
            match guess.trim().parse() {
                Ok(num) => return num,
                Err(_) => println!("Please enter a valid number!"),
            }
        }
    }
}

// Function to check if the guess is correct
fn check_guess(guess: u32, secret_number: u32) -> bool {
    // Compare the guess to the secret number
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small!");
            false
        }
        Ordering::Greater => {
            println!("Too big!");
            false
        }
        Ordering::Equal => {
            println!("You win!");
            true
        }
    }
}

// Function to get difficulty level from user
fn get_difficulty() -> u32 {
    println!("Choose a difficulty level:");
    println!("1: Easy (1-10)");
    println!("2: Medium (1-100)");
    println!("3: Hard (1-1000)");
    
    loop {
        print!("Enter your choice (1-3): ");
        io::stdout().flush().expect("Failed to flush stdout");
        
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        
        match choice.trim().parse() {
            Ok(1) => return 10,
            Ok(2) => return 100,
            Ok(3) => return 1000,
            _ => println!("Please enter 1, 2, or 3!"),
        }
    }
}

fn main() {
    println!("Welcome to the Guessing Game!");
    
    // Get the difficulty level
    let max_number = get_difficulty();
    
    // Generate a random number
    let secret_number = rand::thread_rng().gen_range(1..=max_number);
    
    println!("I'm thinking of a number between 1 and {}", max_number);
    
    // Track the number of guesses
    let mut guesses = 0;
    
    // Main game loop
    loop {
        // Get a guess from the user
        let guess = user_input::get_number();
        guesses += 1;
        
        // Check if the guess is correct
        if check_guess(guess, secret_number) {
            break;
        }
    }
    
    println!("You found the number in {} guesses!", guesses);
}