use core::f64;
use std::{ops::{Index, IndexMut}, str::FromStr};

use aoc_plumbing::Problem;
use aoc_std::collections::BitSet;
use nom::{branch, character::complete, combinator, multi::{fold_many1, separated_list1}, sequence::{delimited, preceded, tuple}, IResult};


#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    vals: Vec<T>,
    cols: usize,
}

impl<T> Matrix<T>
where T: Default + Copy,
{
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { vals: vec![T::default(); rows * cols], cols }
    }

    pub fn rows(&self) -> usize {
        self.vals.len() / self.cols
    }

    pub const fn cols(&self) -> usize {
        self.cols
    }

    pub fn swap_rows(&mut self, row_one: usize, row_two: usize) {
        if row_one == row_two {
            return;
        }

        let cols = self.cols;
        for col in 0..cols {
            self.vals.swap(row_one * cols + col, row_two * cols + col);
        }
    }

    pub fn add_row<Iter: IntoIterator<Item = T>>(&mut self, vals: Iter) {
        self.vals.extend(vals);
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        let r = row * self.cols;
        &self.vals[r..(r + self.cols)]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let r = row * self.cols;
        &mut self.vals[r..(r + self.cols)]
    }
}

// f64::EPSILON is too small
const EPSILON: f64 = 1e-9;

/// I don't really care for problems like this, whese the usual "best" answer is
/// to use z3 or something. So we're going to cobble together this blob form
/// hints from reddit and wikipedia
///
/// https://en.wikipedia.org/wiki/Branch_and_bound
/// https://en.wikipedia.org/wiki/Branch_and_cut
/// https://en.wikipedia.org/wiki/Revised_simplex_method
fn simplex(mat: &Matrix<f64>, c: &[f64]) -> (f64, Option<Vec<f64>>) {
    let rows = mat.rows();
    let cols = mat.cols() - 1;

    let mut n_indices: Vec<i32> = (0..cols as i32).collect();
    n_indices.push(-1);

    let mut b_indices: Vec<i32> = (cols as i32..(rows + cols) as i32).collect();
    let mut d = Matrix::new(rows + 2, cols + 2);

    for row in 0..rows {
        d[row][..=cols].copy_from_slice(&mat[row]);
        d[row][cols + 1] = -1.0;
    }

    for row in 0..rows {
        d[row].swap(cols, cols + 1);
    }

    d[rows][..cols].copy_from_slice(&c[..cols]);
    d[rows + 1][cols] = 1.0;

    let mut split_r = 0;
    let mut min_val = d[0][cols + 1];

    for row in 1..rows {
        if d[row][cols + 1] < min_val {
            min_val = d[row][cols + 1];
            split_r = row;
        }
    }

    if d[split_r][cols + 1] < -EPSILON {
        pivot(&mut d, &mut b_indices, &mut n_indices, split_r, cols);

        if !find(&mut d, &mut b_indices, &mut n_indices, 1, rows, cols) || d[rows + 1][cols + 1] < -EPSILON {
            return (f64::NEG_INFINITY, None);
        }

        for i in 0..rows {
            if b_indices[i] == -1 {
                let mut best_s = 0;
                let mut best_key = (d[i][0], n_indices[0]);
                #[allow(clippy::needless_range_loop)]
                for j in 1..cols {
                    let key = (d[i][j], n_indices[j]);
                    if key.0 < best_key.0 - EPSILON
                        || ((key.0 - best_key.0).abs() <= EPSILON && key.1 < best_key.1)
                    {
                        best_s = j;
                        best_key = key;
                    }
                }
                pivot(&mut d, &mut b_indices, &mut n_indices, i, best_s);
            }
        }
    }

    if find(&mut d, &mut b_indices, &mut n_indices, 0, rows, cols) {
        let mut x = vec![0.0; cols];
        for i in 0..rows {
            if b_indices[i] >= 0 && (b_indices[i] as usize) < cols {
                x[b_indices[i] as usize] = d[i][cols + 1];
            }
        }
        let mut sum_val = 0.0;
        for i in 0..cols {
            sum_val += c[i] * x[i];
        }
        return (sum_val, Some(x));
    }

    (f64::NEG_INFINITY, None)
}

