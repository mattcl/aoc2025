use std::str::FromStr;

use anyhow::anyhow;

use aoc_plumbing::Problem;
use aoc_std::geometry::Interval;

#[derive(Debug, Clone)]
pub struct Cafeteria {
    p1: usize,
    p2: u64,
}

impl FromStr for Cafeteria {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_ranges, raw_ids) = s
            .trim()
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("invalid input"))?;

        let mut ranges: Vec<Interval<u64>> = Vec::default();

        for line in raw_ranges.lines() {
            let (start, end) = line
                .split_once("-")
                .ok_or_else(|| anyhow!("invalid input"))?;
            ranges.push(Interval::new(start.parse()?, end.parse()?));
        }

        let mut p1 = 0;

        ranges.sort();
        let mut merged = Vec::with_capacity(ranges.len());

        let mut cur = ranges.pop().ok_or_else(|| anyhow!("invalid input"))?;

        let mut p2 = 0;

        while let Some(next) = ranges.pop() {
            if cur.overlaps(&next) {
                cur = Interval::new(cur.start.min(next.start), cur.end.max(next.end));
            } else {
                p2 += cur.width();
                merged.push(cur);
                cur = next;
            }
        }

        p2 += cur.width();
        merged.push(cur);

        // because of the way we merged, this is already sorted, but in reverse
        merged.reverse();

        for line in raw_ids.lines() {
            let id: u64 = line.parse()?;

            let mut left = 0;
            let mut right = merged.len() - 1;

            while left <= right {
                let mid = left + ((right - left) / 2);
                let cur = &merged[mid];

                if cur.contains_value(id) {
                    p1 += 1;
                    break;
                }

                if cur.end < id {
                    left = mid + 1;
                } else if cur.start > id && mid > 0 {
                    right = mid - 1;
                } else {
                    break;
                }
            }
        }

        Ok(Self { p1, p2 })
    }
}

impl Problem for Cafeteria {
    const DAY: usize = 5;
    const TITLE: &'static str = "cafeteria";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
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
        let solution = Cafeteria::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(607, 342433357244012));
    }

    #[test]
    fn example() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let solution = Cafeteria::solve(input).unwrap();
        assert_eq!(solution, Solution::new(3, 14));
    }
}
