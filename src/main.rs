use colored::*;
use rand::Rng;
use std::{
    io,
    cmp::Ordering,
    ops::{Div, Mul, Sub},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    const PROGRAMS: [&str; 2] = ["guess_game", "geometry"];
    let program = args.last().expect("Infussient arguments").as_str();
    match program {
        "guess_game" => guess_game::guess_number(),
        "geometry" => geometry::geometry(),
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
