use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    const PROGRAMS: [&str; 1] = ["guess_game"];
    let program = args.last().expect("Infussient arguments").as_str();
    match program {
        "guess_game" => guess_game(),
        _ => {
            println!(
                "{}",
                "Welcome to @KD_MARK Rusty ground !\n Select and run programs with the following commands: "
                    .blue()
            );
            for pp in PROGRAMS {
                println!("{}", format!("> cargo run {}", pp).cyan());
            }
            if  args.len() > 1 {
                println!("{}", format!("# {} program not found !", program).red());
                return;
            }
        }
    }
}

fn guess_game() {
    println!("Guess the number !");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".yellow()),
            Ordering::Greater => println!("{}", "Too big!".yellow()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
