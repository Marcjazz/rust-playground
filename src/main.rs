use colored::*;
use rand::Rng;
use std::{
    cmp::Ordering,
    io,
    ops::{Div, Mul, Sub},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    const PROGRAMS: [&str; 4] = ["guess_number", "guess_word", "geometry", "statistics"];
    let program = args.last().expect("Infussient arguments").as_str();
    match program {
        "guess_number" => guess_game::guess_number(),
        "guess_word" => guess_game::guess_word(),
        "geometry" => geometry::geometry(),
        "statistics" => match collections::process_stats() {
            Err(err) => {
                println!("{}", err.to_string().red())
            }
            Ok(()) => return,
        },
        _ => {
            println!(
                "{}",
                "Welcome to @KD_MARK Rusty ground !\n Select and run programs with the following commands: "
                    .blue()
            );
            for pp in PROGRAMS {
                println!("{}", format!("> cargo run {}", pp).cyan());
            }
            if args.len() > 1 {
                println!("{}", format!("# {} program not found !", program).red());
                return;
            }
        }
    }
}

mod guess_game;

pub mod geometry;

mod collections;
