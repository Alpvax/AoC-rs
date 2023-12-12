use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, Default)]
struct Universe {
    width: usize,
    height: usize,
    empty_rows: Vec<usize>,
    nonempty_cols: HashSet<usize>,
    galaxies: Vec<(usize, usize)>,
}
impl Universe {
    fn expanded_galaxies(&self, increase: usize) -> Vec<(usize, usize)> {
        let col_increments = (0..self.width)
            .fold(
                (Vec::new(), 0usize),
                |(mut vec, mut cumulative_inc), col| {
                    if !self.nonempty_cols.contains(&col) {
                        cumulative_inc += increase - 1;
                    };
                    vec.push(cumulative_inc);
                    (vec, cumulative_inc)
                },
            )
            .0;
        let row_increments = (0..self.height)
            .fold(
                (Vec::new(), 0usize),
                |(mut vec, mut cumulative_inc), col| {
                    if self.empty_rows.contains(&col) {
                        cumulative_inc += increase - 1;
                    };
                    vec.push(cumulative_inc);
                    (vec, cumulative_inc)
                },
            )
            .0;
        self.galaxies
            .iter()
            .map(|(col, row)| (col + col_increments[*col], row + row_increments[*row]))
            .collect()
    }
}

crate::aoc! {
    include_str!("../../../input/2023/11.txt"),
//     r"
// ...#......
// .......#..
// #.........
// ..........
// ......#...
// .#........
// .........#
// ..........
// .......#..
// #...#.....",
    |i| i.split("\n").filter(|s| s.len() > 0).fold((Universe::default(), 0), |(mut universe, mut row), s| {
        if row >= universe.height {
            universe.height = row + 1;
        }
        if s.len() > universe.width {
            universe.width = s.len();
        }
        let mut empty = true;
        for (col, c) in s.chars().enumerate() {
            match c {
                '#' => {
                    universe.galaxies.push((col, row));
                    universe.nonempty_cols.insert(col);
                    empty = false;
                },
                _ => (),
            }
        }
        if empty {
            universe.empty_rows.push(row);
        }
        (universe, row + 1)
    }).0,
    |data| data.expanded_galaxies(2).iter().combinations(2).fold(0, |sum, items| {
        let a = items[0];
        let b = items[1];
        let x = if a.0 > b.0 {
            a.0 - b.0
        } else {
            b.0 - a.0
        };
        let y = if a.1 > b.1 {
            a.1 - b.1
        } else {
            b.1 - a.1
        };
        sum + x + y
    }),
    |data| data.expanded_galaxies(1_000_000).iter().combinations(2).fold(0, |sum, items| {
        let a = items[0];
        let b = items[1];
        let x = if a.0 > b.0 {
            a.0 - b.0
        } else {
            b.0 - a.0
        };
        let y = if a.1 > b.1 {
            a.1 - b.1
        } else {
            b.1 - a.1
        };
        sum + x + y
    }),
}
