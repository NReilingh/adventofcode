// Day three

pub fn binary_diagnostic(diagnostics: Vec<String>) -> (u32, u32) {
    const DEBUG: bool = false;

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

    (pow_consump, 0)
}
