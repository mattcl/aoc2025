use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
pub struct ChristmasTreeFarm {
    p1: usize,
}

impl FromStr for ChristmasTreeFarm {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // so this was kind-of unsatisfying , but the problem, as stated, would
        // be NP hard in the general case
        let mut p1 = 0;
        // let regions = s.trim().split("\n\n").last().ok_or_else(|| anyhow!("invalid input"))?;
        // for r in regions.lines() {
        for r in s.trim().lines().skip(30) {
            let (left, right) = r.split_once(": ").ok_or_else(|| anyhow!("invalid input"))?;
            let mut required = 0_u64;
            for n in right.split(' ') {
                required += n.parse::<u64>()?;
            }
            let (w, h) = left
                .split_once('x')
                .ok_or_else(|| anyhow!("invalid input"))?;
            let area = (w.parse::<u64>()? * h.parse::<u64>()?) / 9;

            if area >= required {
                p1 += 1;
            }
        }
        Ok(Self { p1 })
    }
}

impl Problem for ChristmasTreeFarm {
    const DAY: usize = 12;
    const TITLE: &'static str = "christmas tree farm";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.p1)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(0)
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
        let solution = ChristmasTreeFarm::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(485, 0));
    }
}
