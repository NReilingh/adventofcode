#![allow(unused)]
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "1" => depth_measurements(),
        "2" => sub_position(),
        _ => (),
    }
}

fn read_lines(infile: &str) -> Vec<String> {
    let input = fs::read_to_string(infile)
        .expect("Something went wrong reading the input file");
    input.lines().map(String::from).collect()
}

// Day one
fn depth_measurements() {
    let measurements = read_lines("../inputs/01_1.txt").iter()
        .map(|x| x.parse::<u32>().unwrap()).collect();

    fn count_increases(measurements: &Vec<u32>) -> u32 {
        let mut last_measurement: Option<u32> = None;
        let mut num_increases: u32 = 0;

        for measurement in measurements {
            if let Some(last) = last_measurement {
                if *measurement > last {
                    num_increases += 1;
                }
            }
            last_measurement = Some(*measurement);
        }
        num_increases
    }

    println!("There were {} increases", count_increases(&measurements));

    const WINDOW_SIZE: usize = 3;
    let mut windowed_measurements: Vec<u32> = Vec::new();

    for i in 0..=measurements.len()-WINDOW_SIZE {

        let window = &measurements[i..i+WINDOW_SIZE];
        let window_sum = window.iter().sum::<u32>();

        windowed_measurements.push(window_sum);
    }

    println!("There were {} windowed increases",
             count_increases(&windowed_measurements));
}

// Day two
fn sub_position() {
    enum SubMovement {
        Forward(u32),
        Down(u32),
        Up(u32),
    }

    let movements: Vec<SubMovement> = read_lines("../inputs/02.txt")
        .iter()
        .map(|item| {
            let line: Vec<&str> = item.split_whitespace().collect();
            let mvt = line[0];
            let magnitude = line[1].parse::<u32>().unwrap();
            match mvt as &str {
                "forward" => SubMovement::Forward(magnitude),
                "up" => SubMovement::Up(magnitude),
                "down" => SubMovement::Down(magnitude),
                _ => unreachable!(),
            }
        }).collect();

    struct SubPosition {
        depth: u32,
        distance: u32,
        aim: u32,
    }

    let mut naive_position = SubPosition {
        depth: 0,
        distance: 0,
        aim: 0,
    };

    let mut position = SubPosition {
        depth: 0,
        distance: 0,
        aim: 0,
    };

    for movement in movements {
        match movement {
            SubMovement::Forward(mag) => {
                naive_position.distance += mag;
                position.distance += mag;
                position.depth += position.aim * mag;
            },
            SubMovement::Up(mag) => {
                naive_position.depth -= mag;
                position.aim -= mag;
            },
            SubMovement::Down(mag) => {
                naive_position.depth += mag;
                position.aim += mag;
            }
        }
    }
    println!("The naive position vector product is {}",
             naive_position.depth * naive_position.distance);
    println!("The position vector product is {}",
             position.depth * position.distance);
}
