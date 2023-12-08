use std::{collections::HashMap, ops::ControlFlow};

fn solve(input: &str) -> u32 {
    let mut lines = input.lines();
    let dirs = lines.next().unwrap();

    lines.next();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    while let Some(line) = lines.next() {
        let from_node = &line[..3];
        let left_node = &line[7..10];
        let right_node = &line[12..15];
        map.insert(from_node, (left_node, right_node));
    }

    let mut moves = 0;
    let mut curr = "AAA";

    dirs.chars().cycle().try_for_each(|dir| {
        match dir {
            'L' => curr = map[curr].0,
            'R' => curr = map[curr].1,
            _ => panic!("what"),
        }
        moves += 1;
        if curr == "ZZZ" {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    });

    moves
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
        assert_eq!(solve(test_input), 6);
    }
}
