use std::str::FromStr;

use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
pub struct Cafeteria;

impl FromStr for Cafeteria {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self)
    }
}

impl Problem for Cafeteria {
    const DAY: usize = 5;
    const TITLE: &'static str = "cafeteria";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(0)
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
        let solution = Cafeteria::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(0, 0));
    }

    #[test]
    fn example() {
        let input = "";
        let solution = Cafeteria::solve(input).unwrap();
        assert_eq!(solution, Solution::new(0, 0));
    }
}
