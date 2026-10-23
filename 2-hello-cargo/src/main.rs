use rand::prelude::*;
use std::io;

fn main() {
    const LOWER: usize = 1;
    const UPPER: usize = 100;
    let secret_number: usize = rand::rng().random_range(LOWER..=UPPER);
    println!("secret_number={secret_number}");
    loop {
        print!("guess the number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read input");

        let guess: usize = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess > secret_number {
            println!("lower");
        } else if guess < secret_number {
            println!("higher");
        } else {
            println!("exact!");
            return;
        }
    }
}
