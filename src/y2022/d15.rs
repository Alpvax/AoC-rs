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
struct Sensor {
    x: i64,
    y: i64,
    d: i64,
    occlusions: HashMap<i64, (i64, i64)>,
}
impl Sensor {
    fn new((s_x, s_y): Point, (b_x, b_y): Point) -> Self {
        Self {
            x: s_x,
            y: s_y,
            d: (b_x - s_x).abs() + (b_y - s_y).abs(),
            occlusions: HashMap::new()
        }
    }
    fn calc_occlusions(&mut self) {
        for y in self.y - self.d..=self.y + self.d {
            self.occlusions.insert(y, self.occlusion(y).unwrap());
        }
    }
    fn occlusion(&self, y: i64) -> Option<(i64, i64)> {
        self.occlusions.get(&y).copied().or_else(|| {
            let dy = (y - self.y).abs();
            if dy < self.d {
                let dx = (self.d - dy).abs();
                Some((self.x - dx, self.x + dx))
            } else if dy == self.d {
                Some((self.x, self.x))
            } else {
                None
            }
        })
    }
    fn min_x(&self, x: i64) -> i64 {
        std::cmp::min(self.x - self.d, x)
    }
    fn max_x(&self, x: i64) -> i64 {
        std::cmp::max(self.x - self.d, x)
    }
}


#[derive(Debug)]
struct Grid {
    sensors: Vec<Sensor>,
    beacons: HashSet<Point>,
    left: i64,
    right: i64,
}
impl Grid {
    fn new() -> Self {
        Self {
            sensors: Vec::new(),
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
                (Sensor::new((x, y), (b_x, b_y)), (b_x, b_y))
        }))
        .fold(Grid::new(), |mut g, (mut s, b)| {
            g.beacons.insert(b);
            g.left = s.min_x(g.left);
            g.right = s.max_x(g.right);
            s.calc_occlusions();
            g.sensors.push(s);
            g
        })
}

fn part1(grid: &Grid) -> usize {
    let y = 2_000_000;
    let points = grid.sensors.iter().filter_map(|s| s.occlusion(y).map(|(x0, x1)| x0..=x1)).flatten().collect::<HashSet<_>>();
    points.len() - grid.beacons.iter().filter(|&&(_, b_y)| b_y == y).count()
}


fn part2(grid: Grid) -> i64 {
    for y in 0..=4_000_000 {
        let mut x = 0;
        let mut sensors = grid.sensors.iter().filter_map(|s| s.occlusion(y)).collect::<Vec<_>>();
        sensors.sort();
        for (sx0, sx1) in sensors {
            if grid.beacons.contains(&(x, y))  {
                x += 1;
            }
            if sx1 < x {
                continue;
            } else if sx0 < x {
                x = sx1 + 1;
                if x > 4_000_000{
                    break;
                } else {
                    continue;
                }
            } else {
                return x * 4_000_000 + y;
            }
        }
    }
    todo!()
}

pub fn main(parts: crate::RunPart) {
    use crate::dispatcher::*;
    DispatcherBuilder::setup(setup)
        .part1(part1)
        .part2(part2)
        // .run(TEST_INPUT, parts);
        .run(include_str!("../../../input/2022/15.txt"), parts);
}
