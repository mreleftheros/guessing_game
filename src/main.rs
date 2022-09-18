use std::io;

fn main() {
    println!("Welcome to the guessing game!");
    println!("Enter your name");
    let mut player_name = String::new();

    io::stdin()
        .read_line(&mut player_name)
        .expect("Can't read player name.");
    
    println!("Your name: {}", player_name);
}
