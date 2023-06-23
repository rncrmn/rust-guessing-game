#![deny(clippy::all)]

use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("=== The Guessing Game ===");

    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    let mut guess_attempt: u32 = 0;

    loop {
        guess_attempt += 1;

        print!("\nPlease type in your guess: ");
        io::stdout().flush().unwrap();

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please type a number".red());
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                println!("It took you {} tries", guess_attempt.to_string().yellow());
                break;
            }
        }
    }
}
