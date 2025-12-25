use std::io;
use rand::Rng;

pub fn greetings(){
    // rules & welcome message
    println!("Hello, welcome to the guessing game");
    println!("Here are the rules:"); //TODO: insert rules later
}

pub fn gameeasy(){
    loop {
        let mut rng = rand::rng();
        let number: u8 = rng.random_range(0..10);
    
        let mut guess = String::new();

        let _ = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("enter a number.");
                continue;
            }
        };

        if guess == number.into() {
            println!("you win!")
        } else {
            println!("wrong")
        };
        
    }
}

fn main() {
    greetings();
    gameeasy();
}
