// Day n

use crate::Answer;

pub fn my_puzzle_name(input: Vec<String>) -> Answer {
    Answer::U32(0, 0)
}

#[cfg(test)]
mod answer_tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn first_puzzle() {
        let input = read_input("../testinputs/08.txt");
        if let Answer::U32(first, _) = my_puzzle_name(input) {
            assert_eq!(26, first);
        } else {
            panic!("wrong type");
        }
    }

    #[test]
    #[ignore]
    fn second_puzzle() {
        let input = read_input("../testinputs/08.txt");
        if let Answer::U32(_, second) = my_puzzle_name(input) {
            assert_eq!(61229, second);
        } else {
            panic!("wrong type");
        }
    }
}
