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

pub struct Guess { 
	value: i32,
}

impl Guess {
	pub fn new(value: i32) -> Guess { 
		if value < 1 || value > 100 { 
			panic!("Guess value must be between 1 and 100, got {}.", value);
}

Guess { value }
}

pub fn value(&self) -> i32 { 
	self.value
	}
}

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
    
	/*
	 * we can add negative ranges as input below 
	 * 
	 */


    // let guess: u32 = match guess.trim().parse() {
       let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

   //  println!("You guessed: {}", guess);
	if guess < 1 || guess > 100 {
		println!("The secret number will be between 1 and 100.");
		continue; 
	} 

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