fn pivot(
    d: &mut Matrix<f64>,
    b_idx: &mut [i32],
    n_idx: &mut [i32],
    r: usize,
    s: usize,
) {
    let k = 1.0 / d[r][s];

    let d_rows = d.rows();
    let d_cols = d.cols();
    for i in 0..d_rows {
        if i == r {
            continue;
        }
        for j in 0..d_cols {
            if j != s {
                d[i][j] -= d[r][j] * d[i][s] * k;
            }
        }
    }

    for val in d[r].iter_mut() {
        *val *= k;
    }

    for row in 0..d_rows {
        d[row][s] *= -k;
    }

    d[r][s] = k;

    std::mem::swap(&mut b_idx[r], &mut n_idx[s]);
}

fn find(
    d: &mut Matrix<f64>,
    b_idx: &mut [i32],
    n_idx: &mut [i32],
    p_idx: usize,
    m: usize,
    n: usize,
) -> bool {
    loop {
        let mut best_s = usize::MAX;
        let mut best_val = (f64::INFINITY, i32::MAX);

        #[allow(clippy::needless_range_loop)]
        for i in 0..=n {
            if p_idx != 0 || n_idx[i] != -1 {
                let val = d[m + p_idx][i];
                let key = (val, n_idx[i]);
                if best_s == usize::MAX
                    || key.0 < best_val.0 - EPSILON
                    || ((key.0 - best_val.0).abs() <= EPSILON && key.1 < best_val.1)
                {
                    best_s = i;
                    best_val = key;
                }
            }
        }
        let s = best_s;

        if d[m + p_idx][s] > -EPSILON {
            return true;
        }

        let mut best_r = usize::MAX;
        let mut best_r_key = (f64::INFINITY, i32::MAX);

        for i in 0..m {
            if d[i][s] > EPSILON {
                let ratio = d[i][n + 1] / d[i][s];
                let key = (ratio, b_idx[i]);
                if best_r == usize::MAX
                    || key.0 < best_r_key.0 - EPSILON
                    || ((key.0 - best_r_key.0).abs() <= EPSILON && key.1 < best_r_key.1)
                {
                    best_r = i;
                    best_r_key = key;
                }
            }
        }
        let r = best_r;

        if r == usize::MAX {
            return false;
        }

        pivot(d, b_idx, n_idx, r, s);
    }
}

