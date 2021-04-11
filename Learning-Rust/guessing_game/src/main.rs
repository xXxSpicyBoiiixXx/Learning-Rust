//use std::io;
//use std::cmp::Ordering;

/*
 * Instead of using what is above we can combine them
 * as shown below. 
 */

use std::{cmp::Ordering, io};
use rand::Rng;

// Global operator 
use std::collections::*;


fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1,101);

loop {
    // println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => {
            println!("Correct!");
            break;
            }
        }
    }
}
