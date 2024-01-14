use super::*;
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

use rand::thread_rng as random;

pub fn guess_number() {
    println!("Guess the number !");

    let secret_number = random().gen_range(1..=100);

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

pub fn guess_word() {
    let mut words: Vec<String> = Vec::new();
    load_words(&mut words);
    let secret_index = random().gen_range(0..words.len());
    let secret_word = match words.get(secret_index) {
        Some(word) => word,
        None => {
            eprintln!("Secret index out of bound {} ", secret_index);
            return;
        }
    };
    let word_length = secret_word.len();
    println!(
        "{}",
        format!(
            "Guess the word !! \n Note: The secret word is related to software engineering \n> Hint: it starts with `{}` and has a length of {}",
            secret_word.chars().nth(0).unwrap(),
            word_length
        )
        .blue()
    );

    let mut attempts = 0;
    let mut hints: Vec<usize> = Vec::new();
    loop {
        attempts += 1;
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        if guess.trim().eq(&secret_word.to_string()) {
            println!(
                "{}",
                format!(
                    "You found it! score: {}/{}",
                    word_length - attempts,
                    word_length
                )
                .green()
            );
            if stop_game() {
                break
            } else {
                guess_word()
            }
        } else {
            if attempts == word_length {
                println!(
                    "{}",
                    format!("Game over !! The word was {}", secret_word).red()
                );
                if stop_game() {
                    break
                } else {
                    guess_word()
                }
            } else {
                if attempts == 1 {
                    hints.push(0);
                    for _ in 0..word_length {
                        hints.push(random().gen_range(1..word_length));
                    }
                }
                let hint_str = get_hint_str(&secret_word, &hints[0..=attempts]);
                println!(
                    "{}",
                    format!("Hint: {} \n Give it another try: ", hint_str).cyan()
                );
            }
        }
    }
}

fn get_hint_str(secret_word: &str, hints: &[usize]) -> String {
    let mut hint_str = String::new();
    for (i, char) in secret_word.char_indices() {
        let _ = hint_str.push(if hints.contains(&i) { char } else { '_' });
    }

    hint_str
}

fn stop_game() -> bool {
    loop {
        println!("Try another word? (yes/no)");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        let answer = answer.trim();
        if answer.eq("no") || answer.eq("n") || answer.eq("N") {
            break true;
        } else if answer.eq("yes") || answer.eq("y") || answer.eq("Y") {
            break false;
        } else {
            println!("Wrong option !!! ")
        }
    }
}

fn load_words(words: &mut Vec<String>) {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let dictionary_path = current_dir.join("dictionary.txt");
    // Open the file in read-only mode
    let file = File::open(dictionary_path).expect("Failed to load dictionary.");

    // Create a buffered reader to read the file line by line
    let reader = BufReader::new(file);

    // Iterate over the lines in the file
    for line in reader.lines() {
        // Handle each line as needed (here, we print it)
        match line {
            Ok(word) => words.push(word),
            Err(_) => {
                println!("{}", "Failed to read line.".red())
            }
        }
    }
}
