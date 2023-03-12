use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Psst, here is a hint, it is: {}", secret_number);

    loop {
        println!("\nPlease input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();
        let guess = match guess.to_string().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("'{}' is nog a valid number", guess);
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You got it");
                break;
            }
        }
    }
}