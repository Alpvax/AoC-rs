use std::{cmp, collections::HashMap, str::FromStr};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Contents {
    #[default]
    Air,
    Rock,
    Sand,
}
impl Contents {
    fn is_solid(&self) -> bool {
        match self {
            Contents::Air => false,
            Contents::Rock | Contents::Sand => true,
        }
    }
}

type Point = (u16, u16);

#[derive(Clone)]
struct Grid {
    min_x: u16,
    max_x: u16,
    lowest: u16,
    solids: HashMap<Point, Contents>,
}
impl Grid {
    fn get(&self, point: Point) -> Contents {
        if point.1 == self.lowest + 2 {
            Contents::Rock
        } else {
            *self.solids.get(&point).unwrap_or(&Contents::Air)
        }
    }
    fn set(&mut self, point: Point, contents: Contents) {
        if contents.is_solid() {
            self.solids.insert(point, contents);
        } else {
            self.solids.remove(&point);
        }
    }
    /// Start sand drop at (500, 0)
    /// Return a option of the end position of the drop
    fn drop_sand(&mut self, with_floor: bool) -> Option<Point> {
        let mut x = 500;
        let mut y = 0;
        if self.solids.contains_key(&(x, y)) {
            return None;
        }
        while y <= self.lowest + if with_floor { 2 } else { 0 } {
            // Move down
            y += 1;
            if y == self.lowest + 2 {
                self.set((x, y - 1), Contents::Sand);
                return Some((x, y));
            }
            // If down is blocked
            if self.solids.contains_key(&(x, y)) {
                // If down left is blocked
                if self.solids.contains_key(&(x - 1, y)) {
                    // If down right is blocked
                    if self.solids.contains_key(&(x + 1, y)) {
                        self.set((x, y - 1), Contents::Sand);
                        return Some((x + 1, y));
                    } else {
                        // Move right
                        x += 1;
                    }
                } else {
                    // Move left
                    x -= 1;
                }
            }
        }
        None
    }
}
impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        type PointsIter = Box<dyn Iterator<Item = Point>>;
        let (rocks, min_x, max_x, lowest) = s
            .trim()
            .split("\n")
            .flat_map(|line| {
                let mut points = line.split(" -> ").map(|point| {
                    point
                        .split_once(",")
                        .and_then(|(x, y)| Some((x.parse::<u16>().ok()?, y.parse::<u16>().ok()?)))
                        .unwrap()
                });
                let s = points.next().unwrap();
                points
                    .fold(
                        (Box::new(std::iter::empty()) as PointsIter, s),
                        |(iter, s), e| {
                            (
                                if s.0 == e.0 {
                                    Box::new(
                                        iter.chain(
                                            (cmp::min(s.1, e.1)..=cmp::max(s.1, e.1))
                                                .map(move |y| (s.0, y)),
                                        ),
                                    )
                                } else {
                                    Box::new(
                                        iter.chain(
                                            (cmp::min(s.0, e.0)..=cmp::max(s.0, e.0))
                                                .map(move |x| (x, s.1)),
                                        ),
                                    )
                                },
                                e,
                            )
                        },
                    )
                    .0
            })
            .fold(
                (HashMap::new(), u16::MAX, u16::MIN, 0),
                |(mut map, x_min, x_max, lowest), (x, y)| {
                    map.insert((x, y), Contents::Rock);
                    (
                        map,
                        cmp::min(x, x_min),
                        cmp::max(x, x_max),
                        cmp::max(y, lowest),
                    )
                },
            );
        Ok(Self {
            min_x,
            max_x,
            lowest,
            solids: rocks,
        })
    }
}
impl core::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_x, max_x, lowest) = if f.alternate() {
            let l = self.lowest + 2;
            (500 - l, 500 + l, l)
        } else {
            (self.min_x, self.max_x, self.lowest)
        };
        for row in 0..=lowest {
            for col in min_x..=max_x {
                write!(f, "{}", {
                    let point = (col, row);
                    if point == (500, 0) {
                        '+'
                    } else {
                        match self.get(point) {
                            Contents::Air => '.',
                            Contents::Rock => '#',
                            Contents::Sand => 'o',
                        }
                    }
                })?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
const TEST_INPUT: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

pub fn main(parts: crate::RunPart) {
    use crate::dispatcher::*;
    DispatcherBuilder::setup(|input| input.parse::<Grid>().unwrap())
        .part1(|grid| {
            // println!("{}", grid);
            let mut grid = grid.clone();
            let mut count = 0;
            while grid.drop_sand(false).is_some() {
                // println!("Grid:\n{}", grid);
                count += 1;
            }
            count
        })
        .part2(|mut grid| {
            // println!("{:#}", grid);
            let mut count = 0;
            while grid.drop_sand(true).is_some() {
                // println!("Grid:\n{:#}", grid);
                count += 1;
            }
            count
        })
        // .run(TEST_INPUT, parts);
        .run(include_str!("../../../input/2022/14.txt"), parts);
}
