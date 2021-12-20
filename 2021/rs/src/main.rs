#![allow(unused)]
use std::env;
use std::fs;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

fn main() {
    let args: Vec<String> = env::args().collect();

    calculate_answer("../inputs/01.txt", day_01::depth_measurements);
    run_exercise("../inputs/02.txt", day_02::sub_position);
    run_exercise("../inputs/03.txt", day_03::binary_diagnostic);
    run_exercise("../inputs/04.txt", day_04::play_bingo);
    run_exercise("../inputs/05.txt", day_05::hydrothermal_vents);
    calculate_answer("../inputs/06.txt", day_06::lanternfish);
    calculate_answer("../inputs/07.txt", day_07::crab_target_alignment);
    calculate_answer("../inputs/08.txt", day_08::seven_seg_decode);
    calculate_answer("../inputs/09.txt", day_09::tube_smoke);
}

pub enum Answer {
    U32(u32, u32),
    U64(u64, u64),
    I64(i64, i64),
}

fn run_exercise(input_file: &str, calculator: fn(Vec<String>) -> (u32, u32)) {
    let mut input = read_input(input_file);

    let (first_answer, second_answer) = calculator(input);

    println!("Results from {}:", input_file);
    println!("First answer is {}, second answer is {}.",
        first_answer, second_answer);
}

fn calculate_answer(input_file: &str, calculator: fn(Vec<String>) -> Answer) {
    let mut input = read_input(input_file);

    let answers = calculator(input);

    let (first_answer, second_answer) = match answers {
        Answer::U32(first, second) => (first.into(), second.into()),
        Answer::I64(first, second) => (first.try_into().unwrap(), second.try_into().unwrap()),
        Answer::U64(first, second) => (first, second),
    };

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
