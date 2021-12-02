#![allow(unused)]
// use std::env;
use std::fs;

fn main() {
    day_one_first("../inputs/01_1.txt");
    day_one_second("../inputs/01_1.txt");
}

fn day_one_first(infile: &str) {
    let input = fs::read_to_string(infile)
        .expect("Something went wrong reading the file");

    let measurements = input.lines().map(|x| x.parse::<u32>().unwrap());

    let mut last_measurement: Option<u32> = None;
    let mut num_increases: u32 = 0;

    for measurement in measurements {
        if let Some(last) = last_measurement {
            if measurement > last {
                num_increases += 1;
            }
        }
        last_measurement = Some(measurement);
    }
    println!("There were {} increases", num_increases);
}

fn day_one_second(infile: &str) {
    let input = fs::read_to_string(infile)
        .expect("Something went wrong reading the file");

    let measurements: Vec<u32> = input
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    const WINDOW_SIZE: usize = 3;
    let mut windowed_measurements: Vec<u32> = Vec::new();

    for i in 0..=measurements.len()-WINDOW_SIZE {
        //println!("we are in window index {}", i);

        let window = &measurements[i..i+WINDOW_SIZE];
        let window_sum = window.iter().sum::<u32>();
        //println!("sum of window: {}", window_sum);

        windowed_measurements.push(window_sum);
    }

    let count = windowed_measurements.len();
    //println!("there are {} windowed_measurements", count);

    let mut last_measurement: Option<u32> = None;
    let mut num_increases: u32 = 0;

    for measurement in windowed_measurements {
        if let Some(last) = last_measurement {
            if measurement > last {
                num_increases += 1;
            }
        }
        last_measurement = Some(measurement);
    }
    println!("There were {} windowed increases", num_increases);
}
