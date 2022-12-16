use rand::Rng; //Used to generate random
use std::cmp::Ordering; // Ordering is a result of comparing two inputs
use std::io;
use colored::*;

fn main() {
    println!("Guessing Game");

    println!("Please input your guess.");
    
        let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    
        println!("Guessed number: {}", secret_number);
    
    loop {

        let mut user_input_guess: String = String::new();

        io::stdin()
            .read_line(&mut user_input_guess)
            .expect("Error reading input");

        let user_input_guess: u32 = match user_input_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match user_input_guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            }
        }
    }


    

    // let guessing_number: u32 = user_input_guess
    //     .trim()
    //     .parse()
    //     .expect("Invalid input from user");

    // println!("Guessed number: {}", guessed_number);

    // loop {
    //     gussing_games(guessed_number, gussing_number);
    //     break;
    // }
}

// fn guessing_games(guessed: u32, guessing: u32) {
//     if guessing == guessed {
//         return println!("You guessed right! {guessing}");
//     } else if guessing < guessed {
//         return println!("You guessed too small! {}", guessing);
//     } else if guessing > guessed {
//         return println!("You guessed too big! {}", guessing);
//     }
// }
