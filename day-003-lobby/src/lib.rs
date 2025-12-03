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

        let mut cache1 = vec![vec![0; 100 + 1]; 13];
        let mut cache2 = vec![vec![0; 100 + 1]; 13];

        for l in s.trim().lines() {
            best_battery(l.as_bytes(), &mut cache1, &mut cache2);

            p1 += cache1[2][0];
            p2 += cache1[12][0];

            std::mem::swap(&mut cache1, &mut cache2);
        }

        Ok(Self { p1, p2 })
    }
}

fn best_battery(
    choices: &[u8],
    cache: &mut [Vec<usize>],
    cache2: &mut [Vec<usize>]
) {
    let mut fac = 1;

    for i in 1..13 {
        let mut max = 0;
        for (pos, d) in choices.iter().enumerate().take(choices.len() - i + 1).rev() {
            let j = (d - b'0') as usize;
            max = max.max(j * fac + cache[i - 1][pos + 1]);
            cache[i][pos] = max;
            cache2[i][pos] = 0;
        }

        fac *= 10;
    }
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
