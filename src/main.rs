use std::io;

fn main() {
    println!("What's your guess (1-100)");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to get number");

    println!("Input given is {}",guess);
}