use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let rng_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number, between 1 and 100.");
    let mut guess = String::new();
    loop {
        println!("Input your guess: ");
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Only numbers are allowed!");
                continue;
            }
        };
        print!("Entered guess was: {guess}");
        match guess.cmp(&rng_number) {
            Ordering::Less => println!(", Too small!"),
            Ordering::Greater => println!(", Too big!"),
            Ordering::Equal => {
                println!(". You guessed correctly!");
                break;
            }
        }
    }
    println!("The secret number is {rng_number}");
}
