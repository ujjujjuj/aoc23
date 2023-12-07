fn solve(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();

    let time = lines[0]
        .chars()
        .filter(|c| c.is_ascii_digit())
        .fold(0, |acc, e| acc * 10 + (e as u64 - '0' as u64)) as f64;
    let dist = lines[1]
        .chars()
        .filter(|c| c.is_ascii_digit())
        .fold(0, |acc, e| acc * 10 + (e as u64 - '0' as u64)) as f64;

    let d = (time * time - 4f64 * dist).sqrt();
    let x1 = ((time - d) / 2f64 + 0.0001f64).ceil() as u64;
    let x2 = ((time + d) / 2f64 - 0.0001f64).floor() as u64;
    x2 - x1 + 1
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
        assert_eq!(solve(test_input), 71503);
    }
}
