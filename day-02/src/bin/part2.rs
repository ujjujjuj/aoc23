use lazy_static::lazy_static;
use regex::Regex;

fn solve(input: &str) -> u32 {
    lazy_static! {
        static ref COLOR_NUM_REG: Regex = Regex::new(r#"(\d+)\s(\w+)"#).unwrap();
    };
    input
        .lines()
        .map(|line| {
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;
            for capture in COLOR_NUM_REG.captures_iter(line) {
                let color_amt = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
                match capture.get(2).unwrap().as_str() {
                    "red" => min_red = min_red.max(color_amt),
                    "green" => min_green = min_green.max(color_amt),
                    "blue" => min_blue = min_blue.max(color_amt),
                    _ => {}
                }
            }
            min_red * min_green * min_blue
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
        assert_eq!(solve(test_input), 2286);
    }
}
