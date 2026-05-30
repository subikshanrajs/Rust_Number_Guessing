use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Welcome to Guess the Number Game!");
    let secret_num = rand::thread_rng().gen_range(1..1000);

    loop {
        println!("You Can Enter your guess between 1 and 1000:");

        let mut your_guess: String = String::new();
        io::stdin()
            .read_line(&mut your_guess)
            .expect("Failed to read line");

        println!("You guessed: {}", your_guess);
        let your_guess: u32 = your_guess.trim().parse().expect("Please type a number!");
        match your_guess.cmp(&secret_num) {
            Ordering::Less => println!("Try Higher"),
            Ordering::Greater => println!("Try Lower"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
