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

fn path_find(grid: &Grid, start: Point, end: char, valid_moves: fn(char, char) -> bool) -> u16 {
    let mut steps = 0;
    let mut processed = HashSet::new();
    let mut to_process = HashSet::new();
    to_process.insert(start);
    'a: while !to_process.is_empty() {
        let mut new = HashSet::new();
        for point in to_process.drain() {
            if grid.get(point) == end {
                break 'a;
            }
            if processed.contains(&point) {
                continue;
            }
            processed.insert(point);
            new.extend(
                available_moves(grid, point, valid_moves)
                    .into_iter()
                    .filter(|p| !processed.contains(p)),
            );
        }
        steps += 1;
        to_process = new;
    }
    steps
}

fn available_moves(grid: &Grid, (x, y): Point, valid: fn(char, char) -> bool) -> Vec<Point> {
    let c = grid.get((x, y));
    let mut v = Vec::new();
    let mut f = |p| {
        if valid(c, grid.get(p)) {
            v.push(p);
        }
    };
    if x > 0 {
        f((x - 1, y));
    }
    if x < grid.width - 1 {
        f((x + 1, y));
    }
    if y > 0 {
        f((x, y - 1));
    }
    if y < grid.height - 1 {
        f((x, y + 1));
    }
    v
}

fn part1(grid: &Grid) -> u16 {
    path_find(grid, grid.start, 'E', |c, pc| {
        c == 'S' || (pc == 'E' && c >= 'y') || (pc != 'E' && pc <= ((c as u8) + 1) as char)
    })
}

// Tecnically a path using S could be the shortest path (with length == part1),
// but I made the assumption that that would not be the case
fn part2(grid: Grid) -> u16 {
    path_find(&grid, grid.end, 'a', |c, pc| {
        if c == 'E' {
            pc >= 'y'
        } else {
            pc >= ((c as u8) - 1) as char
        }
    })
}

pub fn main(parts: crate::RunPart) {
    use crate::dispatcher::*;
    DispatcherBuilder::setup(load_grid)
        .part1(part1)
        .part2(part2)
        // .run(TEST_INPUT, parts);
        .run(include_str!("../../../input/2022/12.txt"), parts);
}
