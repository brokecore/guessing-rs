use std::io;
use rand::Rng;
use inquire::Select;
use colored::*;

pub fn greetings() {
    // rules & welcome message
    println!("Hello, welcome to the guessing game");
    println!("Here are the rules:");
    println!("1. You have 3(hard)-10(easy) chances to guess the correct number");
    println!("2. The difficulty either increases or decreases your chances");
    println!("3. The number you pick is in between 1-100");
}

pub fn menu() { //cli menu
    let options = vec!["easy", "medium", "hard"];
    let choices = Select::new("Please pick your game mode.", options)
        .prompt()
        .unwrap();

    match choices {
        "easy" => game(10),
        "medium" => game(5),
        "hard" => game(3),
        _ => {}
    }
}

fn game(chances: u8) {
    let mut rng = rand::thread_rng();
    let number: u8 = rng.gen_range(1..101); // the random number
    let mut count = 0;

    println!("You have {} chances.", chances);

    for _ in 0..chances {
        println!("Please insert your number here:");

        let mut guess = String::new();

        let _ = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number (1-100).");
                continue;
            }
        };
        count += 1;

        if guess == number {
            println!("You win!");
            println!("You took {} guesses", count);
            return; //exit game if won
        } else {
            println!("{}", "Wrong!".red());

            if number > guess {
                println!("{}", format!("The number is {} than your guess", "greater".green()));
            } else if number < guess {
                println!("{}", format!("The number is {} than your guess", "less".red()));
            }
        }
    }
    println!("The correct number was {}", number);
}

fn main() {
    greetings();
    menu();
}
