use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 10);
    println!("The secret number is : {}", secret_number);
    println!("Please input your guess.");

    let mut guess  = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You've guessed right, you win!"),
    }

}

// Note '::' in Rust is used for namespace resolution to access items within modules, 
// traits, or associated types, and it's also used for method call syntax to call associated 
// functions and methods on types and instances
