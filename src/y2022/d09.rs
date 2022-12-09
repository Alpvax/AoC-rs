use std::{collections::HashSet, num::ParseIntError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Vert {
    Up,
    Equal,
    Down,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Horz {
    Left,
    Equal,
    Right,
}
impl core::ops::Add<Vert> for Horz {
    type Output = Direction8;

    fn add(self, rhs: Vert) -> Self::Output {
        match (self, rhs) {
            (Horz::Equal, Vert::Up) => Direction8::U,
            (Horz::Right, Vert::Up) => Direction8::UR,
            (Horz::Right, Vert::Equal) => Direction8::R,
            (Horz::Right, Vert::Down) => Direction8::DR,
            (Horz::Equal, Vert::Down) => Direction8::D,
            (Horz::Left, Vert::Down) => Direction8::DL,
            (Horz::Left, Vert::Equal) => Direction8::L,
            (Horz::Left, Vert::Up) => Direction8::UL,
            (Horz::Equal, Vert::Equal) => Direction8::Equal,
        }
    }
}
impl core::ops::Add<Horz> for Vert {
    type Output = Direction8;

    fn add(self, rhs: Horz) -> Self::Output {
        rhs + self
    }
}
impl core::ops::Add for Vert {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Up, Self::Down) | (Self::Down, Self::Up) => Self::Equal,
            (Self::Equal, _) => rhs,
            _ => self,
        }
    }
}
impl core::ops::Add for Horz {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Left, Self::Right) | (Self::Right, Self::Left) => Self::Equal,
            (Self::Equal, _) => rhs,
            _ => self,
        }
    }
}

// Clockwise from Up
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction8 {
    U,
    UR,
    R,
    DR,
    D,
    DL,
    L,
    UL,
    Equal,
}
impl Direction8 {
    fn x(&self) -> Horz {
        match self {
            Self::U | Self::D | Self::Equal => Horz::Equal,
            Self::UR | Self::R | Self::DR => Horz::Right,
            Self::UL | Self::L | Self::DL => Horz::Left,
        }
    }
    fn y(&self) -> Vert {
        match self {
            Self::L | Self::R | Self::Equal => Vert::Equal,
            Self::UL | Self::U | Self::UR => Vert::Up,
            Self::DL | Self::D | Self::DR => Vert::Down,
        }
    }
    fn offset(&self, (x, y): Point) -> Point {
        (
            x + match self.x() {
                Horz::Left => -1,
                Horz::Equal => 0,
                Horz::Right => 1,
            },
            y + match self.y() {
                Vert::Up => -1,
                Vert::Equal => 0,
                Vert::Down => 1,
            },
        )
    }
    // fn to_xy(&self) -> (i8, i8) {
    //     (self.x(), self.y())
    // }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
}
#[derive(Debug)]
enum MoveErr {
    Parse(ParseIntError),
    Format,
    Direction(char),
}
impl std::str::FromStr for Move {
    type Err = MoveErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(" ").ok_or(MoveErr::Format).and_then(|(s, n)| {
            let d = n.parse().map_err(MoveErr::Parse)?;
            Ok(match s.chars().next().unwrap() {
                'U' => Move::Up(d),
                'D' => Move::Down(d),
                'L' => Move::Left(d),
                'R' => Move::Right(d),
                c => Err(MoveErr::Direction(c))?,
            })
        })
    }
}

// #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
type Point = (i16, i16);
// impl core::fmt::Display for Point {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "(")?;
//         self.0.fmt(f)?;
//         write!(f, ", ")?;
//         self.1.fmt(f)?;
//         write!(f, ")")
//     }
// }
// impl core::fmt::Debug for Point {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Point(")?;
//         self.0.fmt(f)?;
//         write!(f, ", ")?;
//         self.1.fmt(f)?;
//         write!(f, ")")
//     }
// }
// impl Default for Point {
//     fn default() -> Self {
//         ZERO
//     }
// }
const ZERO: Point = (0, 0);

#[derive(Debug)]
struct Bridge {
    head: Point,
    tail_dir: Direction8,
    tail_history: HashSet<Point>,
    x_min: i16,
    x_max: i16,
    y_min: i16,
    y_max: i16,
}
impl Bridge {
    fn new() -> Self {
        Self {
            head: ZERO,
            tail_dir: Direction8::Equal,
            tail_history: std::iter::once(ZERO).collect(),
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0,
        }
    }
    fn set_head(&mut self, x: i16, y: i16) {
        use core::cmp::{max, min};
        self.head = (x, y);
        self.x_min = min(x, self.x_min);
        self.x_max = max(x, self.x_max);
        self.y_min = min(y, self.y_min);
        self.y_max = max(y, self.y_max);
    }
    fn do_move(&mut self, move_: Move) {
        let (x, y) = self.head;
        match move_ {
            Move::Up(dist) => {
                let d = dist.into();
                self.set_head(x, y - d);
                for i in (match self.tail_dir.y() {
                    Vert::Up => 2,
                    Vert::Equal => 1,
                    Vert::Down => 0,
                })..d
                {
                    self.tail_history.insert((x, y - i));
                }
            }
            Move::Down(dist) => {
                let d = dist.into();
                self.set_head(x, y + d);
                for i in (match self.tail_dir.y() {
                    Vert::Up => 0,
                    Vert::Equal => 1,
                    Vert::Down => 2,
                })..d
                {
                    self.tail_history.insert((x, y + i));
                }
            }
            Move::Left(dist) => {
                let d = dist.into();
                self.set_head(x - d, y);
                for i in (match self.tail_dir.x() {
                    Horz::Left => 2,
                    Horz::Equal => 1,
                    Horz::Right => 0,
                })..d
                {
                    self.tail_history.insert((x - i, y));
                }
            }
            Move::Right(dist) => {
                let d = dist.into();
                self.set_head(x + d, y);
                for i in dbg!((match dbg!(self.tail_dir.x()) {
                    Horz::Left => 0,
                    Horz::Equal => 1,
                    Horz::Right => 2,
                })..d)
                {
                    self.tail_history.insert((x + i, y));
                }
            }
        }
    }
}
impl core::ops::AddAssign<Move> for Bridge {
    fn add_assign(&mut self, rhs: Move) {
        self.do_move(rhs);
    }
}
impl core::fmt::Display for Bridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.y_min..=self.y_max {
            writeln!(
                f,
                "{}",
                (self.x_min..=self.x_max)
                    .map(|x| {
                        if x == 0 && y == 0 {
                            's'
                        } else if (x, y) == self.head {
                            'H'
                        } else if (x, y) == self.tail_dir.offset(self.head) {
                            'T'
                        } else if self.tail_history.contains(&(x, y)) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
const TEST_INPUT: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

pub fn main(parts: crate::RunPart) {
    use crate::dispatcher::*;
    DispatcherBuilder::setup(|input| input.split("\n").filter_map(|s| s.parse::<Move>().ok()))
        .part1(|moves| {
            let bridge = moves.clone().fold(Bridge::new(), |mut b, m| {
                println!("{}\n{:?} =>", b, m); //XXX
                b.do_move(m);
                b
            });
            println!("{}", bridge); //XXX
            bridge.tail_history.len()
        })
        .run(TEST_INPUT, parts);
}
