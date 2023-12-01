use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

fn solve(input: &str) -> u32 {
    lazy_static! {
        static ref RE_FRONT: Regex =
            Regex::new(r#"[0-9]|one|two|three|four|five|six|seven|eight|nine"#).unwrap();
    }
    lazy_static! {
        static ref RE_BACK: Regex =
            Regex::new(r#"[0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin"#).unwrap();
    }

    let str_map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("eno", 1),
        ("owt", 2),
        ("eerht", 3),
        ("ruof", 4),
        ("evif", 5),
        ("xis", 6),
        ("neves", 7),
        ("thgie", 8),
        ("enin", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    return input
        .lines()
        .map(|line| {
            let reversed_line: String = line.chars().rev().collect();
            let first_digit = str_map[RE_FRONT.find(line).unwrap().as_str()];
            let last_digit = str_map[RE_BACK.find(&reversed_line).unwrap().as_str()];

            first_digit * 10 + last_digit
        })
        .sum();
}

fn main() {
    let input = include_str!("../inputs/part2.txt");
    dbg!(solve(input));
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test() {
        let test_input = include_str!("../inputs/part2-test.txt");
        assert_eq!(solve(test_input), 281);
    }
}
