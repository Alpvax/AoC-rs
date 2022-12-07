#![allow(dead_code)]

use std::{fmt::Display, time::Instant};

const EMPTY_STR: String = String::new();
macro_rules! format_setup_duration {
    ($s:literal, $d:expr) => {
        if Self::has_setup() {
            format!($s, $d)
        } else {
            EMPTY_STR
        }
    };
}

pub(crate) trait AocSolutions<'s, C, S1, S2 = &'static str>
where
    S1: Display,
    S2: Display,
{
    fn setup(&self, input: &'s str) -> C;
    fn has_setup() -> bool {
        true
    }
    fn part1(&self, input: &C) -> S1;
    fn part2(&self, input: C) -> S2;
    fn run(&self, input: &'s str, parts: crate::RunPart) {
        let now = Instant::now();
        let setup = self.setup(input);
        let d = now.elapsed();
        if Self::has_setup() {
            println!("Setup took {:?}", d)
        };
        match parts {
            crate::RunPart::All => {
                let i1 = Instant::now();
                let p1 = self.part1(&setup);
                let d1 = i1.elapsed();
                println!(
                    "Part 1: {}. Completed in {:?}{}",
                    p1,
                    d1,
                    format_setup_duration!(" ({:?} including setup)", d + d1),
                );
                let i2 = Instant::now();
                let p2 = self.part2(setup);
                let d2 = i2.elapsed();
                println!(
                    "Part 2: {}. Completed in {:?}{}",
                    p2,
                    d2,
                    format_setup_duration!(" ({:?} including setup)", d + d2),
                );
            }
            crate::RunPart::Part1 => {
                let i1 = Instant::now();
                let p1 = self.part1(&setup);
                let d1 = i1.elapsed();
                println!(
                    "Part 1: {}. Completed in {:?}{}",
                    p1,
                    d1,
                    format_setup_duration!(" ({:?} including setup)", d + d1),
                );
            }
            crate::RunPart::Part2 => {
                let i2 = Instant::now();
                let p2 = self.part2(setup);
                let d2 = i2.elapsed();
                println!(
                    "Part 2: {}. Completed in {:?}{}",
                    p2,
                    d2,
                    format_setup_duration!(" ({:?} including setup)", d + d2),
                );
            }
        }
    }
}

pub(crate) struct SolutionP1<'s, S1>(fn(&'s str) -> S1)
where
    S1: Display;
impl<'s, S1> SolutionP1<'s, S1>
where
    S1: Display,
{
    pub fn new(p1: fn(&'s str) -> S1) -> Self {
        Self(p1)
    }
}
impl<'s, S1> AocSolutions<'s, &'s str, S1> for SolutionP1<'s, S1>
where
    S1: Display,
{
    fn setup(&self, input: &'s str) -> &'s str {
        input
    }
    fn has_setup() -> bool {
        false
    }
    fn part1(&self, input: &&'s str) -> S1 {
        self.0(*input)
    }
    fn part2(&self, _input: &'s str) -> &'static str {
        "Not yet implemented"
    }
}
pub(crate) struct SolutionP1S<'s, C, S1>(fn(&'s str) -> C, fn(&C) -> S1)
where
    S1: Display;
impl<'s, C, S1> SolutionP1S<'s, C, S1>
where
    S1: Display,
{
    pub fn new(setup: fn(&'s str) -> C, p1: fn(&C) -> S1) -> Self {
        Self(setup, p1)
    }
}
impl<'s, C, S1> AocSolutions<'s, C, S1> for SolutionP1S<'s, C, S1>
where
    S1: Display,
{
    fn setup(&self, input: &'s str) -> C {
        self.0(input)
    }
    fn part1(&self, input: &C) -> S1 {
        self.1(input)
    }
    fn part2(&self, _input: C) -> &'static str {
        "Not yet implemented"
    }
}
pub(crate) struct SolutionP2<'s, S1, S2>(fn(&'s str) -> S1, fn(&'s str) -> S2)
where
    S1: Display,
    S2: Display;
impl<'s, S1, S2> SolutionP2<'s, S1, S2>
where
    S1: Display,
    S2: Display,
{
    pub fn new(p1: fn(&'s str) -> S1, p2: fn(&'s str) -> S2) -> Self {
        Self(p1, p2)
    }
}
impl<'s, S1, S2> AocSolutions<'s, &'s str, S1, S2> for SolutionP2<'s, S1, S2>
where
    S1: Display,
    S2: Display,
{
    fn setup(&self, input: &'s str) -> &'s str {
        input
    }
    fn has_setup() -> bool {
        false
    }
    fn part1(&self, input: &&'s str) -> S1 {
        self.0(*input)
    }
    fn part2(&self, input: &'s str) -> S2 {
        self.1(input)
    }
}
pub(crate) struct SolutionP2S<'s, C, S1, S2>(fn(&'s str) -> C, fn(&C) -> S1, fn(C) -> S2)
where
    S1: Display,
    S2: Display;
impl<'s, C, S1, S2> SolutionP2S<'s, C, S1, S2>
where
    S1: Display,
    S2: Display,
{
    pub fn new(setup: fn(&'s str) -> C, p1: fn(&C) -> S1, p2: fn(C) -> S2) -> Self {
        Self(setup, p1, p2)
    }
}
impl<'s, C, S1, S2> AocSolutions<'s, C, S1, S2> for SolutionP2S<'s, C, S1, S2>
where
    S1: Display,
    S2: Display,
{
    fn setup(&self, input: &'s str) -> C {
        self.0(input)
    }
    fn part1(&self, input: &C) -> S1 {
        self.1(input)
    }
    fn part2(&self, input: C) -> S2 {
        self.2(input)
    }
}
#[macro_export]
macro_rules! aoc {
    (
        $input:expr,
        $f1:expr $(,)?
    ) => {
        pub(super) fn main(parts: crate::RunPart) {
            use crate::AocSolutions;
            crate::SolutionP1::new($f1).run($input, parts);
        }
    };
    (
        $input:expr,
        setup: $setup:expr,
        part1: $f1:expr$(,)?
    ) => {
        pub(super) fn main(parts: crate::RunPart) {
            use crate::AocSolutions;
            crate::SolutionP1S::new($setup, $f1).run($input, parts);
        }
    };
    (
        $input:expr,
        $f1:expr,
        $f2:expr $(,)?
    ) => {
        pub(super) fn main(parts: crate::RunPart) {
            use crate::AocSolutions;
            crate::SolutionP2::new($f1, $f2).run($input, parts);
        }
    };
    ($input:expr, setup, part1, part2 $(,)?) => {
        pub(super) fn main(parts: crate::RunPart) {
            use crate::AocSolutions;
            crate::SolutionP2S::new(setup, part1, part2).run($input, parts);
        }
    };
    (
        $input:expr,
        $(setup:)? $setup:expr,
        $(part1:)? $f1:expr,
        $(part2:)? $f2:expr $(,)?
    ) => {
        pub(super) fn main(parts: crate::RunPart) {
            use crate::AocSolutions;
            crate::SolutionP2S::new($setup, $f1, $f2).run($input, parts);
        }
    };
}
