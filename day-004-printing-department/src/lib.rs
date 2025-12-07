use std::{collections::VecDeque, str::FromStr};

use aoc_plumbing::Problem;
use aoc_std::{collections::BitSet, geometry::Location};

pub type PrintingDepartment = PrintingDepartmentGen<3>;

#[derive(Debug, Clone)]
pub struct PrintingDepartmentGen<const M: usize> {
    p1: usize,
    p2: usize,
}

impl<const M: usize> FromStr for PrintingDepartmentGen<M> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p1 = 0;

        let height = s.trim().lines().count();
        // let mut removed = VecDeque::default();
        let mut removed = VecDeque::default();
        let mut grid = vec![BitSet::<M>::ZERO; height + 2];

        let mut width = 0;

        for (row, line) in s.trim().lines().enumerate() {
            width = line.len();
            for (col, ch) in line.as_bytes().iter().enumerate() {
                if ch == &b'@' {
                    grid[row + 1].insert(col + 1);
                }
            }
        }

        let mut seen = vec![vec![0_u8; width]; height];
        let mut masks = vec![BitSet::<M>::zero(); width + 2];

        let mut base_mask = BitSet::<M>::zero();
        base_mask.set_lower(0b111);

        #[allow(clippy::needless_range_loop)]
        for col in 1..(width + 1) {
            masks[col] = base_mask << (col - 1);
        }

        for row in 1..(height + 1) {
            let mut col = 0;
            while let Some(next) = grid[row].next_beyond(col) {
                col = next;

                if col > height {
                    break;
                }

                let count = (grid[row] & masks[col]).count()
                    + (grid[row - 1] & masks[col]).count()
                    + (grid[row + 1] & masks[col]).count()
                    - 1;

                let loc = Location::new(row - 1, col - 1);
                seen[loc.row][loc.col] = count as u8;
                if count < 4 {
                    p1 += 1;
                    removed.push_back(loc);
                }
            }
        }

        let mut p2 = 0;

        while let Some(loc) = removed.pop_back() {
            seen[loc.row][loc.col] = 0;
            p2 += 1;

            for (_, neighbor) in loc
                .neighbors()
                .filter(|(_, n)| n.col < width && n.row < height)
            {
                let val = seen[neighbor.row][neighbor.col];
                if val == 4 {
                    seen[neighbor.row][neighbor.col] -= 1;
                    removed.push_back(neighbor);
                } else if val > 1 {
                    seen[neighbor.row][neighbor.col] -= 1;
                }
            }
        }

        Ok(Self { p1, p2 })
    }
}

impl<const M: usize> Problem for PrintingDepartmentGen<M> {
    const DAY: usize = 4;
    const TITLE: &'static str = "printing department";
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
        let solution = PrintingDepartment::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1441, 9050));
    }

    #[test]
    fn example() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let solution = PrintingDepartmentGen::<1>::solve(input).unwrap();
        assert_eq!(solution, Solution::new(13, 43));
    }
}
