use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;
use aoc_std::collections::bitset::BitSet192;

#[derive(Debug, Clone)]
pub struct Laboratories {
    p1: usize,
    p2: usize,
}

impl FromStr for Laboratories {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.trim().lines().count() / 2;
        let mut grid = vec![BitSet192::zero(); height];

        let mut iter = s.trim().lines();
        let first_row = iter.next().ok_or_else(|| anyhow!("invalid input"))?;
        let width = first_row.len();

        let mut front = BitSet192::zero();
        let mut timelines = vec![0_usize; width];
        let s_idx = first_row
            .as_bytes()
            .iter()
            .enumerate()
            .find(|(_, ch)| **ch == b'S')
            .ok_or_else(|| anyhow!("invalid input"))?
            .0;
        front.insert(s_idx);
        timelines[s_idx] += 1;

        // skip the empty row
        iter.next();

        let mut row = 0;
        while let Some(line) = iter.next() {
            for (idx, b) in line.as_bytes().iter().enumerate() {
                if *b == b'^' {
                    grid[row].insert(idx);
                }
            }

            // skip the empty ones
            iter.next();
            row += 1;
        }

        let mut p1 = 0;

        for row in grid.iter() {
            // we can find all the split locations at once
            let splits = row & front;
            p1 += splits.count() as usize;

            // xor to clear the bits we split on
            front ^= splits;

            let mut j = 0;
            while let Some(jn) = splits.next_beyond(j) {
                j = jn;
                if j > width {
                    break;
                }

                // insert new beam locations
                front.insert(j - 1);
                front.insert(j + 1);

                let prev = timelines[j];
                timelines[j] = 0;

                timelines[j - 1] += prev;
                timelines[j + 1] += prev;
            }
        }

        let p2 = timelines.iter().sum();
        Ok(Self { p1, p2 })
    }
}

impl Problem for Laboratories {
    const DAY: usize = 7;
    const TITLE: &'static str = "laboratories";
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
        let solution = Laboratories::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1642, 47274292756692));
    }

    #[test]
    fn example() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let solution = Laboratories::solve(input).unwrap();
        assert_eq!(solution, Solution::new(21, 40));
    }
}
