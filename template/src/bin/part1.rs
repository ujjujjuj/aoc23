fn solve(input: &str) -> u32 {
    todo!();
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
