#![allow(dead_code)]

use std::{
    fmt::Display,
    marker::PhantomData,
    time::{Duration, Instant},
};

pub trait SolutionPart {
    type Input<'s>; // = &'s str;
    type Answer: Display;
    fn run<'s>(&self, input: Self::Input<'s>) -> Self::Answer;
    fn run_bench<'s>(&self, input: Self::Input<'s>) -> (Self::Answer, Duration) {
        let now = Instant::now();
        let answer = self.run(input);
        let d = now.elapsed();
        (answer, d)
    }
}

pub trait Dispatcher<'s> {
    fn run(&self, input: &'s str, parts: crate::RunPart);
}
pub struct DispatcherSetup<'s, T> {
    setup: fn(&'s str) -> T,
}
impl<'s, T> DispatcherSetup<'s, T> {
    pub fn part1<F, U>(self, f: F) -> DispatcherP1S<'s, T, F, U>
    where
        F: Fn(&T) -> U,
        U: Display,
    {
        DispatcherP1S {
            setup: self.setup,
            part1: f,
        }
    }
}

pub struct DispatcherP1<'s, P1, R1>
where
    P1: Fn(&'s str) -> R1,
    R1: Display,
{
    part1: P1,
    _phantom: PhantomData<&'s R1>,
}
impl<'s, P1, R1> DispatcherP1<'s, P1, R1>
where
    P1: Fn(&'s str) -> R1,
    R1: Display,
{
    pub fn part2<P2, U>(self, f: P2) -> DispatcherP2<'s, P1, R1, P2, U>
    where
        P2: Fn(&'s str) -> U,
    {
        DispatcherP2 {
            part1: self.part1,
            part2: f,
            _phantom: PhantomData,
        }
    }
}
impl<'s, P1, R1> Dispatcher<'s> for DispatcherP1<'s, P1, R1>
where
    P1: Fn(&'s str) -> R1,
    R1: Display,
{
    fn run(&self, input: &'s str, parts: crate::RunPart) {
        if parts.run_p1() {
            let i1 = Instant::now();
            let p1 = (self.part1)(input);
            let d1 = i1.elapsed();
            println!("Part 1: {}. Completed in {:?}", p1, d1,);
        }
    }
}

pub struct DispatcherP1S<'s, T, P1, R1>
where
    P1: Fn(&T) -> R1,
    R1: Display,
{
    setup: fn(&'s str) -> T,
    part1: P1,
}
impl<'s, T, P1, R1> DispatcherP1S<'s, T, P1, R1>
where
    P1: Fn(&T) -> R1,
    R1: Display,
{
    pub fn part2<P2, U>(self, f: P2) -> DispatcherP2S<'s, T, P1, R1, P2, U>
    where
        P2: Fn(T) -> U,
        U: Display,
    {
        DispatcherP2S {
            setup: self.setup,
            part1: self.part1,
            part2: f,
        }
    }
}
impl<'s, T, P1, R1> Dispatcher<'s> for DispatcherP1S<'s, T, P1, R1>
where
    P1: Fn(&T) -> R1,
    R1: Display,
{
    fn run(&self, input: &'s str, parts: crate::RunPart) {
        if parts.run_p1() {
            let now = Instant::now();
            let s = (self.setup)(input);
            let d = now.elapsed();
            let i1 = Instant::now();
            let p1 = (self.part1)(&s);
            let d1 = i1.elapsed();
            println!(
                "Part 1: {}. Completed in {:?} ({:?} including setup)",
                p1,
                d1,
                d + d1,
            );
        }
    }
}

pub struct DispatcherP2<'s, P1, R1, P2, R2>
where
    P1: Fn(&'s str) -> R1,
    P2: Fn(&'s str) -> R2,
{
    part1: P1,
    part2: P2,
    _phantom: PhantomData<&'s (R1, R2)>,
}
impl<'s, P1, R1, P2, R2> Dispatcher<'s> for DispatcherP2<'s, P1, R1, P2, R2>
where
    P1: Fn(&'s str) -> R1,
    R1: Display,
    P2: Fn(&'s str) -> R2,
    R2: Display,
{
    fn run(&self, input: &'s str, parts: crate::RunPart) {
        match parts {
            crate::RunPart::All => {
                let i1 = Instant::now();
                let p1 = (self.part1)(input);
                let d1 = i1.elapsed();
                println!("Part 1: {}. Completed in {:?}", p1, d1,);
                let i2 = Instant::now();
                let p2 = (self.part2)(input);
                let d2 = i2.elapsed();
                println!("Part 2: {}. Completed in {:?}", p2, d2,);
            }
            crate::RunPart::Part1 => {
                let i1 = Instant::now();
                let p1 = (self.part1)(input);
                let d1 = i1.elapsed();
                println!("Part 1: {}. Completed in {:?}", p1, d1,);
            }
            crate::RunPart::Part2 => {
                let i2 = Instant::now();
                let p2 = (self.part2)(input);
                let d2 = i2.elapsed();
                println!("Part 2: {}. Completed in {:?}", p2, d2,);
            }
        }
    }
}

pub struct DispatcherP2S<'s, T, P1, R1, P2, R2>
where
    P1: Fn(&T) -> R1,
    R1: Display,
    P2: Fn(T) -> R2,
    R2: Display,
{
    setup: fn(&'s str) -> T,
    part1: P1,
    part2: P2,
}
impl<'s, T, P1, R1, P2, R2> Dispatcher<'s> for DispatcherP2S<'s, T, P1, R1, P2, R2>
where
    P1: Fn(&T) -> R1,
    R1: Display,
    P2: Fn(T) -> R2,
    R2: Display,
{
    fn run(&self, input: &'s str, parts: crate::RunPart) {
        let now = Instant::now();
        let s = (self.setup)(input);
        let d = now.elapsed();
        println!("Setup took {:?}", d);
        match parts {
            crate::RunPart::All => {
                let i1 = Instant::now();
                let p1 = (self.part1)(&s);
                let d1 = i1.elapsed();
                println!(
                    "Part 1: {}. Completed in {:?} ({:?} including setup)",
                    p1,
                    d1,
                    d + d1,
                );
                let i2 = Instant::now();
                let p2 = (self.part2)(s);
                let d2 = i2.elapsed();
                println!(
                    "Part 2: {}. Completed in {:?} ({:?} including setup)",
                    p2,
                    d2,
                    d + d2,
                );
            }
            crate::RunPart::Part1 => {
                let i1 = Instant::now();
                let p1 = (self.part1)(&s);
                let d1 = i1.elapsed();
                println!(
                    "Part 1: {}. Completed in {:?} ({:?} including setup)",
                    p1,
                    d1,
                    d + d1,
                );
            }
            crate::RunPart::Part2 => {
                let i2 = Instant::now();
                let p2 = (self.part2)(s);
                let d2 = i2.elapsed();
                println!(
                    "Part 2: {}. Completed in {:?} ({:?} including setup)",
                    p2,
                    d2,
                    d + d2,
                );
            }
        }
    }
}

pub struct DispatcherBuilder;
impl DispatcherBuilder {
    // pub fn with_inputs<'i, I>(self, inputs: I) -> Self where I: IntoIterator<Item = &'i str>, 's: 'i {
    //     todo!()
    // }
    pub fn setup<'s, T>(setup: fn(&'s str) -> T) -> DispatcherSetup<'s, T> {
        DispatcherSetup { setup }
    }
    pub fn part1<'s, F, T>(f: F) -> DispatcherP1<'s, F, T>
    where
        F: Fn(&'s str) -> T,
        T: Display,
    {
        DispatcherP1 {
            part1: f,
            _phantom: PhantomData,
        }
    }
}
