use std::io;
use advent_of_code_2025::run;

fn main() {
    println!("Which Advent of Code day's solution to run?");

    let mut day: String = String::new();

    io::stdin()
        .read_line(&mut day)
        .expect("Failed to read line");

    let day: u32 = day.trim().parse().expect("Need to input a number");

    run(day);
    
}

