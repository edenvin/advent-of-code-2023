use crate::Part;
use regex::Regex;

pub fn run(input: &str, part: Part) -> usize {
    match part {
        Part::One => part_1(input),
        Part::Two => part_2(input),
    }
}

struct Limits {
    red: usize,
    blue: usize,
    green: usize,
}

fn part_1(input: &str) -> usize {
    let limits = Limits {
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut sum = 0;

    'line: for line in input.lines() {
        let red_regex = Regex::new(r"(\d*) red").unwrap();
        let green_regex = Regex::new(r"(\d*) green").unwrap();
        let blue_regex = Regex::new(r"(\d*) blue").unwrap();

        let red_captures = red_regex.captures_iter(line);
        let green_captures = green_regex.captures_iter(line);
        let blue_captures = blue_regex.captures_iter(line);

        for capture in red_captures {
            let nr = capture.get(1).unwrap().as_str();
            if nr.parse::<usize>().unwrap() > limits.red {
                continue 'line;
            }
        }

        for capture in green_captures {
            if capture.get(1).unwrap().as_str().parse::<usize>().unwrap() > limits.green {
                continue 'line;
            }
        }

        for capture in blue_captures {
            if capture.get(1).unwrap().as_str().parse::<usize>().unwrap() > limits.blue {
                continue 'line;
            }
        }

        let regex = Regex::new(r".* (\d*): .*").unwrap();
        let id = regex.captures(line).unwrap().get(1).unwrap().as_str();

        sum += id.parse::<usize>().unwrap();
    }
    sum
}

fn part_2(input: &str) -> usize {
    let mut sum = 0;

    for line in input.lines() {
        let red_regex = Regex::new(r"(\d*) red").unwrap();
        let green_regex = Regex::new(r"(\d*) green").unwrap();
        let blue_regex = Regex::new(r"(\d*) blue").unwrap();

        let red_captures = red_regex.captures_iter(line);
        let green_captures = green_regex.captures_iter(line);
        let blue_captures = blue_regex.captures_iter(line);

        let biggest_red_count = red_captures
            .map(|capture| capture.get(1).unwrap().as_str().parse::<usize>().unwrap())
            .max()
            .unwrap_or(0);
        let biggest_green_count = green_captures
            .map(|capture| capture.get(1).unwrap().as_str().parse::<usize>().unwrap())
            .max()
            .unwrap_or(0);
        let biggest_blue_count = blue_captures
            .map(|capture| capture.get(1).unwrap().as_str().parse::<usize>().unwrap())
            .max()
            .unwrap_or(0);
        sum += biggest_red_count * biggest_green_count * biggest_blue_count;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, part_1(input));
    }

    #[test]
    fn example_part_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(2286, part_2(input));
    }
}
