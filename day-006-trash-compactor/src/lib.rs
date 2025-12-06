use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Op {
    Add = 0,
    Mul,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Value {
    pub col_idx: usize,
    pub last_idx: usize,
    pub normal: u64,
    column: [u64; 5],
    max_col: usize,
    op: Op,
}

impl Value {
    pub fn new(col_idx: usize, op: Op) -> Self {
        Self {
            col_idx,
            last_idx: usize::MAX,
            column: [0; 5],
            max_col: 0,
            normal: op as u64,
            op,
        }
    }

    pub fn append_normal(&mut self, val: u64) {
        match self.op {
            Op::Add => self.normal += val,
            Op::Mul => self.normal *= val,
        }
    }

    pub fn insert_digit(&mut self, idx: usize, val: u64) {
        self.column[idx] = 10 * self.column[idx] + val;
        self.max_col = self.max_col.max(idx);
    }

    pub fn column_val(&self) -> u64 {
        self.column
            .iter()
            .take(self.max_col + 1)
            .copied()
            .reduce(|acc, e| match self.op {
                Op::Add => acc + e,
                Op::Mul => acc * e,
            })
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
pub struct TrashCompactor {
    p1: u64,
    p2: u64,
}

impl FromStr for TrashCompactor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().lines().rev();

        let mut vals = Vec::default();
        for (idx, b) in iter
            .next()
            .ok_or_else(|| anyhow!("invalid input"))?
            .as_bytes()
            .iter()
            .enumerate()
        {
            match b {
                b'+' => vals.push(Value::new(idx, Op::Add)),
                b'*' => vals.push(Value::new(idx, Op::Mul)),
                _ => continue,
            }

            let len = vals.len();
            if vals.len() > 1 {
                vals[len - 2].last_idx = idx - 1;
            }
        }

        for line in iter.rev() {
            let bytes = line.as_bytes();
            for val in vals.iter_mut() {
                let mut cur = 0;
                let stop = val.last_idx.min(line.len());

                #[allow(clippy::needless_range_loop)]
                for cur_idx in (val.col_idx)..stop {
                    let b = bytes[cur_idx];
                    if b.is_ascii_digit() {
                        let rel_idx = cur_idx - val.col_idx;
                        let digit = (b - b'0') as u64;
                        cur = cur * 10 + digit;
                        val.insert_digit(rel_idx, digit);
                    }
                }

                if cur > 0 {
                    val.append_normal(cur);
                }
            }
        }

        let p1 = vals.iter().map(|o| o.normal).sum();
        let p2 = vals.iter().map(|o| o.column_val()).sum();

        Ok(Self { p1, p2 })
    }
}

impl Problem for TrashCompactor {
    const DAY: usize = 6;
    const TITLE: &'static str = "trash compactor";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u64;
    type P2 = u64;

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
        let solution = TrashCompactor::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(6299564383938, 11950004808442));
    }

    #[test]
    fn example() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        let solution = TrashCompactor::solve(input).unwrap();
        assert_eq!(solution, Solution::new(4277556, 3263827));
    }
}
