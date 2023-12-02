use crate::Part;

pub fn run(input: &str, part: Part) -> usize {
    match part {
        Part::One => part_1(input),
        Part::Two => part_2(input),
    }
}

fn part_1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let mut first_digit: Option<char> = None;
        let mut last_digit = ' ';
        for char in line.chars() {
            if char.is_ascii_digit() {
                let nr = char;
                if first_digit.is_none() {
                    first_digit = Some(nr);
                }
                last_digit = nr;
            }
        }
        sum += format!("{}{}", first_digit.unwrap(), last_digit)
            .parse::<usize>()
            .unwrap();
    }
    sum
}

fn part_2(input: &str) -> usize {
    let string_nrs = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum = 0;
    for line in input.lines() {
        let chars = line.chars();

        let mut first_digit_index: Option<usize> = None;
        let mut last_digit_index: Option<usize> = None;

        for i in 0..line.len() {
            if chars.clone().nth(i).unwrap().is_ascii_digit() {
                if first_digit_index.is_none() {
                    first_digit_index = Some(i);
                }
                last_digit_index = Some(i);
            }
        }

        let mut first_string_nr: Option<usize> = None;
        let mut first_string_index = 999;
        let mut last_string_nr: Option<usize> = None;
        let mut last_string_index = 0;
        for j in 0..10 {
            if let Some(index) = line.find(string_nrs[j]) {
                if (first_digit_index.is_none() || first_digit_index.unwrap() > index)
                    && first_string_index > index
                {
                    first_string_nr = Some(j);
                    first_string_index = index;
                }
            }
            if let Some(index) = line.rfind(string_nrs[j]) {
                if (last_digit_index.is_none() || last_digit_index.unwrap() < index)
                    && last_string_index < index
                {
                    last_string_nr = Some(j);
                    last_string_index = index;
                }
            }
        }

        let concatenated_nr = format!(
            "{}{}",
            first_string_nr.unwrap_or_else(|| chars
                .clone()
                .nth(first_digit_index.unwrap())
                .unwrap()
                .to_digit(10)
                .unwrap() as usize),
            last_string_nr.unwrap_or_else(|| chars
                .clone()
                .nth(last_digit_index.unwrap())
                .unwrap()
                .to_digit(10)
                .unwrap() as usize),
        )
        .parse::<usize>()
        .unwrap();

        println!("{concatenated_nr} {line}");
        sum += concatenated_nr
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, part_1(input));
    }

    #[test]
    fn example_part_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281, part_2(input));
    }
}
