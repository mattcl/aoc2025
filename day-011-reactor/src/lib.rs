use std::{collections::hash_map::Entry, str::FromStr, u32, usize};

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
    idx: usize,
    name: u32,
    outputs: Vec<usize>,
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
        let fft = get_init_node_idx(make_id("fft"), &mut nodes, &mut seen);
        let dac = get_init_node_idx(make_id("dac"), &mut nodes, &mut seen);
        let you = get_init_node_idx(make_id("you"), &mut nodes, &mut seen);
        let out = get_init_node_idx(make_id("out"), &mut nodes, &mut seen);
        let svr = get_init_node_idx(make_id("svr"), &mut nodes, &mut seen);

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

        let mut cache = FxHashMap::default();
        let mut fft_seen = false;
        let mut dac_seen = false;
        let p1 = explore_with_seen(you, out, &nodes, &mut cache, &mut fft_seen, &mut dac_seen);

        #[allow(unused_assignments)]
        let mut fft_first = false;
        #[allow(unused_assignments)]
        let mut a = 0;
        // okay, so if we've seen both, we have to gamble, but, if we only saw
        // one, we know the _other_ one is first. if we saw neither, we have to
        // gamble
        if fft_seen && dac_seen || fft_seen == dac_seen {
            // gamble
            let mut fft_dac_cache = FxHashMap::default();
            a = explore(fft, dac, &nodes, &mut fft_dac_cache);
            fft_first = a > 0;

            // if our gamble is wrong we need to explore the other option
            if !fft_first {
                fft_dac_cache.clear();
                a = explore(dac, fft, &nodes, &mut fft_dac_cache);
            }
        } else if fft_seen {
            // fft is _second
            fft_first = false;
            let mut fft_dac_cache = FxHashMap::default();
            a = explore(dac, fft, &nodes, &mut fft_dac_cache);
        } else {
            // fft is first
            fft_first = true;
            let mut fft_dac_cache = FxHashMap::default();
            a = explore(fft, dac, &nodes, &mut fft_dac_cache);
        }

        let b = if fft_first {
            let mut svr_fft_cache = FxHashMap::default();
            explore(svr, fft, &nodes, &mut svr_fft_cache)
        } else {
            let mut svr_dac_cache = FxHashMap::default();
            explore(svr, dac, &nodes, &mut svr_dac_cache)
        };

        // we can use the same cache from part 1, because some of the paths
        // below YOU would contain fft or dac
        let c = if fft_first {
            explore(dac, out, &nodes, &mut cache)
        } else {
            explore(fft, out, &nodes, &mut cache)
        };

        let p2 = a * b * c;

        Ok(Self { p1, p2 })
    }
}

fn get_init_node_idx(id: u32, nodes: &mut Vec<Node>, seen: &mut FxHashMap<u32, usize>) -> usize {
    match seen.entry(id) {
        Entry::Occupied(entry) => *entry.get(),
        Entry::Vacant(entry) => {
            let idx = nodes.len();
            let new = Node {
                idx,
                name: id,
                outputs: Vec::default(),
            };
            nodes.push(new);
            entry.insert(idx);
            idx
        }
    }
}

fn explore_with_seen(
    cur: usize,
    target: usize,
    nodes: &[Node],
    cache: &mut FxHashMap<usize, usize>,
    fft_seen: &mut bool,
    dac_seen: &mut bool,
) -> usize {
    if cur == target {
        return 1;
    }

    if cur == FFT {
        *fft_seen = true;
    }

    if cur == DAC {
        *dac_seen = true;
    }

    if let Some(&prev) = cache.get(&cur) {
        return prev;
    }

    let node = &nodes[cur];

    let mut paths = 0;
    for &ch in node.outputs.iter() {
        paths += explore_with_seen(ch, target, nodes, cache, fft_seen, dac_seen);
    }

    cache.insert(cur, paths);
    paths
}

fn explore(
    cur: usize,
    target: usize,
    nodes: &[Node],
    cache: &mut FxHashMap<usize, usize>,
) -> usize {
    if cur == target {
        return 1;
    }

    if let Some(&prev) = cache.get(&cur) {
        return prev;
    }

    let node = &nodes[cur];

    let mut paths = 0;
    for &ch in node.outputs.iter() {
        paths += explore(ch, target, nodes, cache);
    }

    cache.insert(cur, paths);
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

    #[test]
    fn example() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let solution = Reactor::solve(input).unwrap();
        assert_eq!(solution, Solution::new(5, 0));
    }

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
