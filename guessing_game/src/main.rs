use std::io::stdin;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    

    let mut guess: String = String::new();
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    let mut status: bool = false;


    while status == false {
        println!("Please input your guess:");
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {}", guess);

        let numeric_guess: i32 = guess.trim().parse::<i32>().expect("Please type a number!");

        if numeric_guess == secret_number {
            println!("You win!");
            status = true;
        } else {
            println!("Try again!");
            guess.clear(); // clear the string to start fresh
        }

        

        
    }    
}