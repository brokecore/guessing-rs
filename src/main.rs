use std::io;
use rand::Rng;
use inquire::Select;


pub fn greetings(){
    // rules & welcome message
    println!("Hello, welcome to the guessing game");
    println!("Here are the rules:");
    println!("1. You have 3(hard)-10(easy) chances to guess the correct number");
    println!("2. The difficulty either increases or decreases your chances");
    println!("3. The number you pick is inbetween 1-100");
}

pub fn menu(){ //cli menu
    let options = vec!["easy", "medium", "hard"];
    let choices = Select::new("please pick
     you gamemode.", options)
        .prompt()
        .unwrap();

    match choices {
        "easy" => game(10),
        "medium" => game(5),
        "hard" => game(3),
        _ => {}
    }
}

fn game(chances: u8){
        let mut rng = rand::rng();
        let number: u8 = rng.random_range(1..100); // the random number

        println!("You have {} chances.", chances);

        for i in 0..chances {
            println!("Please insert your number here:");

            let mut guess = String::new();

            let _ = io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read guess");

            let guess: u8 = match guess.trim().parse() {
                Ok(num) => num,
              Err(_) => {
                println!("enter a valid number (1-100.)");
                continue;
            }
        };
        if guess == number.into() {
            println!("you win!");
            return; //exit game if won
        } else {
            println!("wrong")
        }
    }
    println!("the corrent number was {}", number);
}

fn main() {
    greetings();
    menu();
}
