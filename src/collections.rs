use std::{
    collections::HashMap,
    io::{self, Error},
    ops::Div,
};

use colored::Colorize;

struct Stats(Vec<i32>);
impl Stats {
    fn collect_data(&mut self) {
        println!(
            "{}",
            "Enter a list of integers separated by commats !".blue()
        );
        let mut list_of_integers = String::new();
        io::stdin()
            .read_line(&mut list_of_integers)
            .expect("Failed to read line");
        let mut data: Vec<i32> = Vec::new();
        for str in list_of_integers.trim().split_terminator(',') {
            let integer: i32 = str.trim().parse().expect("Failed to parse integer !");
            data.push(integer);
        }
        data.sort();
        *self = Stats(data);
    }

    fn median(&self) -> Result<i32, Error> {
        let Stats(data) = self;
        match data.get(data.len().div(2)) {
            Some(median) => Ok(*median),
            None => Err(Error::new(io::ErrorKind::NotFound, "Median not found !")),
        }
    }

    fn mode(&self) -> i32 {
        let mut occurencies = HashMap::new();
        let Stats(data) = self;
        for value in data {
            let occurency = occurencies.entry(value).or_insert(0);
            *occurency += 1;
        }
        let mut mode = *data.get(0).expect("Mode cannot be called on empty data");
        let mut mode_occurency = occurencies[&mode];
        for (val, occ) in occurencies {
            (mode, mode_occurency) = if mode_occurency < occ {
                (*val, occ)
            } else {
                (mode, mode_occurency)
            };
        }
        mode
    }
}

pub fn process_stats() -> Result<(), Error> {
    println!("{}", "Let's do some stats.".blue());
    let mut stats = Stats(Vec::new());
    stats.collect_data();
    let stats_median = stats.median()?;
    let stats_mode = stats.mode();
    println!(
        "{}",
        format!(
            "Your stats and mode are: (median: {}, mode: {})",
            stats_median, stats_mode
        )
    );
    Ok(())
}
