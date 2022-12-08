use std::{iter::Rev, marker::PhantomData, str::Chars};

struct Directions<'s, 'f> {
    up: Box<dyn Iterator<Item = char> + 'f>,
    down: Box<dyn Iterator<Item = char> + 'f>,
    left: Rev<Chars<'s>>,
    right: Chars<'s>,
    _phantom: PhantomData<&'f Forest<'s>>,
}

struct Forest<'s> {
    rows: Vec<&'s str>,
}
impl<'s> Forest<'s> {
    fn directions<'a>(&'a self, row: usize, col: usize) -> Directions<'s, 'a> {
        let row_ = self.rows[row];
        let l = row_.len();
        Directions {
            up: Box::new(
                (0..row)
                    .rev()
                    .filter_map(move |r| self.rows.get(r).and_then(|&r| r.chars().nth(col))),
            ),
            down: if col >= l {
                Box::new(std::iter::empty())
            } else {
                Box::new(
                    (row + 1..l)
                        .filter_map(move |r| self.rows.get(r).and_then(|&r| r.chars().nth(col))),
                )
            },
            left: row_[0..col].chars().rev(),
            right: row_[col + 1..].chars(),
            _phantom: PhantomData,
        }
    }
}
impl core::fmt::Display for Forest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}
impl core::fmt::Debug for Forest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

fn setup(input: &str) -> Forest {
    Forest {
        rows: input.trim().split("\n").collect(),
    }
}

fn part1(forest: &Forest) -> u16 {
    let mut count = 0;
    for (r, &row) in forest.rows.iter().enumerate() {
        for (c, tree) in row.chars().enumerate() {
            if c == 0 || c == row.len() - 1 || r == 0 || r == forest.rows.len() - 1 || {
                let mut dirs = forest.directions(r, c);
                dirs.left.find(|&c| c >= tree).is_none()
                    || dirs.right.find(|&c| c >= tree).is_none()
                    || dirs.up.find(|&c| c >= tree).is_none()
                    || dirs.down.find(|&c| c >= tree).is_none()
            } {
                count += 1;
            }
        }
    }
    count
}

fn part2(forest: Forest) -> u32 {
    fn find_limit<I>(mut it: I, tree: char) -> u32
    where
        I: Iterator<Item = char>,
    {
        let mut i = 0;
        while let Some(c) = it.next() {
            i += 1;
            if c >= tree {
                break;
            }
        }
        i
    }
    let mut highest = 0;
    for (r, &row) in forest.rows.iter().enumerate() {
        for (c, tree) in row.chars().enumerate() {
            let dirs = forest.directions(r, c);
            let score = find_limit(dirs.left, tree)
                * find_limit(dirs.right, tree)
                * find_limit(dirs.up, tree)
                * find_limit(dirs.down, tree);
            if score > highest {
                highest = score;
            }
        }
    }
    highest
}

crate::aoc! {
// r"30373
// 25512
// 65332
// 33549
// 35390",
    include_str!("../../../input/2022/08.txt"),
    setup,
    part1,
    part2,
}
