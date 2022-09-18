use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Enter your name");
    let mut user_name = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is {}", secret_number);
    
    io::stdin()
    .read_line(&mut user_name)
    .expect("Can't read name.");
    
    
    loop {
        let mut user_guess = String::new();
        println!("Enter your number guess ( 1-100 )");

        io::stdin()
            .read_line(&mut user_guess)
            .expect("Can't read guess.");
        
        let user_guess: i32 = match user_guess.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Please enter valid number");
                continue
            }
        };

        match user_guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win, {}", user_name.trim());
                break;
            },
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!")
        }
    }
}