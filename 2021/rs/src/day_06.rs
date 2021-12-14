pub fn lanternfish(input: Vec<String>) -> (u64, u64) {
    let mut school: School = School::from_str(&input[0]).unwrap();
    school.time_travel(80);
    let first_count = school.census().try_into().unwrap();
    school.time_travel(256-80);
    let second_count = school.census().try_into().unwrap();
    (first_count, second_count)
}

#[derive(Debug, PartialEq, Eq)]
struct School(u64, u64, u64, u64, u64, u64, u64, u64, u64);

impl School {
    fn age(&mut self) {
        *self = School(
            self.1,
            self.2,
            self.3,
            self.4,
            self.5,
            self.6,
            self.7 + self.0,
            self.8,
            self.0
        );

    }

    fn time_travel(&mut self, ticks: u32) {
        for tick in 0..ticks {
            self.age();
        }
    }

    fn census(&self) -> u64 {
        self.0 +
        self.1 +
        self.2 +
        self.3 +
        self.4 +
        self.5 +
        self.6 +
        self.7 +
        self.8
    }
}

use std::str::FromStr;
use std::error::Error;
impl FromStr for School {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let results: Result<_, Self::Err> = s.split(',')
            .try_fold(Vec::new(), |mut acc, cur| {
                acc.push(cur.parse::<u8>()?);
                Ok(acc)
            });

        let results = results?;

        let pops = results.iter()
            .try_fold(School(0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64), |mut acc, cur| {
                match cur {
                    0 => acc.0 += 1,
                    1 => acc.1 += 1,
                    2 => acc.2 += 1,
                    3 => acc.3 += 1,
                    4 => acc.4 += 1,
                    5 => acc.5 += 1,
                    6 => acc.6 += 1,
                    7 => acc.7 += 1,
                    8 => acc.8 += 1,
                    _ => return Err("Got an impossible age from the submarine")
                }
                Ok(acc)
            });

        Ok(pops?)
    }
}

#[cfg(test)]
mod function_tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn school_from_string() {
        assert_eq!(School(0, 0, 3, 2, 0, 0, 0, 0, 1), School::from_str("8,2,3,2,3,2").unwrap());
    }

    #[test]
    fn calculate_total_population() {
        let school = School(0, 3, 6, 9, 1, 0, 0, 4, 5);
        assert_eq!(28, school.census());
    }
}

#[cfg(test)]
mod answer_tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn simulate_fish() {
        let input = read_input("../testinputs/06.txt");
        let (fish, _) = lanternfish(input);
        assert_eq!(5934, fish);
    }

    #[test]
    fn simulate_fish_harder() {
        let input = read_input("../testinputs/06.txt");
        let (_, more_fish) = lanternfish(input);
        assert_eq!(26984457539, more_fish);
    }
}
