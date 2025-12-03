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
            let (a, b) = best_battery(l.as_bytes());
            p1 += a;
            p2 += b;
        }

        Ok(Self { p1, p2 })
    }
}

fn best_battery(choices: &[u8]) -> (usize, usize) {
    let mut fac = 1;

    let mut cache = vec![vec![0; choices.len() + 1]; 13];

    for i in 1..13 {
        let mut max = 0;
        for (pos, d) in choices.iter().enumerate().take(choices.len() - i + 1).rev() {
            let j = (d - b'0') as usize;
            max = max.max(j * fac + cache[i - 1][pos + 1]);
            cache[i][pos] = max;
        }

        fac *= 10;
    }

    (cache[2][0], cache[12][0])
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
