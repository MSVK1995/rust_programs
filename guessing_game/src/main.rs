use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let mut rng = rand::rng();
    let secret_number: u32 = rng.random_range(1..=100);

    let mut guess_count: i32 = 0;

    loop {
        println!(
            "Please input your guess{}between 1 and 100 (inclusive): ",
            if guess_count == 0 { " " } else { " again, " }
        );

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Attempt to parse the input into a u32. Use a match expression to handle errors.
        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => {
                if num < 1 || num > 100 {
                    println!("Your guess must be between 1 and 100 (inclusive).");
                    continue; // if not, prompt for input again
                } else {
                    num
                }
            }
            Err(_) => {
                println!("Please enter a valid number.");
                continue; // restart loop to ask for input again
            }
        };

        // Compare the guess to the secret number.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        guess_count += 1;
    }
}
