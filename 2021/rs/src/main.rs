#![allow(unused)]
use std::env;
use std::fs;

mod day_01;
mod day_02;
mod day_03;
mod day_04;

fn main() {
    let args: Vec<String> = env::args().collect();

    run_exercise("../inputs/01.txt", day_01::depth_measurements);
    run_exercise("../inputs/02.txt", day_02::sub_position);
    run_exercise("../inputs/03.txt", day_03::binary_diagnostic);
    run_exercise("../inputs/04.txt", day_04::play_bingo);
}

fn run_exercise(input_file: &str, calculator: fn(Vec<String>) -> (u32, u32)) {
    let mut input = read_input(input_file);

    let (first_answer, second_answer) = calculator(input);

    println!("Results from {}:", input_file);
    println!("First answer is {}, second answer is {}.",
        first_answer, second_answer);
}

fn read_input(input_file: &str) -> Vec<String> {
    let input = fs::read_to_string(input_file)
        .expect("Something went wrong reading the input file")
        .lines().map(String::from).collect();
    input
}
