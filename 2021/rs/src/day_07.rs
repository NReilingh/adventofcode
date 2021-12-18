// Day 7

use crate::Answer;
use std::cmp::min;

pub fn crab_target_alignment(input: Vec<String>) -> Answer {
    let mut crab_positions: Vec<i64> = input.iter().take(1)
        .flat_map(|x| x.split(','))
        .map(|x| x.parse().unwrap()).collect();

    crab_positions.sort_unstable();

    let optimal_target = crab_positions[crab_positions.len() / 2];
    let optimal_cost = cost_to_target(optimal_target, &crab_positions);

    let num_crabs = crab_positions.len() as f64;
    let optimal_inc_target = crab_positions.iter().map(|p| *p as f64).sum::<f64>() / num_crabs;

    let (opt_floor, opt_ceil) = (optimal_inc_target.floor() as i64, optimal_inc_target.ceil() as i64);

    let opt_inc_cost = [opt_floor, opt_ceil].iter()
        .map(|x| increasing_cost_to_target(*x, &crab_positions))
        .min().unwrap();

    Answer::I64(optimal_cost, opt_inc_cost)
}

fn increasing_cost_to_target(target: i64, set: &[i64]) -> i64 {
    set.iter().map(|x| (target - *x).abs()).map(|n| (n * (n + 1)) / 2).sum()
}

fn cost_to_target(target: i64, set: &[i64]) -> i64 {
    set.iter().map(|x| (target - *x).abs()).sum()
}

#[cfg(test)]
mod cost_to_target_tests {
    use super::cost_to_target;

    #[test]
    fn interior_target() {
        let set = vec![3, 4, 4, 5, 7, 8, 9];
        assert_eq!(14, cost_to_target(6, &set));
    }

    #[test]
    fn below_target() {
        let set = vec![3, 4, 4, 5, 7, 8, 9];
        assert_eq!(33, cost_to_target(1, &set));
    }

    #[test]
    fn above_target() {
        let set = vec![3, 4, 4, 5, 7, 8, 9];
        assert_eq!(72, cost_to_target(16, &set));
    }
}

#[cfg(test)]
mod answer_tests {
    use super::*;

    use crate::read_input;

    #[test]
    fn minimum_fuel() {
        let input = read_input("../testinputs/07.txt");
        if let Answer::I64(fuel, _) = crab_target_alignment(input) {
            assert_eq!(37, fuel);
        } else { panic!("Answer was not a I64 variant.") }
    }

    #[test]
    fn minimum_increasing_fuel() {
        let input = read_input("../testinputs/07.txt");
        if let Answer::I64(_, fuel) = crab_target_alignment(input) {
            assert_eq!(168, fuel);
        } else { panic!("Answer was not a I64 variant.") }
    }
}
