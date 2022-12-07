use std::time::{SystemTime, Duration};

trait AocSolutions<'s, C, S1, S2> {
    fn common(&self, input: &'s str) -> C;
    fn clone_common(common: &C) -> C;
    fn part1(&self, input: C) -> S1;
    fn part2(&self, input: C) -> S2;
    fn run_p1(&self, common: C) -> (S1, Duration) {
        let s = SystemTime::now();
        let res = self.part1(common);
        let e = SystemTime::now();
        (res, e - s)
    }
    fn run_p2(&self, common: C) -> (S2, Duration) {
        let s = SystemTime::now();
        let res = self.part2(common);
        let e = SystemTime::now();
        (res, e - s)
    }
    fn run(&self, input: &'s str, parts: crate::RunPart) {
        let common = self.common(input);
        match parts {
            crate::RunPart::All => {
                let p1 = self.run_p1(Self::clone_common(&common));
                println!("Part 1: {}. Completed in {}", p1.0, p1.1);
                let p2 = self.run_p2(common);
                println!("Part 2: {}. Completed in {}", p2.0, p2.1);
            },
            crate::RunPart::Part1 => {
                let p1 = self.run_p1(common);
                println!("Part 1: {}. Completed in {}", p1.0, p1.1);
            },
            crate::RunPart::Part2 => {
                let p2 = self.run_p2(common);
                println!("Part 2: {}. Completed in {}", p2.0, p2.1);
            },
        }
    }
}

pub(crate) struct SolutionP1<'s, S1>(fn(&'s str) -> S1);
impl<'s, S1> AocSolutions<'s, &'s str, S1, ()> for SolutionP1<'s, S1> {
    fn common(&self, input: &'s str) -> &'s str {
        input
    }
    fn clone_common(common: &&'s str) -> &'s str {
        common.clone()
    }
    fn part1(&self, input: &'s str) -> S1 {
        self.0(input)
    }
    fn part2(&self, input: &'s str) -> &'static str {
        "Not yet implemented"
    }
}
pub(crate) struct SolutionP1C<'s, C, S1>(fn(&'s str) -> C, fn(C) -> S1);
impl<'s, C, S1> AocSolutions<'s, C, S1, ()> for SolutionP1C<'s, C, S1> {
    fn common(&self, input: &'s str) -> C {
        self.0(input)
    }
    fn clone_common(common: &C) -> C {
        common.clone()
    }
    fn part1(&self, input: C) -> S1 {
        self.1(input)
    }
    fn part2(&self, input: C) -> &'static str {
        "Not yet implemented"
    }
}
pub(crate) struct SolutionP2<'s, S1, S2>(fn(&'s str) -> S1, fn(&'s str) -> S2);
impl<'s, S1, S2> AocSolutions<'s, &'s str, S1, S2> for SolutionP2<'s, S1, S2> {
    fn common(&self, input: &'s str) -> &'s str {
        input
    }
    fn clone_common(common: &&'s str) -> &'s str {
        common.clone()
    }
    fn part1(&self, input: &'s str) -> S1 {
        self.0(input)
    }
    fn part2(&self, input: &'s str) -> S2 {
        self.1(input)
    }
}
pub(crate) struct SolutionP2C<'s, C, S1, S2>(fn(&'s str) -> C, fn(C) -> S1, fn(C) -> S2);
impl<'s, C, S1, S2> AocSolutions<'s, C, S1, S2> for SolutionP2C<'s, C, S1, S2> {
    fn common(&self, input: &'s str) -> C {
        self.0(input)
    }
    fn clone_common(common: &C) -> C {
        common.clone()
    }
    fn part1(&self, input: C) -> S1 {
        self.1(input)
    }
    fn part2(&self, input: C) -> S2 {
        self.2(input)
    }
}


#[macro_export]
macro_rules! aoc {
    // (
    //     common: $common:expr,
    //     ($c1:ident) => $p1:expr,
    //     ($c2:ident) => $p2:expr $(,)?
    // ) => {};
    // (
    //     common: $common:expr,
    //     $f1:expr,
    //     $f2:expr $(,)?
    // ) => {
    //     struct SolutionFactory<C, S1, S2> {
    //         common: fn(&'static str) -> C,
    //         p1: fn(C) -> S1,
    //         p2: fn(C) -> S2,
    //     }
    //     pub(super) fn main(parts: crate::RunPart) {
    //         let sol = SolutionFactory {
    //             common: $common,
    //             p1: $f1,
    //             p2: $f2,
    //         }
    //     }
    // };
    // (
    //     ($s1:ident) => $p1:expr,
    //     ($s2:ident) => $p2:expr $(,)?
    // ) => {};
    // (
    //     $f1:expr,
    //     $f2:expr $(,)?
    // ) => {
    //     struct SolutionFactory<C, S1, S2> {
    //         p1: fn(&'static str) -> S1,
    //         p2: fn(&'static str) -> S2,
    //     }
    //     pub(super) fn main(parts: crate::RunPart) {
    //         let sol = SolutionFactory {
    //             p1: |$s1| $f1,
    //             p2: |$s2| $f2,
    //         }
    //     }
    // };
    (
        $f1:expr $(,)?
    ) => {
        
    };
    (
        common: $common:expr,
        $f1:expr$(,)?
    ) => {};
    (
        $f1:expr,
        $f2:expr $(,)?
    ) => {};
    (
        common: $common:expr,
        $f1:expr,
        $f2:expr $(,)?
    ) => {};
}