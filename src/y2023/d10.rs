use std::collections::{HashMap, HashSet};

/// (0,0) is at top left
#[derive(Debug, Default)]
struct Grid {
    width: usize,
    height: usize,
    map: HashMap<(usize, usize), Pipe>,
    start: Option<(usize, usize)>,
}
impl Grid {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pipe {
    /// | is a vertical pipe connecting north and south.
    Vert,
    /// - is a horizontal pipe connecting east and west.
    Horz,
    /// L is a 90-degree bend connecting north and east.
    NE,
    /// J is a 90-degree bend connecting north and west.
    NW,
    /// 7 is a 90-degree bend connecting south and west.
    SE,
    /// F is a 90-degree bend connecting south and east.
    SW,
    /// . is ground; there is no pipe in this tile.
    None,
    /// S is the starting position of the animal; there is a pipe on this
    Start,
}
impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vert,
            '-' => Self::Horz,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::None,
            'S' => Self::Start,
            _ => unreachable!("invalid character"),
        }
    }
}
impl Pipe {
    // fn directions(&self) -> Option<[Direction; 2]> {
    //     use Direction::*;
    //     match self {
    //         Pipe::Vert => Some([North, South]),
    //         Pipe::Horz => Some([East, West]),
    //         Pipe::NE => Some([North, East]),
    //         Pipe::NW => Some([North, West]),
    //         Pipe::SE => Some([South, East]),
    //         Pipe::SW => Some([South, West]),
    //         Pipe::None => None,
    //         Pipe::Start => None,
    //     }
    // }
    fn connect(&self, direction: Direction) -> bool {
        match (self, direction) {
            (Self::Start, _)
            | (Self::Vert | Self::NE | Self::NW, Direction::North)
            | (Self::Vert | Self::SE | Self::SW, Direction::South)
            | (Self::Horz | Self::NE | Self::SE, Direction::East)
            | (Self::Horz | Self::NW | Self::SW, Direction::West) => true,
            _ => false,
        }
    }
    fn next_dir(&self, from_direction: Direction) -> Direction {
        use Direction::*;
        match (self, from_direction) {
            (Self::Vert, South) | (Self::NE, East) | (Self::NW, West) => North,
            (Self::Vert, North) | (Self::SE, East) | (Self::SW, West) => South,
            (Self::Horz, West) | (Self::NE, North) | (Self::SE, South) => East,
            (Self::Horz, East) | (Self::NW, North) | (Self::SW, South) => West,
            _ => panic!(
                "Attempted to enter a pipe from an invalid direction: {:?}, {:?}",
                self, from_direction
            ),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
    fn offset(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Self::North => (x, y - 1),
            Self::South => (x, y + 1),
            Self::East => (x + 1, y),
            Self::West => (x - 1, y),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PipeInstance {
    /// The end pos of the flow
    pos: (usize, usize),
    /// The direction of flow. The position the flow started was the offset position
    from_direction: Direction,
    pipe: Pipe,
}
impl PipeInstance {
    fn new_from(
        start_pos: (usize, usize),
        to_direction: Direction,
        pipe: Option<Pipe>,
    ) -> Option<Self> {
        let from_direction = to_direction.opposite();
        pipe.map(|pipe| Self {
            pos: to_direction.offset(start_pos),
            from_direction,
            pipe,
        })
    }
    fn lookup(grid: &Grid, start_pos: (usize, usize), to_direction: Direction) -> Option<Self> {
        use Direction::*;
        match (start_pos, to_direction) {
            ((0, _), West) | ((_, 0), North) => None,
            ((x, _), East) if x >= grid.width - 1 => None,
            ((_, y), East) if y >= grid.height - 1 => None,
            _ => {
                let pos = to_direction.offset(start_pos);
                grid.map.get(&pos).and_then(|&pipe| {
                    let from_direction = to_direction.opposite();
                    if pipe.connect(from_direction) {
                        Some(Self {
                            pos,
                            from_direction,
                            pipe,
                        })
                    } else {
                        None
                    }
                })
            }
        }
    }
    fn next_pipe(&self, grid: &Grid) -> Option<PipeInstance> {
        Self::lookup(grid, self.pos, self.pipe.next_dir(self.from_direction))
    }
}

crate::aoc! {
    include_str!("../../../input/2023/10.txt"),
//     r"
// -L|F7
// 7S-7|
// L|7||
// -L-J|
// L|-JF",
//     r"
// 7-F7-
// .FJ|7
// SJLL7
// |F--J
// LJ.LJ",
    |i| i.split("\n").filter(|s| s.len() > 0).enumerate().fold(Grid::default(), |mut grid, (y, s)| {
        if y >= grid.height {
            grid.height = y + 1;
        }
        s.chars().enumerate().for_each(|(x, c)| {
            if x >= grid.width {
                grid.width = x + 1;
            }
            match c {
                'S' => grid.start = Some((x, y)),
                '.' => (),
                _ => {
                    grid.map.insert((x, y), c.into());
                }
            }
        });
        grid
    }),
    |grid| {
        use Direction::*;
        let start_pos = dbg!(grid.start.unwrap());
        let mut to_process = HashSet::new();
        if let Some(p) = PipeInstance::lookup(grid, start_pos, North) {
            to_process.insert(p);
        }
        if let Some(p) = PipeInstance::lookup(grid, start_pos, South) {
            to_process.insert(p);
        }
        if let Some(p) = PipeInstance::lookup(grid, start_pos, East) {
            to_process.insert(p);
        }
        if let Some(p) = PipeInstance::lookup(grid, start_pos, West) {
            to_process.insert(p);
        }
        // if y > 0 {
        //     if let Some(p) = grid.map.get(&(x, y - 1)) {
        //         if p.connect(North) {
        //             to_process.insert(PipeInstance::flow_from((x, y), North));
        //         }
        //     }
        // }
        // if y < grid.height - 1 {
        //     if let Some(p) = grid.map.get(&(x, y + 1)) {
        //         if p.connect(North) {
        //             to_process.insert(PipeInstance::flow_from((x, y), South));
        //         }
        //     }
        // }
        // if x > 0 {
        //     if let Some(p) = grid.map.get(&(x - 1, y)) {
        //         if p.connect(East) {
        //             to_process.insert(PipeInstance::flow_from((x, y), West));
        //         }
        //     }
        // }
        // if x < grid.width - 1 {
        //     if let Some(p) = grid.map.get(&(x + 1, y)) {
        //         if p.connect(West) {
        //             to_process.insert(PipeInstance::flow_from((x, y), East));
        //         }
        //     }
        // }
        let mut computed = HashMap::new();
        computed.insert(start_pos, 0);
        let mut dist = 0;
        while to_process.len() > 0 {
            dist += 1;
            // println!("Processing pipes at distance {dist} from ({start_pos:?}): {to_process:?}");//XXX
            let mut new_pipes = HashSet::new();
            for pipe in to_process.into_iter() {
                // println!("Processing pipe: {pipe:?}");//XXX
                computed.insert(pipe.pos, dist);
                if let Some(p) = pipe.next_pipe(grid) {
                    if !computed.contains_key(&p.pos) {
                        new_pipes.insert(p);
                    }
                }
            }
            to_process = new_pipes;
        }
        dist
    },
    |grid| 0,
}
