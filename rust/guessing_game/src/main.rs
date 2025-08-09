use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    let secret: u8 = rand::rng().random_range(..101);
    println!("The secret is {secret}");

    loop {
        let mut guess = String::new();
        println!("Enter the number you guessed:");
        stdin()
            .read_line(&mut guess)
            .expect("input cannot be read!");

        let guess: u8 = guess.trim().parse().expect("Cannot be parsed");
        match guess.cmp(&secret) {
            Ordering::Less => {
                println!("{}", "TOO SMALL".yellow());
                continue;
            }
            Ordering::Greater => {
                println!("{}", "TOO BIG".red());
                continue;
            }
            Ordering::Equal => println!("{}", "You WON".green()),
        }

        if guess == secret {
            break;
        }
    }
}
