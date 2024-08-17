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
    let programs = [
        "guess_number",
        "guess_word",
        "geometry",
        "todo_list",
        "statistics",
    ];
    let program = args.last().expect("Infussient arguments").as_str();
    match program {
        "guess_number" => guess_game::guess_number(),
        "guess_word" => guess_game::guess_word(),
        "geometry" => geometry::geometry(),
        "todo_list" => todo_list::todo_list(),
        "statistics" => match collections::process_stats() {
            Err(err) => eprintln!("{}", err.to_string().red()),
            Ok(()) => return,
        },
        _ => {
            println!(
                "{}",
                "Welcome to @KD_MARK Rusty ground !\n Select and run programs with the following commands: "
                    .blue()
            );
            for pp in programs {
                println!("{}", format!("> cargo run {}", pp).cyan());
            }
            if args.len() > 1 {
                eprintln!("{}", format!("# {} program not found !", program).red());
                return;
            }
        }
    }
}

mod collections;
mod geometry;
mod guess_game;
mod todo_list;
