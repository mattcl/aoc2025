use std::{collections::hash_map::Entry, ops::AddAssign, str::FromStr, u32, usize};

use anyhow::anyhow;
use aoc_plumbing::Problem;
use rustc_hash::FxHashMap;

const FFT: usize = 0;
const DAC: usize = 1;

const fn make_id(name: &str) -> u32 {
    let b = name.as_bytes();
    ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node {
    outputs: Vec<usize>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
    fft: usize,
    dac: usize,
    both: usize,
    none: usize,
}

impl AddAssign<State> for State {
    fn add_assign(&mut self, rhs: State) {
        self.fft += rhs.fft;
        self.dac += rhs.dac;
        self.both += rhs.both;
        self.none += rhs.none;
    }
}

#[derive(Debug, Clone)]
pub struct Reactor {
    p1: usize,
    p2: usize,
}

impl FromStr for Reactor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes: Vec<Node> = Vec::default();
        let mut seen: FxHashMap<u32, usize> = FxHashMap::default();

        // insert these two first so they have idx 0 and 1
        let _fft = get_init_node_idx(make_id("fft"), &mut nodes, &mut seen);
        let _dac = get_init_node_idx(make_id("dac"), &mut nodes, &mut seen);
        // then the others
        let you = get_init_node_idx(make_id("you"), &mut nodes, &mut seen);
        let out = get_init_node_idx(make_id("out"), &mut nodes, &mut seen);
        let svr = get_init_node_idx(make_id("svr"), &mut nodes, &mut seen);
        // at this point, the next index is 5

        for line in s.trim().lines() {
            let (name, rem) = line
                .split_once(": ")
                .ok_or_else(|| anyhow!("invalid input"))?;
            let id = make_id(name);

            let idx = get_init_node_idx(id, &mut nodes, &mut seen);

            for output in rem.split(' ') {
                let child_id = make_id(output);
                let child_idx = get_init_node_idx(child_id, &mut nodes, &mut seen);
                nodes[idx].outputs.push(child_idx);
            }
        }

        let mut cache = vec![None; nodes.len()];
        let State { both: p2, .. } = explore(svr, out, &nodes, &mut cache);
        let p1 = cache[you].take().unwrap_or_default().none;

        Ok(Self { p1, p2 })
    }
}

fn get_init_node_idx(id: u32, nodes: &mut Vec<Node>, seen: &mut FxHashMap<u32, usize>) -> usize {
    match seen.entry(id) {
        Entry::Occupied(entry) => *entry.get(),
        Entry::Vacant(entry) => {
            let idx = nodes.len();
            let new = Node {
                outputs: Vec::default(),
            };
            nodes.push(new);
            entry.insert(idx);
            idx
        }
    }
}

fn explore(cur: usize, target: usize, nodes: &[Node], cache: &mut [Option<State>]) -> State {
    if let Some(prev) = cache[cur] {
        return prev;
    }

    if cur == target {
        return State {
            none: 1,
            ..Default::default()
        };
    }

    let node = &nodes[cur];

    let mut paths = State::default();
    for &ch in node.outputs.iter() {
        let res = explore(ch, target, nodes, cache);
        paths += res;
        if cur == FFT {
            paths.fft += res.none;
            paths.both += res.dac;
        } else if cur == DAC {
            paths.dac += res.none;
            paths.both += res.fft;
        }
    }

    cache[cur] = Some(paths);
    paths
}

impl Problem for Reactor {
    const DAY: usize = 11;
    const TITLE: &'static str = "reactor";
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
        let solution = Reactor::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(585, 349322478796032));
    }

    // #[test]
    // fn example() {
    //     let input = "aaa: you hhh
    // you: bbb ccc
    // bbb: ddd eee
    // ccc: ddd eee fff
    // ddd: ggg
    // eee: out
    // fff: out
    // ggg: out
    // hhh: ccc fff iii
    // iii: out";
    //     let solution = Reactor::solve(input).unwrap();
    //     assert_eq!(solution, Solution::new(5, 0));
    // }

    #[test]
    fn example_part_2() {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let solution = Reactor::solve(input).unwrap();
        assert_eq!(solution, Solution::new(0, 2));
    }
}
