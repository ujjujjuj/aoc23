fn solve(input: &str) -> u32 {
    let width = input.find("\n").unwrap();
    let grid: Vec<char> = input.chars().filter(|c| *c != '\n').collect();
    let height = grid.len() / width;
    let mut number_grid = vec![false; width * height];

    let mut ans = 0;

    for row in 0..height {
        for col in 0..width {
            let item = grid[row * width + col];
            if item == '.' || item.is_ascii_digit() {
                continue;
            }
            for i in row - 1..row + 2 {
                for j in col - 1..col + 2 {
                    if i == j && j == 1 {
                        continue;
                    }
                    if grid.get((i * width) + j).unwrap().is_ascii_digit()
                        && !number_grid[i * width + j]
                    {
                        let mut tmp_j = j;
                        let mut start_j = j;
                        while grid[i * width + tmp_j].is_ascii_digit() {
                            start_j = tmp_j;
                            if tmp_j == 0 {
                                break;
                            }
                            tmp_j -= 1;
                        }

                        let mut num = 0;
                        tmp_j = start_j;
                        while tmp_j < width {
                            if !grid[i * width + tmp_j].is_ascii_digit() {
                                break;
                            }
                            number_grid[i * width + tmp_j] = true;
                            num *= 10;
                            num += grid[i * width + tmp_j].to_digit(10).unwrap();
                            tmp_j += 1;
                        }
                        ans += num;
                    }
                }
            }
        }
    }

    ans
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
        assert_eq!(solve(test_input), 4361);
    }
}
