use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to Guess the Number Game!");

    let mut wins = 0;
    let mut losses = 0;

    loop {
        let secret_num = rand::thread_rng().gen_range(1..1000);
        let mut attempts = 0;
        const MAX_ATTEMPTS: u32 = 10;

        println!("\nNew round started! Guess the number between 1 and 1000.");
        println!("You have {} attempts.", MAX_ATTEMPTS);

        let round_result = loop {
            if attempts >= MAX_ATTEMPTS {
                break RoundResult::Lose;
            }

            println!("Enter your guess (or type 'quit' to stop):");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();

            if input.eq_ignore_ascii_case("quit") {
                break RoundResult::Quit;
            }

            let guess = match input.parse::<u32>() {
                Ok(num) if num >= 1 && num <= 1000 => num,
                _ => {
                    println!("Please enter a valid number from 1 to 1000 or 'quit'.");
                    continue;
                }
            };

            attempts += 1;
            println!("You guessed: {}", guess);

            match guess.cmp(&secret_num) {
                Ordering::Less => println!("Try higher."),
                Ordering::Greater => println!("Try lower."),
                Ordering::Equal => break RoundResult::Win,
            }
        };

        match round_result {
            RoundResult::Win => {
                wins += 1;
                println!("You win! Total wins: {} | losses: {}", wins, losses);
            }
            RoundResult::Lose => {
                losses += 1;
                println!("You lose! The number was {}.", secret_num);
                println!("Total wins: {} | losses: {}", wins, losses);
            }
            RoundResult::Quit => {
                println!("Goodbye! Final record: {} wins, {} losses.", wins, losses);
                break;
            }
        }

        println!("Would you like to play again? (yes/no)");
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("Failed to read line");
        let play_again = play_again.trim();

        if !play_again.eq_ignore_ascii_case("yes") && !play_again.eq_ignore_ascii_case("y") {
            println!("Thanks for playing! Final record: {} wins, {} losses.", wins, losses);
            break;
        }
    }
}

enum RoundResult {
    Win,
    Lose,
    Quit,
}
