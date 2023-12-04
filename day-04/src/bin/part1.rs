use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

fn solve(input: &str) -> u32 {
    lazy_static! {
        static ref NUM_REGEX: Regex = Regex::new(r#"\d+"#).unwrap();
    }
    input
        .lines()
        .map(|line| {
            let mut winning_nums = HashSet::new();
            let mut ans = 0;
            line[line.find(':').unwrap()..]
                .split('|')
                .enumerate()
                .for_each(|(idx, nums)| {
                    NUM_REGEX.find_iter(nums).for_each(|num| match idx {
                        0 => {
                            winning_nums.insert(num.as_str());
                        }
                        1 => {
                            if winning_nums.contains(num.as_str()) {
                                ans += 1;
                            }
                        }
                        _ => (),
                    });
                });
            if ans == 0 {
                0
            } else {
                1 << (ans - 1)
            }
        })
        .sum()
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
        assert_eq!(solve(test_input), 13);
    }
}
