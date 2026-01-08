use std::fs;
use std::path::Path;

pub mod days;
use crate::days::day1::day1::{calculate_password, calculate_password2};

pub fn run(day: u32) {
    match day {
        1 => day1_soln(),
        2 => day2_soln(),
        3 => day3_soln(),
        4 => day4_soln(),
        5 => day5_soln(),
        _ => println!("That day doesn't exist!"),
    }
}

pub fn day1_soln() {
    // call the password calculator on day1_input.txt
    println!("Getting turn dial input data and calculating password!");
    let input_path = Path::new("data/day1_input.txt");
    let input = fs::read_to_string(input_path)
                    .expect("Should have been able to read file");
    let password = calculate_password(&input);
    println!("The solution is: {password}");
    println!("Wait! There is more! Need to use different password calculating algorithm");
    let password = calculate_password2(&input);
    println!("The new solution is: {password}");
}

pub fn day2_soln() {
    unimplemented!();
}

pub fn day3_soln() {
    unimplemented!();
}

pub fn day4_soln() {
    unimplemented!();
}

pub fn day5_soln() {
    unimplemented!();
}

pub fn day6_soln() {
    unimplemented!();
}

pub fn day7_soln() {
    unimplemented!();
}

pub fn day8_soln() {
    unimplemented!();
}

pub fn day9_soln() {
    unimplemented!();
}

pub fn day10_soln() {
    unimplemented!();
}

pub fn day11_soln() {
    unimplemented!();
}

pub fn day12_soln() {
    unimplemented!();
}

