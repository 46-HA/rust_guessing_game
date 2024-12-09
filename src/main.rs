use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Guessing Game!");

    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("the secret number is: {random_number}");
    loop {
    println!("Enter a number: ");
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    println!("You guessed: {}", guess);

    match guess.cmp(&random_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => { println!("You win!");
        break; }
    }
}

}
