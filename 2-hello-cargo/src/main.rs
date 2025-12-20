use std::{cmp::Ordering, io};

use rand::Rng;

const LOWER_BOUNDARY: u32 = 1;
const UPPER_BOUNDARY: u32 = 100;

fn main() {
    let secret_number = rand::thread_rng().gen_range(LOWER_BOUNDARY..=UPPER_BOUNDARY);

    println!("Guess the number!");

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        // Shadow the variable guess so the type changes for the remaining scope.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

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
