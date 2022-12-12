use std::collections::HashSet;

type Point = (usize, usize);

#[allow(dead_code)]
const TEST_INPUT: &str = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

#[derive(Debug)]
struct Grid<'s> {
    heightmap: Vec<&'s str>,
    width: usize,
    height: usize,
    start: Point,
    end: Point,
}
impl<'s> Grid<'s> {
    fn get(&self, (x, y): Point) -> char {
        if y < self.heightmap.len() && x < self.heightmap[y].len() {
            self.heightmap[y].chars().nth(x).unwrap()
        } else {
            panic!(
                "Point ({}, {}) is outside the grid (width = {}, height = {})",
                x,
                y,
                self.heightmap[0].len(),
                self.heightmap.len()
            )
        }
    }
    fn available_moves(&self, (x, y): Point) -> Vec<Point> {
        let c = self.get((x, y));
        let mut v = Vec::new();
        let mut f = |p| {
            let pc = self.get(p);
            if c == 'S' || (pc == 'E' && c >= 'y') || (pc != 'E' && pc <= ((c as u8) + 1) as char) {
                v.push(p);
            }
        };
        if x > 0 {
            f((x - 1, y));
        }
        if x < self.width - 1 {
            f((x + 1, y));
        }
        if y > 0 {
            f((x, y - 1));
        }
        if y < self.height - 1 {
            f((x, y + 1));
        }
        v
    }
    fn is_end(&self, point: Point) -> bool {
        // self.get(point) == 'E'
        self.end == point
    }
}

fn load_grid(input: &str) -> Grid {
    let mut start = None;
    let mut end = None;
    let heightmap = input
        .split_whitespace()
        .enumerate()
        .fold(Vec::new(), |mut h, (y, line)| {
            if start.is_none() || end.is_none() {
                for (x, c) in line
                    .chars()
                    .enumerate()
                    .filter(|&(_, c)| c == 'S' || c == 'E')
                {
                    if c == 'S' {
                        start = Some((x, y));
                    } else if c == 'E' {
                        end = Some((x, y));
                    }
                }
            }
            h.push(line);
            h
        });
    let width = heightmap[0].len();
    let height = heightmap.len();
    Grid {
        heightmap,
        width,
        height,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

fn part1(grid: &Grid) -> u16 {
    let mut steps = 0;
    let mut processed = HashSet::new();
    let mut to_process = HashSet::new();
    to_process.insert(grid.start);
    'a: while !to_process.is_empty() {
        let mut new = HashSet::new();
        for point in to_process.drain() {
            if grid.is_end(point) {
                break 'a;
            }
            if processed.contains(&point) {
                continue;
            }
            processed.insert(point);
            new.extend(
                grid.available_moves(point)
                    .into_iter()
                    .filter(|p| !processed.contains(p)),
            );
        }
        steps += 1;
        to_process = new;
    }
    steps
}

pub fn main(parts: crate::RunPart) {
    use crate::dispatcher::*;
    DispatcherBuilder::setup(load_grid)
        .part1(part1)
        // .run(TEST_INPUT, parts);
        .run(include_str!("../../../input/2022/12.txt"), parts);
}
