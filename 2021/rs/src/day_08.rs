// Day 8

use crate::Answer;
use std::str::FromStr;

pub fn seven_seg_decode(input: Vec<String>) -> Answer {
    let mut displays: Vec<Display> = input.iter()
        .map(|s| Display::from_str(s).unwrap()).collect();

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

use std::collections::BTreeMap;

#[derive(Debug)]
struct Display {
    patterns: [Pattern; 10],
    values: [Pattern; 4],
    index: BTreeMap<u8, u8>,
}

impl Display {
    fn new(patterns: [Pattern; 10], values: [Pattern; 4]) -> Self {
        Display {
            patterns,
            values,
            index: BTreeMap::new(),
        }
    }

    fn insert(&mut self, pattern: u8, value: u8) {
        self.index.insert(pattern, value);
    }

    fn get_value(&self, pattern: u8) -> u8 {
        *self.index.get(&pattern).unwrap()
    }

    fn decode_patterns(&mut self) {
        let one = self.patterns.into_iter()
            .find(|p| p.cardinality() == 2).unwrap();
        self.insert(one.bits(), 1);

        let seven = self.patterns.into_iter()
            .find(|p| p.cardinality() == 3).unwrap();
        self.insert(seven.bits(), 7);

        let four = self.patterns.into_iter()
            .find(|p| p.cardinality() == 4).unwrap();
        self.insert(four.bits(), 4);

        let eight = self.patterns.into_iter()
            .find(|p| p.cardinality() == 7).unwrap();
        self.insert(eight.bits(), 8);

        let three = self.patterns.into_iter()
            .find(|p| {
                p.cardinality() == 5
                    && p.bits() == p.bits() | one.bits()
            }).unwrap();
        self.insert(three.bits(), 3);

        let nine = self.patterns.into_iter()
            .find(|p| {
                p.cardinality() == 6
                    && p.bits() == p.bits() | four.bits()
            }).unwrap();
        self.insert(nine.bits(), 9);

        let zero = self.patterns.into_iter()
            .find(|p| {
                p.cardinality() == 6
                    && p.bits() != nine.bits()
                    && p.bits() == p.bits() | one.bits()
            }).unwrap();
        self.insert(zero.bits(), 0);

        let six = self.patterns.into_iter()
            .find(|p| {
                p.cardinality() == 6
                    && p.bits() != nine.bits()
                    && p.bits() != zero.bits()
            }).unwrap();
        self.insert(six.bits(), 6);

        let two = self.patterns.into_iter()
            .find(|p| {
                p.cardinality() == 5
                    && Pattern::new(p.bits() | nine.bits()).cardinality() == 7
            }).unwrap();
        self.insert(two.bits(), 2);

        let five = self.patterns.into_iter()
            .find(|p| {
                p.cardinality() == 5 
                    && p.bits() != two.bits()
                    && p.bits() != three.bits()
            }).unwrap();
        self.insert(five.bits(), 5);
    }

    fn print_value(&self) -> u32 {
        self.values.into_iter().enumerate()
            .fold(0, |acc, (i, p)| {
                let value = self.get_value(p.bits()) as u32;
                let i: u32 = i.try_into().unwrap();
                acc + 10u32.pow(3 - i) * value
            })
    }
}

#[derive(Debug, Clone, Copy)]
struct Pattern([bool; 7]);

impl Pattern {
    fn new(bits: u8) -> Self {
        let mut pat = Pattern([false; 7]);
        pat.0.iter_mut().enumerate().for_each(|(i, d)| {
            if (bits >> i) % 2 == 1 {
                *d = true;
            }
        });
        pat
    }

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

        Ok(Display::new(
                patterns[..10].try_into().unwrap(),
                patterns[10..].try_into().unwrap()
        ))
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
