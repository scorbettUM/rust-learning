use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Guess the number:");

        println!("Please input your guess...");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");


        let parsed_guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match parsed_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        };

        println!("You guessed: {guess}");

    }
}
