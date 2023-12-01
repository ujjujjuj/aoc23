fn solve(input: &str) -> u32 {
    return input
        .lines()
        .map(|line| {
            let first_dig = line
                .chars()
                .find(|c| c.is_numeric())
                .unwrap()
                .to_digit(10)
                .unwrap();
            let last_dig = line
                .chars()
                .rev()
                .find(|c| c.is_numeric())
                .unwrap()
                .to_digit(10)
                .unwrap();

            first_dig * 10 + last_dig
        })
        .sum();
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
        assert_eq!(solve(test_input), 142);
    }
}
