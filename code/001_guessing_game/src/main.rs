extern crate rand;

// The standard library + io module
use std::io;
// Random
use rand::Rng;
// Comparisons
use std::cmp::Ordering;

fn main() -> () {
    println!("Guess the number!");

    let secret_nr = rand::thread_rng().gen_range(1, 101);

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Using match: if the number can be converted from string to number, guess = num; otherwise begin
        // another iteration.
        // parse() returns a Result which is an enum: Has Ok and Err.
        let guess: u32 = match guess.trim().parse() {
            Ok(val)     => val,
            Err(_)      => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_nr) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                                    println!("You win!");
                                    break;
                                },
        }

    }
}

