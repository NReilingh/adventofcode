// Day one

pub fn depth_measurements(input_lines: Vec<String>) -> (u32, u32) {
    const DEBUG: bool = false;

    let measurements = input_lines.iter()
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

    (increases, windowed_increases)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn increase_count() {
        let input = read_input("../testinputs/01.txt");
        let (increases, _) = depth_measurements(input);
        assert_eq!(7, increases);
    }

    #[test]
    fn windowed_increase_count() {
        let input = read_input("../testinputs/01.txt");
        let (_, increases) = depth_measurements(input);
        assert_eq!(5, increases);
    }
}
