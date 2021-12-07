// Day three

pub fn binary_diagnostic(mut diagnostics: Vec<String>) -> (u32, u32) {
    const DEBUG: bool = true;

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

    if DEBUG {
        println!("Diag digits are {:?} and meas count is {}", diag_digits, meas_count);
    }

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
    if DEBUG { println!("Bits are {:?}", bits); }

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
    if DEBUG {
        println!("Submarine metrics are {} gamma, {} epsilon", rate.gamma, rate.epsilon);
    }

    let pow_consump = rate.power_consumption();
    if DEBUG { println!("submarine power consumption is {}", pow_consump); }

    // We still own diagnostics and we've retained our first result already.
    diagnostics.sort_unstable();
    if DEBUG { println!("Sorted diagnostics {:?}", diagnostics); }

    // What we're doing next is...
    // Recursively(?) searching through progressively smaller slices of the vec
    // to find out which single item satisfies the rule.
    // We have to do this twice, with two separate rules,
    // which means this could probably be done in parallel with immutable borrows.
    // This also means there's probably a higher-order function solution,
    // and I'm not sure if recursive programming is a thing in Rust.
    // I can't use exactly the same logic I used in part one, because
    // some of the filtered slices could have only one of the possible values.
    // So that means we actually need to aggregate/count them,
    // and then take the value with either the larger or smaller count
    // per the criteria (also verifying that the counts were not equal).
    // Since the value doesn't matter in this algorithm,
    // I'm not going to bother parsing the char into a usize/u32;
    // instead we'll just treat everything as a... byte?
    // Actually yeah, lets see if we can do Vec<&[u8]> and then
    // parse the result from UTF-8 once we're done.
    let diagnostics: Vec<Vec<u8>> = diagnostics.iter()
        .map(|item| item.clone().into_bytes())
        .collect();

    if DEBUG { println!("Diagnostic byte vector {:?}", diagnostics); }

    let search_vector = transpose(&diagnostics);

    if DEBUG { println!("Search vector {:?}", search_vector); }

    let oxygen_rating_idx = search(&search_vector, Rule {
        criteria: Frequency::Most,
        tie: b'1',
    });
    let co2_rating_idx = search(&search_vector, Rule {
        criteria: Frequency::Least,
        tie: b'0',
    });
    let oxygen_rating = utf8bin_to_u32(&diagnostics[oxygen_rating_idx]);
    let co2_rating = utf8bin_to_u32(&diagnostics[co2_rating_idx]);

    (pow_consump, oxygen_rating * co2_rating)
}

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
    where T: Copy,
{
    (0..v[0].len())
        .map(|i| v.iter().map(|item| item[i]).collect::<Vec<T>>())
        .collect::<Vec<Vec<T>>>()
}

// Filter to more or less, and tie
// Some function takes in a set of indices and returns another set of indices
enum Frequency {
    Most,
    Least,
}
struct Rule {
    criteria: Frequency,
    tie: u8,
}

fn apply_rule(slice: &[u8], rule: &Rule, bounds: (usize, usize)) -> (usize, usize) {
    (0,0)
}

fn search(places: &Vec<Vec<u8>>, rule: Rule) -> usize {
    let (mut start, mut end) = (0, places[0].len());
    for (i, place) in places.iter().enumerate() {
        let new_bounds = apply_rule(&place[..], &rule, (start, end));
        start = new_bounds.0;
        end = new_bounds.1;
        if start == end {
            return start;
        }
    }
    start
}

fn utf8bin_to_u32(input: &Vec<u8>) -> u32 {
    u32::from_str_radix(std::str::from_utf8(input).unwrap(), 2).unwrap()
}

#[cfg(test)]
mod function_tests {
    use super::*;

    #[test]
    fn utf8_conversion() {
        let input: Vec<u8> = vec![b'1', b'0', b'1', b'0', b'1', b'0'];
        assert_eq!(42, utf8bin_to_u32(&input));
    }

    #[test]
    fn transposition() {
        let input: Vec<Vec<u8>> = vec![
            vec![1,2],
            vec![3,4],
            vec![5,6],
        ];
        let expected: Vec<Vec<u8>> = vec![
            vec![1,3,5],
            vec![2,4,6],
        ];
        assert_eq!(transpose(&input), expected);
    }
}

#[cfg(test)]
mod answer_tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn power_consumption() {
        let input = read_input("../testinputs/03.txt");
        let (consumption, _) = binary_diagnostic(input);
        assert_eq!(198, consumption);
    }

    #[test]
    //#[ignore]
    fn life_support_rating() {
        let input = read_input("../testinputs/03.txt");
        let (_, rating) = binary_diagnostic(input);
        assert_eq!(230, rating);
    }
}