fn branch_and_bound(init: Matrix<f64>, coefficients: &[f64]) -> usize {
    let mut best = f64::INFINITY;
    let mut stack = Vec::new();
    stack.push(init);

    while let Some(cur) = stack.pop() {
        let (val, x_opt) = simplex(&cur, coefficients);

        if val == f64::NEG_INFINITY || val >= best - EPSILON {
            continue;
        }

        let mut fractional_idx = None;
        let mut fractional_val = 0.0;

        if let Some(x) = x_opt {
            for (i, &xv) in x.iter().enumerate() {
                if (xv - xv.round()).abs() > EPSILON {
                    fractional_idx = Some(i);
                    fractional_val = xv;
                    break;
                }
            }

            if let Some(idx) = fractional_idx {
                let floor_v = fractional_val.floor();
                let n_cols = cur.cols();

                let mut row1 = vec![0.0; n_cols];
                row1[idx] = 1.0;
                row1[n_cols - 1] = floor_v;
                let mut a1 = cur.clone();
                a1.add_row(row1);
                stack.push(a1);

                let ceil_v = fractional_val.ceil();
                let mut row2 = vec![0.0; n_cols];
                row2[idx] = -1.0;
                row2[n_cols - 1] = -ceil_v;
                let mut a2 = cur.clone();
                a2.add_row(row2);
                stack.push(a2);
            } else if val < best {
                best = val;
            }
        }
    }

    if best == f64::INFINITY {
        0
    } else {
        best.round() as usize
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Machine {
    target: u16,
    buttons: Vec<u16>,
    joltages: Vec<u16>,
}

impl Machine {
    pub fn fewest_indicator_presses(
        &self,
        front: &mut Vec<u16>,
        next: &mut Vec<u16>,
    ) -> usize {
        front.clear();
        front.push(0);
        next.clear();

        let mut count = 0;
        let mut seen = BitSet::<16>::zero();
        seen.insert(0);
        loop {
            count += 1;
            for cur in front.drain(..) {
                for b in self.buttons.iter() {
                    let new = cur ^ b;
                    if new == self.target {
                        return count;
                    }

                    if !seen.contains(new as usize) {
                        seen.insert(new as usize);
                        next.push(new);
                    }
                }
            }

            if next.is_empty() {
                break;
            }

            std::mem::swap(front, next);
        }

        usize::MAX
    }

    pub fn fewest_joltage_presses(
        &self,
    ) -> usize {
        let num_joltages = self.joltages.len();
        let num_buttons = self.buttons.len();

        let rows = 2 * num_joltages + num_buttons;
        let cols = num_buttons + 1;
        let mut matrix = Matrix::new(rows, cols);

        for (b, r) in (0..rows).rev().take(num_buttons).enumerate() {
            matrix[r][b] = -1.0;
        }

        for col in 0..num_buttons {
            for row in 0..num_joltages {
                if (self.buttons[col] >> row) & 1 == 1 {
                    matrix[row][col] = 1.0;
                    matrix[row + num_joltages][col] = -1.0;
                }
            }
        }

        for row in 0..num_joltages {
            let val = self.joltages[row] as f64;
            matrix[row][cols - 1] = val;
            matrix[row + num_joltages][cols - 1] = -val;
        }

        let coefficients = vec![1.0; num_buttons];
        branch_and_bound(matrix, &coefficients)
    }
}

#[derive(Debug, Clone)]
pub struct Factory {
    p1: usize,
    p2: usize,
}

impl FromStr for Factory {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut indicators_front = Vec::with_capacity(1024);
        let mut indicators_next = Vec::with_capacity(1024);
        let mut p1 = 0;
        let mut p2 = 0;
        for line in s.trim().lines() {
            let (_, machine) = parse_machine(line).map_err(|e| e.to_owned())?;
            p1 += machine.fewest_indicator_presses(&mut indicators_front, &mut indicators_next);
            p2 += machine.fewest_joltage_presses();
        }

        Ok(Self { p1, p2 })
    }
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    combinator::map(
        tuple((
            parse_target,
            parse_buttons,
            parse_joltages,
        )),
        |(target, buttons, joltages)| Machine { target, buttons, joltages }
    )(input)
}

fn parse_target(input: &str) -> IResult<&str, u16> {
    combinator::map(
        delimited(
            complete::char('['),
            fold_many1(
                complete::none_of("]"),
                || (0_u16, 0_u16),
                |(count, mut acc): (u16, u16), item| {
                    if item == '#' {
                        acc |= 1 << count;
                    }
                    (count + 1, acc)
                }
            ),
            complete::char(']')
        ),
        |(_, v)| v
    )(input)
}

fn parse_buttons(input: &str) -> IResult<&str, Vec<u16>> {
    preceded(
        complete::space0,
        separated_list1(complete::space1, parse_button)
    )(input)
}

fn parse_button(input: &str) -> IResult<&str, u16> {
    delimited(
        complete::char('('),
        fold_many1(
            branch::alt((
                preceded(complete::char(','), complete::u16),
                complete::u16,
            )),
            || 0_u16,
            |mut acc: u16, item| {
                acc |= 1 << item;
                acc
            }
        ),
        complete::char(')')
    )(input)
}

fn parse_joltages(input: &str) -> IResult<&str, Vec<u16>> {
    preceded(
        complete::space0,
        delimited(
            complete::char('{'),
            separated_list1(complete::char(','), complete::u16),
            complete::char('}')
        )
    )(input)
}

impl Problem for Factory {
    const DAY: usize = 10;
    const TITLE: &'static str = "factory";
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
        let solution = Factory::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(520, 20626));
    }

    #[test]
    fn example() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let solution = Factory::solve(input).unwrap();
        assert_eq!(solution, Solution::new(7, 33));
    }
}
