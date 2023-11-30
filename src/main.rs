mod day_1;

use std::env;
use std::fs::read_to_string;

pub enum Part {
    One,
    Two,
}

fn main() {
    let day_number: String = env::args().nth(1).expect("Please enter day number");

    let part: Part = match env::args()
        .nth(2)
        .expect("Please enter what part you want to run")
        .as_str()
    {
        "1" => Part::One,
        "2" => Part::Two,
        _ => panic!("Couldn't parse part number"),
    };

    println!("Running day {}", day_number);

    let input =
        read_to_string(format!("inputs/day_{}.txt", day_number)).expect("Couldn't read input file");

    let answer = match day_number.as_str() {
        "1" => day_1::run(&input, part),
        _ => todo!(),
    };

    println!("Answer: {}", answer);
}
