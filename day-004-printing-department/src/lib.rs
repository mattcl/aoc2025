use std::{collections::VecDeque, str::FromStr};

use aoc_plumbing::Problem;
use aoc_std::{collections::BitSet, geometry::Location};

#[derive(Debug, Clone, Copy)]
struct MaskSet<const M: usize> {
    pub cur: BitSet<M>,
    pub adj: BitSet<M>,
}

impl<const M: usize> MaskSet<M> {
    pub fn set(&mut self, col: usize) {
        self.cur.insert(col - 1);
        self.cur.insert(col + 1);
        self.adj.insert(col - 1);
        self.adj.insert(col);
        self.adj.insert(col + 1);
    }
}

impl<const M: usize> Default for MaskSet<M> {
    fn default() -> Self {
        Self {
            cur: BitSet::ZERO,
            adj: BitSet::ZERO,
        }
    }
}

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
        let mut masks = vec![MaskSet::default(); width + 2];

        #[allow(clippy::needless_range_loop)]
        for col in 1..(width + 1) {
            masks[col].set(col);
        }

        for row in 1..(height + 1) {
            let mut col = 0;
            while let Some(next) = grid[row].next_beyond(col) {
                col = next;

                if col > height {
                    break;
                }

                let count = (grid[row] & masks[col].cur).count()
                    + (grid[row - 1] & masks[col].adj).count()
                    + (grid[row + 1] & masks[col].adj).count();

                let loc = Location::new(row - 1, col - 1);
                seen[loc.row][loc.col] = count as u8;
                if count < 4 {
                    p1 += 1;
                    removed.push_back(loc);
                }
            }
        }

        let mut p2 = 0;

        while let Some(loc) = removed.pop_front() {
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
