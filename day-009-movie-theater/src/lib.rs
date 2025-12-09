use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;
use aoc_std::geometry::{Point2D, Rectangle};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Line {
    left: Point2D<usize>,
    right: Point2D<usize>,
}

impl Line {
    pub fn new(left: Point2D<usize>, right: Point2D<usize>) -> Self {
        // avoid the extra copy of using max/min
        if left > right {
            Self {
                left: right,
                right: left,
            }
        } else {
            Self { left, right }
        }
    }

    pub fn intersects(&self, rect: &Rectangle<usize>) -> bool {
        if (rect.p1().x < self.left.x
            && rect.p2().x > self.left.x
            && rect.p1().y < self.left.y
            && rect.p2().y > self.left.y)
            || (rect.p1().x < self.right.x
                && rect.p2().x > self.right.x
                && rect.p1().y < self.right.y
                && rect.p2().y > self.right.y)
        {
            return true;
        }

        if self.left.x == self.right.x {
            // we are vertical
            let horiz = Line::new(*rect.p1(), Point2D::new(rect.p2().x, rect.p1().y));
            if intersects_perpendicular(self, &horiz) {
                return true;
            }

            let horiz = Line::new(*rect.p2(), Point2D::new(rect.p1().x, rect.p2().y));
            if intersects_perpendicular(self, &horiz) {
                return true;
            }
        } else {
            // we are horizontal
            let horiz = Line::new(*rect.p1(), Point2D::new(rect.p1().x, rect.p2().y));
            if intersects_perpendicular(&horiz, self) {
                return true;
            }

            let horiz = Line::new(*rect.p2(), Point2D::new(rect.p2().x, rect.p1().y));
            if intersects_perpendicular(&horiz, self) {
                return true;
            }
        }

        false
    }
}

// this doesn't need to be generic for any line, just the ones we're dealing with
pub fn intersects_perpendicular(vert: &Line, horiz: &Line) -> bool {
    vert.left.x > horiz.left.x
        && vert.left.x < horiz.right.x
        && vert.left.y < horiz.right.y
        && vert.right.y > horiz.left.y
}

#[derive(Debug, Clone)]
pub struct MovieTheater {
    p1: usize,
    p2: usize,
}

impl FromStr for MovieTheater {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = Vec::with_capacity(800);
        let mut segments = Vec::with_capacity(800);

        for line in s.trim().lines() {
            let (rx, ry) = line
                .split_once(',')
                .ok_or_else(|| anyhow!("invalid input"))?;
            let pt = Point2D::<usize>::new(rx.parse()?, ry.parse()?);
            points.push(pt);
        }

        // we're going to exploit how the inputs are actually shaped instead of
        // doing the winding order intersection thing. While this will be generic
        // over all official inputs, this won't be general for non-official inputs
        //
        // i wasn't expecting this to work on the example input as well, but it
        // does apparently, just by chance
        //
        // if we don't make this assumption, this _can_ solve the general case,
        // just without the benefit of drastically reducing the search space
        let mut prev = points[points.len() - 1];
        // let mut winding_area = 0;

        for (idx, point) in points.iter().enumerate() {
            // winding_area += (point.x as i64 - prev.x as i64) * (point.y as i64 - prev.y as i64);
            let length = point.x.abs_diff(prev.x).max(point.y.abs_diff(prev.y));
            segments.push((length, idx, Line::new(prev, *point)));
            prev = *point;
        }

        // sorting this will do two things:
        // 1) allow us to find the two candidate points for one corner of the rect
        // 2) make the two longest lines at the start of the list which can help
        //    with intersection checking
        segments.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let mut p1 = usize::MIN;
        for (i, a) in points.iter().enumerate() {
            #[allow(clippy::needless_range_loop)]
            for j in i + 1..points.len() {
                let b = &points[j];
                if a == b {
                    continue;
                }

                let rect = Rectangle::new(*a, *b);
                let area = rect.area();
                if area > p1 {
                    p1 = area;
                }
            }
        }

        // we know because of the input that one of the points has to be on the
        // longest segment(s)
        let c1 = segments[0];
        let c2 = segments[1];

        // because we're CCW
        let (cw, ccw) = if c1.1 < c2.1 {
            ((points[c1.1], c1), (points[c2.1 - 1], c2))
        } else {
            ((points[c2.1], c2), (points[c1.1 - 1], c1))
        };

        let (fixed, (longest_possible_edge, idx, _)) = cw;
        let cw_max = find_largest(
            fixed,
            longest_possible_edge,
            idx - 1,
            false,
            &points,
            &segments,
        );

        let (fixed, (longest_possible_edge, idx, _)) = ccw;
        let ccw_max = find_largest(
            fixed,
            longest_possible_edge,
            idx + 1,
            true,
            &points,
            &segments,
        );

        Ok(Self {
            p1,
            p2: cw_max.max(ccw_max),
        })
    }
}

fn find_largest(
    fixed: Point2D<usize>,
    longest_possible_edge: usize,
    mut cur_idx: usize,
    ccw: bool,
    points: &[Point2D<usize>],
    segments: &[(usize, usize, Line)],
) -> usize {
    let min_edge = (longest_possible_edge * 2) / 3;
    let mut max = 0;
    if ccw {
        'outer: loop {
            let cur = &points[cur_idx];
            let length = cur.x.abs_diff(fixed.x);
            if length < min_edge {
                break;
            }

            cur_idx = (cur_idx + 1) % points.len();

            let rect = Rectangle::new(*cur, fixed);
            let area = rect.area();

            if area < max {
                continue;
            }

            for (_, _, seg) in segments.iter() {
                if seg.intersects(&rect) {
                    continue 'outer;
                }
            }

            max = area;
        }
    } else {
        'outer: loop {
            let cur = &points[cur_idx];
            let length = cur.x.abs_diff(fixed.x);
            if length < min_edge {
                break;
            }

            if cur_idx == 0 {
                cur_idx = points.len() - 1;
            } else {
                cur_idx -= 1;
            }

            let rect = Rectangle::new(*cur, fixed);
            let area = rect.area();

            if area < max {
                continue;
            }

            for (_, _, seg) in segments.iter() {
                if seg.intersects(&rect) {
                    continue 'outer;
                }
            }

            max = area;
        }
    }

    max
}

impl Problem for MovieTheater {
    const DAY: usize = 9;
    const TITLE: &'static str = "movie theater";
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
        let solution = MovieTheater::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(4771508457, 1539809693));
    }

    #[test]
    fn example() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let solution = MovieTheater::solve(input).unwrap();
        assert_eq!(solution, Solution::new(50, 24));
    }
}
