use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

fn solve(input: &str) -> u32 {
    lazy_static! {
        static ref NUM_REGEX: Regex = Regex::new(r#"\d+"#).unwrap();
    }
    let mut card_count: HashMap<u32, u32> = HashMap::new();
    input
        .lines()
        .map(|line| {
            let game_num = NUM_REGEX
                .find(line)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
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
            let curr_cnt = *card_count.entry(game_num).or_insert(1);
            for incr_card_num in game_num + 1..game_num + ans + 1 {
                let cnt = card_count.entry(incr_card_num).or_insert(1);
                *cnt += curr_cnt;
            }
            curr_cnt
        })
        .sum()
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
        assert_eq!(solve(test_input), 30);
    }
}
