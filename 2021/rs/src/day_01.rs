// Day one

use crate::Answer;

pub fn depth_measurements(input_lines: Vec<String>) -> Answer {
    const DEBUG: bool = false;

    let measurements: Vec<u32> = input_lines.iter()
        .map(|x| x.parse::<u32>().unwrap()).collect();

    fn count_increases(measurements: &[u32]) -> u32 {
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

    let increases = count_increases(&measurements);
    if DEBUG { println!("There were {} increases", increases); }

    const WINDOW_SIZE: usize = 3;
    let mut windowed_measurements: Vec<u32> = Vec::new();

    for i in 0..=measurements.len()-WINDOW_SIZE {

        let window = &measurements[i..i+WINDOW_SIZE];
        let window_sum = window.iter().sum::<u32>();

        windowed_measurements.push(window_sum);
    }

    let windowed_increases = count_increases(&windowed_measurements);
    if DEBUG { println!("There were {} windowed increases", windowed_increases); }

    Answer::U32(increases, windowed_increases)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn increase_count() {
        let input = read_input("../testinputs/01.txt");
        if let Answer::U32(increases, _) = depth_measurements(input) {
            assert_eq!(7, increases);
        } else { panic!("Answer was not a U32 variant.") }
    }

    #[test]
    fn windowed_increase_count() {
        let input = read_input("../testinputs/01.txt");
        if let Answer::U32(_, increases) = depth_measurements(input) {
            assert_eq!(5, increases);
        } else { panic!("Answer was not a U32 variant.") }
    }
}
