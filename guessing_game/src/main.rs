use std::io::stdin;
// use rand::Rng;
use rand_core::{RngCore, OsRng};

fn main() {
    println!("Guess the number!");

    

    let mut guess: String = String::new();
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    let secret_number = OsRng.next_u32();

    println!("The secret number is: {secret_number}");

    let mut status: bool = false;


    while status == false {
        println!("Please input your guess:");
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {}", guess);

        let numeric_guess: u32 = guess.trim().parse::<u32>().expect("Please type a number!");

        if numeric_guess == secret_number {
            println!("You win!");
            status = true;
        } else {
            println!("Try again!");
            guess.clear(); // clear the string to start fresh
        }

        

        
    }    
}