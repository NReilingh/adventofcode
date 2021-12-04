#![allow(unused)]
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "1" => depth_measurements(),
        "2" => sub_position(),
        "3" => binary_diagnostic(),
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

// Day three
fn binary_diagnostic() {
    let diagnostics: Vec<String> = read_lines("../inputs/03.txt");

    let meas_len = diagnostics[0].chars().count();
    let meas_count = diagnostics.len();

    // The plan:
    // Measure the input to get the number of inputs and the number of digits
    // Allocate an int for each position and count the number of trues in each position
    // Subtract from the total number of inputs to get the falses;
    // basically just determine if it is less than or more than half the inputs
    // Then reduce this into two strings of binary and then parse them as such

    let diag_digits = diagnostics.iter().fold(vec![0; meas_len], |mut acc, item| {
        for (i, char) in item.chars().enumerate() {
            acc[i] += char.to_digit(10).unwrap();
        }
        acc
    });
    println!("Diag digits are {:?} and meas count is {}", diag_digits, meas_count);

    let bits = diag_digits.iter().fold((String::from(""), String::from("")), |mut acc, item| {
        if item * 2 > meas_count.try_into().unwrap() {
            acc.0.push_str("1");
            acc.1.push_str("0");
        } else {
            acc.0.push_str("0");
            acc.1.push_str("1");
        }
        acc
    });
    println!("Bits are {:?}", bits);

    struct Metrics {
        gamma: u32,
        epsilon: u32,
    }
    impl Metrics {
        fn power_consumption(&self) -> u32 {
            self.gamma * self.epsilon
        }
    }
    let rate = Metrics {
        gamma: u32::from_str_radix(&bits.0, 2).unwrap(),
        epsilon: u32::from_str_radix(&bits.1, 2).unwrap(),
    };
    println!("Submarine metrics are {} gamma, {} epsilon", rate.gamma, rate.epsilon);

    println!("submarine power consumption is {}", rate.power_consumption());
}
