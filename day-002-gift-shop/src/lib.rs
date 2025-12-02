use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy)]
struct IdRange {
    left: usize,
    right: usize,
}

impl IdRange {
    fn invalid_sum(&self) -> usize {
        let mut sum = 0;

        let mut left = self.left;
        let mut right = self.right;

        let digits_left = left.checked_ilog10().unwrap_or(0) + 1;
        let mut digits_right = right.checked_ilog10().unwrap_or(0) + 1;

        if !digits_right.is_multiple_of(2) {
            right = 10_usize.pow(digits_right - 1) - 1;
            digits_right -= 1;
        }

        if digits_left < digits_right {
            left = 10_usize.pow(digits_right - 1);
        }

        let half_shift = 10_usize.pow(digits_right / 2);
        let half = left / half_shift;
        let mut cur = half;

        loop {
            let candidate = cur * half_shift + cur;
            if candidate > right {
                return sum;
            }
            if candidate >= left {
                sum += candidate;
            }

            cur += 1;
        }
    }

    fn invalid_sum_cached(&self, cache: &mut FxHashSet<usize>) -> usize {
        let mut sum = 0;

        let mut left = self.left;
        let mut right = self.right;

        let digits_left = left.checked_ilog10().unwrap_or(0) + 1;
        let mut digits_right = right.checked_ilog10().unwrap_or(0) + 1;

        if !digits_right.is_multiple_of(2) {
            right = 10_usize.pow(digits_right - 1) - 1;
            digits_right -= 1;
        }

        if digits_left < digits_right {
            left = 10_usize.pow(digits_right - 1);
        }

        let half_shift = 10_usize.pow(digits_right / 2);
        let half = left / half_shift;
        let mut cur = half;

        loop {
            let candidate = cur * half_shift + cur;
            if candidate > right {
                return sum;
            }
            if candidate >= left {
                cache.insert(candidate);
                sum += candidate;
            }

            cur += 1;
        }
    }

    fn multiple_invalid_sum(&self) -> usize {
        let mut seen = FxHashSet::default();
        let mut sum = self.invalid_sum_cached(&mut seen);

        let digits_left = self.left.checked_ilog10().unwrap_or(0) + 1;
        let digits_right = self.right.checked_ilog10().unwrap_or(0) + 1;

        let mut digits_cur = 1;
        let mut cur = if digits_left < digits_right {
            // we can start cur at any
            1
        } else {
            // we have to start cur at the first digit of left
            let mut left = self.left;
            loop {
                if left < 10 {
                    break left;
                }

                left /= 10;
            }
        };

        loop {
            let shift = 10_usize.pow(digits_cur);
            // we've already checked all the N == 2 variants, so bound lower at 3
            for replicas in (digits_left / digits_cur).max(3)..=(digits_right / digits_cur) {
                let candidate = build_candidate(cur, shift, replicas);

                if candidate > self.right {
                    continue;
                }

                if candidate >= self.left && seen.insert(candidate) {
                    sum += candidate;
                }
            }

            cur += 1;
            digits_cur = cur.checked_ilog10().unwrap_or(0) + 1;

            if digits_cur > digits_right / 3 {
                return sum;
            }
        }
    }
}

fn build_candidate(chunk: usize, shift: usize, replicas: u32) -> usize {
    let mut out = 0;
    for _ in 0..replicas {
        out = out * shift + chunk;
    }
    out
}

impl FromStr for IdRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left_c, right_c) = s
            .split_once('-')
            .ok_or_else(|| anyhow!("invalid range: {}", s))?;

        let left: usize = left_c.parse()?;
        let right: usize = right_c.parse()?;

        Ok(IdRange { left, right })
    }
}

#[derive(Debug, Clone)]
pub struct GiftShop {
    ranges: Vec<IdRange>,
}

impl FromStr for GiftShop {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .trim()
            .split(',')
            .map(|c| c.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { ranges })
    }
}

impl Problem for GiftShop {
    const DAY: usize = 2;
    const TITLE: &'static str = "gift shop";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.ranges.iter().map(|r| r.invalid_sum()).sum())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.ranges.iter().map(|r| r.multiple_invalid_sum()).sum())
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
        let solution = GiftShop::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(23701357374, 34284458938));
    }

    #[test]
    fn example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let solution = GiftShop::solve(input).unwrap();
        assert_eq!(solution, Solution::new(1227775554, 4174379265));
    }
}
