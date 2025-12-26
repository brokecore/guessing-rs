use std::io;
use rand::Rng;
use inquire::Select;


pub fn greetings(){
    // rules & welcome message
    println!("Hello, welcome to the guessing game");
    println!("Here are the rules:");
    println!("1. You have 5 chances to guess the correct number");
    println!("2. The difficulty either increases or decreases your chances");
    println!("3. The number you pick is inbetween 1-100");
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
    for i in 0..10 { //stops after 10 tries
        let mut rng = rand::rng();
        let number: u8 = rng.random_range(1..100);

        println!("You have 10 chances.");
        println!("Please insert your number here:");

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
            println!("wrong") //TODO: add different message after loop break
        };
        
    }
}

fn main() {
    greetings();
    menu();
}
