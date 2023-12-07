type Hand = [u32; 5];
type Bet = (Hand, u32);
type Score = [u32; 2]; // type, remaining

fn get_bet(line: &str) -> Bet {
    let cards: Hand = line[..5]
        .chars()
        .map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _ => c as u32 - '0' as u32,
        })
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap();
    let bet = line[6..].parse::<u32>().unwrap();

    (cards, bet)
}
fn get_score(mut hand: Hand) -> Score {
    let val = hand.iter().fold(0, |acc, e| acc * 15 + e);
    let j_count = hand.iter().filter(|&card| *card == 1).count() as u32;
    hand.sort_by(|a, b| b.cmp(a));
    if hand.iter().all(|&card| card == hand[0]) {
        return [7, val]; // five of a kind
    } else if let Some(_) = hand
        .windows(4)
        .find(|&win| win.iter().all(|&card| card == win[0]))
    {
        return [7.min(6 + j_count), val]; // four of a kind
    } else if let Some(equ_win) = hand
        .windows(3)
        .find(|&win| win.iter().all(|&card| card == win[0]))
    {
        let remaining = hand
            .iter()
            .filter(|&card| *card != equ_win[0])
            .collect::<Vec<&u32>>();
        if remaining[0] == remaining[1] {
            return [7.min(5 + j_count), val]; // full house
        } else {
            return [6.min(4 + 2 * j_count), val]; // three of a kind
        }
    } else if let Some(equ_win) = hand
        .windows(2)
        .find(|&win| win.iter().all(|&card| card == win[0]))
    {
        if let Some(_) = hand
            .iter()
            .filter(|&card| *card != equ_win[0])
            .collect::<Vec<&u32>>()
            .windows(2)
            .find(|&win| win.iter().all(|&card| card == win[0]))
        {
            return [6.min(3 + 2 * j_count), val]; // two pair
        } else {
            return [4.min(2 + 2 * j_count), val]; // pair
        }
    } else {
        return [2.min(1 + j_count), val]; // all different
    }
}

fn solve(input: &str) -> u32 {
    let mut bets: Vec<(Score, u32)> = input
        .lines()
        .map(|line| get_bet(line))
        .map(|bet| (get_score(bet.0), bet.1))
        .collect();
    bets.sort();

    bets.iter()
        .enumerate()
        .fold(0, |acc, (idx, e)| acc + (idx + 1) as u32 * e.1)
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
        assert_eq!(solve(test_input), 5905);
    }
}
