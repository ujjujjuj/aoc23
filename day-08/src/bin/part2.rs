use std::{collections::HashMap, ops::ControlFlow};
use num::integer::lcm;

fn solve(input: &str) -> u64 {
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

    let start_nodes = map
        .keys()
        .filter(|&&node| node.chars().last().unwrap() == 'A')
        .map(|node| (*node))
        .collect::<Vec<&str>>();
    let mut cycle_lens = vec![];
    for mut curr in start_nodes {
        let mut moves = 0;
        dirs.chars().cycle().try_for_each(|dir| {
            match dir {
                'L' => curr = map[curr].0,
                'R' => curr = map[curr].1,
                _ => panic!("what"),
            }
            moves += 1;
            if curr.chars().last().unwrap() == 'Z' {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
        cycle_lens.push(moves as u64);
    }

    cycle_lens.into_iter().fold(1, |acc, cycle| lcm(acc,cycle))
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
        assert_eq!(solve(test_input), 6);
    }
}
