// Day 8

use crate::Answer;
use std::str::FromStr;

pub fn seven_seg_decode(input: Vec<String>) -> Answer {
    let mut displays: Vec<Display> = input.iter()
        .map(|s| Display::from_str(s).unwrap()).collect();

    // println!("displays are {:#?}", displays);
    let simple_outputs = displays.iter()
        .map(|d| {
            d.values.iter()
                .filter(|v| {
                    match v.cardinality() {
                        2 | 3 | 4 | 7 => true,
                        _ => false
                    }
                })
                .count()
        }).sum::<usize>().try_into().unwrap();

    let full_monty = displays.iter_mut()
        .map(|d| {
            d.decode_patterns();
            d.print_value()
        })
        .sum();

    Answer::U32(simple_outputs, full_monty)
}

#[derive(Debug)]
struct Display {
    patterns: [Pattern; 10],
    values: [Pattern; 4],
    codes: Vec<u8>,
}

impl Display {
    fn decode_patterns(&mut self) {
    }

    fn print_value(&self) -> u32 {
        0
    }
}

#[derive(Debug, Clone, Copy)]
struct Pattern([bool; 7]);

impl Pattern {
    fn cardinality(&self) -> u8 {
        self.0.iter().filter(|s| **s).count().try_into().unwrap()
    }

    fn bits(&self) -> u8 {
        self.0.iter().enumerate().fold(0, |acc, (i, val)| {
            if *val {
                acc + (1 << i)
            } else {
                acc
            }
        })
    }
}

#[cfg(test)]
mod pattern_tests {
    use super::*;

    #[test]
    fn bit_output() {
        let pat = Pattern([true, false, true, true, false, false, false]);
        assert_eq!(0b1101, pat.bits());
    }

    #[test]
    fn full_output() {
        let pat = Pattern([true; 7]);
        assert_eq!(0b1111111, pat.bits());
    }
}

impl FromStr for Display {
    type Err = std::string::ParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let patterns = input
            .replace(" | ", " ").split_whitespace()
            .map(|pattern| {
                pattern.chars()
                    .map(|c| (c as u32) - 97 )
                    .fold(Pattern([false; 7]), |mut acc, i| {
                        acc.0[i as usize] = true;
                        acc
                    })
            })
            .collect::<Vec<Pattern>>();

        Ok(Display {
            patterns: patterns[..10].try_into().unwrap(),
            values: patterns[10..].try_into().unwrap(),
            codes: Vec::new(),
        })
    }
}

#[cfg(test)]
mod answer_tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn simple_digits() {
        let input = read_input("../testinputs/08.txt");
        if let Answer::U32(count, _) = seven_seg_decode(input) {
            assert_eq!(26, count);
        } else {
            panic!("wrong type");
        }
    }

    #[test]
    fn display_values() {
        let input = read_input("../testinputs/08.txt");
        if let Answer::U32(_, sum) = seven_seg_decode(input) {
            assert_eq!(61229, sum);
        } else {
            panic!("wrong type");
        }
    }
}
