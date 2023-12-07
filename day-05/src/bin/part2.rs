use itertools::Itertools;

fn solve(input: &str) -> u32 {
    let mut lines = input.lines();

    let seeds_line = lines.next().unwrap();
    let old_seeds: Vec<u32> = seeds_line[seeds_line.find(": ").unwrap() + 2..]
        .split(" ")
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    // TODO: find a better solution
    let mut seeds = vec![];
    old_seeds.chunks(2).for_each(|chunk| {
        for seed in chunk[0]..chunk[0] + chunk[1] {
            seeds.push(seed);
        }
    });

    lines.next();
    while let Some(_) = lines.next() {
        let mut src_arr: Vec<(u32, u32, u32)> = vec![];

        while let Some(map_line) = lines.next() {
            if map_line.len() == 0 {
                break;
            }
            let (dest, src, size) = map_line
                .split_whitespace()
                .map(|c| c.parse::<u32>().unwrap())
                .next_tuple()
                .unwrap();
            src_arr.push((dest, src, size));
        }

        src_arr.sort();

        seeds = seeds
            .into_iter()
            .map(|seed| {
                let mut lb = src_arr.len() - 1;
                while lb > 0 && seed < src_arr[lb].1 {
                    lb -= 1;
                }

                let (dest, src, size) = src_arr[lb];
                if src + size < seed || src > seed {
                    seed
                } else {
                    dest + (seed - src)
                }
            })
            .collect();
    }

    seeds.into_iter().min().unwrap()
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
        assert_eq!(solve(test_input), 46);
    }
}
