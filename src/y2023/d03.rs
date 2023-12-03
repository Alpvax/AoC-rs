use std::collections::HashMap;

#[derive(Debug)]
struct PotentialPart {
    id: u32,
    row: usize,
    min_col: usize,
    max_col: usize,
    is_part: bool,
}
impl PotentialPart {
    fn new(row: usize, start: usize, slice: &str) -> Self {
        Self {
            id: slice.parse().unwrap(),
            row,
            min_col: start,
            max_col: start + slice.len(),
            is_part: false,
        }
    }
    fn adjacent_positions(&self) -> impl Iterator<Item = (usize, usize)> {
        SurroundingPosIterator::new(
            self.row,
            if self.min_col > 0 {
                self.min_col - 1
            } else {
                0
            },
            self.max_col,
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum SurroundingPosIterator {
    Pre {
        row: usize,
        start: usize,
        end_incl: usize,
        x: usize,
    },
    Current {
        row: usize,
        start: usize,
        end_incl: usize,
        at_start: bool,
    },
    Post {
        row: usize,
        end_incl: usize,
        x: usize,
    },
    Complete,
}
impl SurroundingPosIterator {
    fn new(row: usize, start: usize, end_inclusive: usize) -> Self {
        if row > 0 {
            Self::Pre {
                row: row - 1,
                start,
                end_incl: end_inclusive,
                x: start,
            }
        } else {
            Self::Current {
                row,
                start,
                end_incl: end_inclusive,
                at_start: true,
            }
        }
    }
}
impl Iterator for SurroundingPosIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let mut res = None;
        let next_self = match self {
            Self::Pre {
                row,
                start,
                end_incl,
                x,
            } => {
                res = Some((*x, *row));
                *x += 1;
                if x > end_incl {
                    Some(Self::Current {
                        row: *row + 1,
                        start: *start,
                        end_incl: *end_incl,
                        at_start: true,
                    })
                } else {
                    None
                }
            }
            Self::Current {
                row,
                start,
                end_incl,
                at_start,
            } => {
                if *at_start {
                    res = Some((*start, *row));
                    *at_start = false;
                    None
                } else {
                    res = Some((*end_incl, *row));
                    Some(Self::Post {
                        row: *row + 1,
                        end_incl: *end_incl,
                        x: *start,
                    })
                }
            }
            Self::Post { row, end_incl, x } => {
                if x > end_incl {
                    res = None;
                    Some(Self::Complete)
                } else {
                    res = Some((*x, *row));
                    *x += 1;
                    None
                }
            }
            Self::Complete => None,
        };
        if let Some(s) = next_self {
            *self = s;
        }
        res
    }
}

crate::aoc! {
    include_str!("../../../input/2023/03.txt"),
//     r"
// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..",
    |i| {
        let (mut parts, symbols) = i.split("\n").filter(|s| s.len() > 0).enumerate().map(|(row,s)| {
            let mut chars = s.chars().enumerate();
            let mut parts = Vec::new();
            let mut symbols = HashMap::new();
            let mut digit_start = None;
            let mut digit_end = None;
            while let Some((col, c)) = chars.next() {
                if c.is_ascii_digit() {
                    if digit_start.is_none() {
                        digit_start = Some(col);
                    }
                    digit_end = Some(col);
                } else if digit_start.is_some() {
                    let start = digit_start.take().unwrap();
                    let end = digit_end.take().unwrap();
                    parts.push(PotentialPart::new(row, start, &s[start..=end]));
                }
                if c == '.' {
                    continue;
                } else if !c.is_ascii_digit() {
                    symbols.insert((col, row), c);
                }
            }
            if digit_start.is_some() {
                let start = digit_start.take().unwrap();
                let end = digit_end.take().unwrap();
                parts.push(PotentialPart::new(row, start, &s[start..=end]));
            }
            (parts, symbols)
        }).fold((Vec::new(), HashMap::new()), |(mut parts, mut symbols), (mut row_parts, row_symbols)| {
            parts.append(&mut row_parts);
            for (k, v) in row_symbols {
                symbols.insert(k, v);
            }
            (parts, symbols)
        });
        for part in &mut parts {
            if part.adjacent_positions().any(|pos| symbols.get(&pos).is_some()) {
                part.is_part = true
            }
            // println!("{part:?}");//XXX
        }
        (parts, symbols)
    },
    |(parts, symbols)| parts.iter().filter_map(|p| if p.adjacent_positions().any(|pos| symbols.get(&pos).is_some()) { Some(p.id) } else { None }).sum::<u32>(),
    |(parts, symbols)| symbols.iter().filter_map(|(pos, s)| if *s == '*' {
        let adjacent = parts.iter().filter_map(|p| if p.adjacent_positions().any(|ppos| pos == &ppos) { Some(p.id) } else { None }).collect::<Vec<_>>();
        if adjacent.len() == 2 {
            Some(adjacent[0] * adjacent[1])
        } else {
            None
        }
    } else { None }).sum::<u32>(),
}
