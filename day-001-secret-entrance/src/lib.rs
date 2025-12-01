use std::str::FromStr;

use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
pub struct SecretEntrance {
    zeros: i64,
    pass_zeros: i64,
}

impl FromStr for SecretEntrance {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut zeros = 0;
        let mut pass_zeros = 0;
        let mut sum = 50_i64;
        for line in s.trim().lines() {
            let (s, r) = line.split_at(1);

            let v: i64 = if s == "R" { r.parse()? } else { -r.parse()? };

            pass_zeros += if v < 0 {
                (100 - sum - v) / 100 - if sum == 0 { 1 } else { 0 }
            } else {
                (sum + v) / 100
            };

            sum += v;
            sum = sum.rem_euclid(100);

            if sum == 0 {
                zeros += 1;
            }
        }
        Ok(Self { zeros, pass_zeros })
    }
}

impl Problem for SecretEntrance {
    const DAY: usize = 1;
    const TITLE: &'static str = "secret entrance";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.zeros)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.pass_zeros)
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
        let solution = SecretEntrance::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1084, 6475));
    }

    #[test]
    fn example() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let solution = SecretEntrance::solve(input).unwrap();
        assert_eq!(solution, Solution::new(3, 6));
    }
}
