use std::io::stdin;
use rand::{Rng, thread_rng};
use colored::Colorize;
use std::cmp::Ordering;

fn main() {
    let mut user_name:String = String::new();
    let secret_number: u8 = thread_rng().gen_range(1..=100);
    println!("{}", "Welcome to the guessing game.".on_bright_cyan());
    
    println!("Enter your name:");
    match stdin().read_line(&mut user_name) {
        Ok(_) => {
            user_name = user_name.trim().to_owned();
            println!("Welcome, {}!", user_name.blue());
        },
        Err(_) => println!("Can't read name.")
    };
    
    loop {
        let mut user_guess:String = String::new();

        println!("Enter your guess (1-100) :");
        match stdin().read_line(&mut user_guess) {
            Ok(_) => {
                let user_guess: u8 = match user_guess.trim().parse() {
                    Ok(v) => {
                        println!("You guessed {}", v);
                        v
                    }
                    Err(_) => {
                        println!("Enter valid number between 1-100.");
                        continue;
                    }
                };

                match user_guess.cmp(&secret_number) {
                    Ordering::Equal => {
                        println!("Congrats, {}, {}!", user_name.blue(), "YOU WIN".green());
                        break;
                    },
                    Ordering::Greater => println!("{}", "Your guess is too high".red()),
                    Ordering::Less => println!("{}", "Your guess is too low".red())
                }
            }
            Err(_) => {
                println!("Can't read guess.");
                continue;
            }
        }
    }
}