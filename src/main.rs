use std::io;
use rand::Rng;
use inquire::Select;

pub fn greetings(){
    // rules & welcome message
    println!("Hello, welcome to the guessing game");
    println!("Here are the rules:"); //TODO: insert rules later
}

pub fn menu(){ //cli menu
    let options = vec!["easy", "medium (doesnt work)", "hard (doesnt work)"];
    let choices = Select::new("please pick you gamemode.", options)
        .prompt()
        .unwrap();

    match choices {
        "easy" => gameeasy(),
        _ => {}
    }
}

fn gameeasy(){
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
            println!("you win!");
            break
        } else {
            println!("wrong")
        };
        
    }
}

fn main() {
    greetings();
    menu();
}
