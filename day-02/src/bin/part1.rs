use lazy_static::lazy_static;
use regex::Regex;

fn solve(input: &str) -> u32 {
    lazy_static! {
        static ref COLOR_NUM_REG: Regex = Regex::new(r#"(\d+)\s(\w+)"#).unwrap();
    };
    input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            for capture in COLOR_NUM_REG.captures_iter(line) {
                let color_amt = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let mut upper_bound = 0;
                match capture.get(2).unwrap().as_str() {
                    "red" => upper_bound = 12,
                    "green" => upper_bound = 13,
                    "blue" => upper_bound = 14,
                    _ => (),
                }
                if color_amt > upper_bound {
                    return 0;
                }
            }
            idx + 1
        })
        .sum::<usize>() as u32
}

fn main() {
    let input = include_str!("../inputs/part1.txt");
    dbg!(solve(input));
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test() {
        let test_input = include_str!("../inputs/part1-test.txt");
        assert_eq!(solve(test_input), 8);
    }
}
