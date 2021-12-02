// use std::env;
use std::fs;

fn main() {
    day_one_first("../testinputs/01_1.txt");
}

fn day_one_first(infile: &str) {
    let input = fs::read_to_string(infile)
        .expect("Something went wrong reading the file");

    println!("inputfile:\n{}", input);
    let measurements = input.lines();

  //  println!("first_line:\n{}", measurements[0]);
}
