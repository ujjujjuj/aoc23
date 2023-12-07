use regex::Regex;

fn solve(input: &str) -> u32 {
    let num_regex = Regex::new(r#"\d+"#).unwrap();

    let lines: Vec<&str> = input.lines().collect();
    let times = num_regex
        .find_iter(lines[0])
        .map(|num_match| num_match.as_str().parse::<f32>().unwrap());
    let distances = num_regex
        .find_iter(lines[1])
        .map(|num_match| num_match.as_str().parse::<f32>().unwrap());

    times
        .zip(distances)
        .map(|(time, dist)| {
            let d = (time * time - 4f32 * dist).sqrt();
            let x1 = ((time - d) / 2f32 + 0.0001f32).ceil() as u32;
            let x2 = ((time + d) / 2f32 - 0.0001f32).floor() as u32;
            x2 - x1 + 1
        })
        .product()
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
        assert_eq!(solve(test_input), 288);
    }
}
