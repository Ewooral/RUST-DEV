use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 10);
    println!("The secret number is : {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You've guessed right, you win!".green());
                break;
            
            }
        }
    }
}

// Note '::' in Rust is used for namespace resolution to access items within modules,
// traits, or associated types, and it's also used for method call syntax to call associated
// functions and methods on types and instances
