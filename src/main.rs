use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

/// Guessing game implementation in Rust!
/// # Credits
/// Rust course in [the Rust book](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {}", secret_number);

    loop {
        print!("Please, input a number: ");
        io::stdout().flush().expect("Error flushing output :(");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}. Please enter a number", err);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
