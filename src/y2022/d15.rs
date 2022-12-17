use std::collections::{HashMap, HashSet};

use fancy_regex::Regex;

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): .+ x=(-?\d+), y=(-?\d+)").unwrap();
}

#[allow(dead_code)]
const TEST_INPUT: &str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

type Point = (i64, i64);

#[derive(Debug)]
struct Grid {
    sensors: HashMap<Point, i64>,
    beacons: HashSet<Point>,
    left: i64,
    right: i64,
}
impl Grid {
    fn new() -> Self {
        Self {
            sensors: HashMap::new(),
            beacons: HashSet::new(),
            left: i64::MAX,
            right: i64::MIN,
        }
    }
}

fn setup(input: &str) -> Grid {
    input
        .split("\n")
        .filter_map(|s| RE.captures(s).unwrap().map(|caps| {
                let x: i64 = caps.get(1).unwrap().as_str().parse().unwrap();
                let y: i64 = caps.get(2).unwrap().as_str().parse().unwrap();
                let b_x: i64 = caps.get(3).unwrap().as_str().parse().unwrap();
                let b_y: i64 = caps.get(4).unwrap().as_str().parse().unwrap();
                (((x, y), (b_x - x).abs() + (b_y - y).abs()), (b_x, b_y))
        }))
        .fold(Grid::new(), |mut g, ((s, d), b)| {
            g.sensors.insert(s, d);
            g.beacons.insert(b);
            g.left = std::cmp::min(s.0 - d, g.left);
            g.right = std::cmp::max(s.0 + d, g.right);
            g
        })
}

fn part1(grid: &Grid) -> usize {
    let y = 2_000_000;
    let points = grid.sensors.iter().filter_map(|(&(s_x, s_y), &d)| {
        let dy = (s_y - y).abs();
        if dy > d {
            None
        } else {
            let dx = (dy - d).abs();
            Some(s_x - dx..=s_x + dx)
        }
    }).flatten().collect::<HashSet<_>>();
    points.len() - grid.beacons.iter().filter(|&&(_, b_y)| b_y == y).count()
}

pub fn main(parts: crate::RunPart) {
    use crate::dispatcher::*;
    DispatcherBuilder::setup(setup)
        .part1(part1)
        // .run(TEST_INPUT, parts);
        .run(include_str!("../../../input/2022/15.txt"), parts);
}
