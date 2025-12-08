use std::{ops::Index, str::FromStr};

use anyhow::bail;
use aoc_plumbing::Problem;
use aoc_std::{
    collections::BitSet,
    geometry::{AocPoint, Point3D},
};
use nom::{
    IResult,
    character::complete,
    combinator,
    sequence::{preceded, tuple},
};
use rayon::slice::ParallelSliceMut;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node {
    parent: usize,
    size: usize,
}

impl Node {
    pub const fn new(parent: usize) -> Self {
        Self { parent, size: 1 }
    }
}

// TODO: Move to aoc_std - MCL - 2025-12-08
// based on https://en.wikipedia.org/wiki/Disjoint-set_data_structure
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DisjointSet {
    nodes: Vec<Node>,
}

impl DisjointSet {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, parent: usize) {
        self.nodes.push(Node::new(parent));
    }

    /// find and improve subsequent finds
    pub fn find(&mut self, idx: usize) -> usize {
        if self.nodes[idx].parent != idx {
            self.nodes[idx].parent = self.find(self.nodes[idx].parent);
            return self.nodes[idx].parent;
        }

        idx
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let left = self.find(a);
        let right = self.find(b);

        if left == right {
            return;
        }

        if self.nodes[left].size < self.nodes[right].size {
            self.nodes[left].parent = right;
            self.nodes[right].size += self.nodes[left].size;
        } else {
            self.nodes[right].parent = left;
            self.nodes[left].size += self.nodes[right].size;
        }
    }
}

impl Index<usize> for DisjointSet {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

pub type Playground = PlaygroundGen<1_000, 1_000>;

#[derive(Debug, Clone)]
pub struct PlaygroundGen<const N: usize, const M: usize> {
    p1: usize,
    p2: i64,
}

impl<const N: usize, const M: usize> FromStr for PlaygroundGen<N, M> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = Vec::with_capacity(N);
        let mut dists = Vec::with_capacity(N * N / 2);
        for line in s.trim().lines() {
            let (_, coord) = parse_coord(line).map_err(|e| e.to_owned())?;
            points.push(coord);
        }

        for left in 0..points.len() {
            for right in (left + 1)..points.len() {
                let d = points[left].euclidean_dist_sq(&points[right]);
                dists.push((d, left, right));
            }
        }

        dists.par_sort_unstable_by(|a, b| a.0.cmp(&b.0));
        // return Ok(Self { p1: dists.len(), p2: 0 });

        let mut disjoint_set = DisjointSet::with_capacity(N);
        for i in 0..N {
            disjoint_set.insert(i);
        }

        let mut iter = dists.into_iter();

        let mut groups = points.len();
        let mut count = 0;

        // clippy is wrong about this
        #[allow(clippy::while_let_on_iterator)]
        while let Some((_, left, right)) = iter.next() {
            let a = disjoint_set.find(left);
            let b = disjoint_set.find(right);

            if a != b {
                groups -= 1;
                disjoint_set.union(a, b);
            }

            count += 1;
            if count >= M {
                break;
            }
        }

        let mut sizes = Vec::default();
        let mut seen = BitSet::<16>::zero();

        for i in 0..N {
            let p = disjoint_set.find(i);
            if !seen.contains(p) {
                seen.insert(p);
                sizes.push(disjoint_set[p].size);
            }
        }

        sizes.sort_unstable();

        let p1 = sizes.iter().rev().take(3).product();

        // okay, keep making connections until we only have one group left
        for (_, left, right) in iter {
            let a = disjoint_set.find(left);
            let b = disjoint_set.find(right);

            if a != b {
                groups -= 1;
                disjoint_set.union(a, b);

                if groups < 2 {
                    let p2 = points[left].x * points[right].x;
                    return Ok(Self { p1, p2 });
                }
            }
        }

        bail!("no solution found");
    }
}

fn parse_coord(input: &str) -> IResult<&str, Point3D<i64>> {
    combinator::map(
        tuple((
            complete::i64,
            preceded(complete::char(','), complete::i64),
            preceded(complete::char(','), complete::i64),
        )),
        |(x, y, z)| Point3D::new(x, y, z),
    )(input)
}

impl<const N: usize, const M: usize> Problem for PlaygroundGen<N, M> {
    const DAY: usize = 8;
    const TITLE: &'static str = "playground";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = i64;

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
        let solution = Playground::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(47040, 4884971896));
    }

    #[test]
    fn example() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let solution = PlaygroundGen::<20, 10>::solve(input).unwrap();
        assert_eq!(solution, Solution::new(40, 25272));
    }
}
