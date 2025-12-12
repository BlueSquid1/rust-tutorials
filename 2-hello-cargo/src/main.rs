use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Err(_) => {
                println!("Not a number");
                continue;
            }
            Ok(num) => num,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too large"),
            Ordering::Equal => {
                println!("equal");
                break;
            }
        };
    }
}
