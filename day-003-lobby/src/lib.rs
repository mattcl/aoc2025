use std::str::FromStr;

use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
pub struct Lobby {
    p1: usize,
    p2: usize,
}

impl FromStr for Lobby {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p1 = 0;
        let mut p2 = 0;

        for l in s.trim().lines() {
            let battery = l.as_bytes();
            let mut max_pos = 0;
            let mut max_v = 0;

            #[allow(clippy::needless_range_loop)]
            for i in 0..(battery.len() - 12) {
                let v = battery[i] - b'0';
                if v > max_v {
                    max_v = v;
                    max_pos = i;

                    if v == 9 {
                        break;
                    }
                }
            }

            let shrunk = &battery[max_pos..];

            p1 += best_battery2(shrunk);
            p2 += best_battery12(shrunk);
        }

        Ok(Self { p1, p2 })
    }
}

fn best_battery2(choices: &[u8]) -> usize {
    let mut first = choices[0] - b'0';
    let mut max_pos = 0;
    if first != 9 {
        #[allow(clippy::needless_range_loop)]
        for i in 0..(choices.len() - 1) {
            let v = choices[i] - b'0';
            if v > first {
                first = v;
                max_pos = i;

                if v == 9 {
                    break;
                }
            }
        }
    }

    let mut second = 0;
    for ch in choices[max_pos + 1..].iter() {
        let v = ch - b'0';
        if v > second {
            second = v;
            if v == 9 {
                break;
            }
        }
    }

    (first * 10 + second) as usize
}

fn best_battery12(choices: &[u8]) -> usize {
    let mut max = 0;
    let mut start = 0;
    for pos in 0..12 {
        let exclude = 11 - pos as usize;
        let (max_idx, max_digit) = choices[start..(choices.len() - exclude)]
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|(_, d)| *d)
            .expect("we know this has at least one element");
        start += max_idx + 1;
        max += (max_digit - b'0') as usize * 10_usize.pow(11 - pos);
    }
    max
}

impl Problem for Lobby {
    const DAY: usize = 3;
    const TITLE: &'static str = "lobby";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.p1)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.p2)
    }
}

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = Lobby::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(17179, 170025781683941));
    }

    #[test]
    fn example() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let solution = Lobby::solve(input).unwrap();
        assert_eq!(solution, Solution::new(357, 3121910778619));
    }
}
