use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret = rand::thread_rng().gen_range(1,100);
        
    loop {
        println!("What's your guess (1-100)");
    
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Only number is accepted :(, Try again!");
                continue;
            },
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("You Guess, is less than secret."),
            Ordering::Greater => println!("You Guess, is greater than secret."),
            Ordering::Equal => {
                println!("Bingo!. You made it");
                break;
            },
        }
    }

}
