use std::str::FromStr;

use regex::Regex;

#[allow(dead_code)]
const TEST_INPUT: &str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[derive(Debug)]
struct Stacks<const N: usize>([Vec<char>; N]);
impl<const N: usize> FromStr for Stacks<N> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const EMPTY_VEC: Vec<char> = Vec::new();
        let mut arr = [EMPTY_VEC; N];
        fn push_crate<const N: usize>(c: char, arr: &mut [Vec<char>; N], i: usize) {
            match c {
                ' ' => (),
                _ => arr[i].push(c),
            }
        }
        for line in s.rsplit("\n").skip(1) {
            let mut chars = line.chars();
            push_crate(chars.nth(1).unwrap_or(' '), &mut arr, 0);
            for i in 1..=N {
                push_crate(chars.nth(3).unwrap_or(' '), &mut arr, i);
            }
        }
        Ok(Self(arr))
    }
}
impl<const N: usize> Stacks<N> {
    fn index(i: usize) -> usize {
        if i < 1 || i > N {
            panic!("Out of range: {} (1-{})", i, N);
        } else {
            i - 1
        }
    }
    fn move_crate(&mut self, from: usize, to: usize) {
        let c = self.0[Self::index(from)]
            .pop()
            .expect("Attempted to move crate from an empty stack");
        self.0[Self::index(to)].push(c);
    }
    fn move_crates(&mut self, inst: &Instruction) {
        for _ in 0..inst.count {
            self.move_crate(inst.from, inst.to);
        }
    }
    fn move_crates_simultaneously(&mut self, inst: &Instruction) {
        let mut c = Vec::new();
        for _ in 0..inst.count {
            c.push(
                self.0[Self::index(inst.from)]
                    .pop()
                    .expect("Attempted to move crate from an empty stack"),
            );
        }
        self.0[Self::index(inst.to)].extend(c.into_iter().rev());
    }
    fn top_crates(&self) -> String {
        self.0.iter().map(|v| v.last().unwrap_or(&' ')).collect()
    }
}

lazy_static::lazy_static! {
    static ref INST_RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}
impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = INST_RE.captures(s).ok_or(())?;
        Ok(Self {
            count: caps.get(1).unwrap().as_str().parse().map_err(|_| ())?,
            from: caps.get(2).unwrap().as_str().parse().map_err(|_| ())?,
            to: caps.get(3).unwrap().as_str().parse().map_err(|_| ())?,
        })
    }
}

pub fn main(parts: crate::RunPart) {
    let (stacks, moves) = include_str!("../../../input/2022/05.txt")
        .split_once("\n\n")
        .unwrap();
    let mut stacks: Stacks<9> = stacks.parse().unwrap();
    let moves = moves
        .split("\n")
        .filter_map(|s| s.parse::<Instruction>().ok())
        .collect::<Vec<_>>();
    if parts.run_p1() {
        for inst in &moves {
            stacks.move_crates(inst);
        }
        println!("Part 1: {}", stacks.top_crates());
    }
    if parts.run_p2() {
        for inst in &moves {
            stacks.move_crates_simultaneously(inst);
        }
        println!("Part 2: {}", stacks.top_crates());
    }
}
