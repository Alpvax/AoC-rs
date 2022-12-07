use std::{
    fmt::Display,
    time::{Duration, Instant},
};

pub(crate) trait AocSolutions<'s, C, S1, S2>
where
    S1: Display,
    S2: Display,
{
    fn setup(&self, input: &'s str) -> C;
    fn clone_setup(setup: &C) -> C;
    fn part1(&self, input: C) -> S1;
    fn part2(&self, input: C) -> S2;
    fn run_p1(&self, setup: C) -> (S1, Duration) {
        let now = Instant::now();
        let res = self.part1(setup);
        let e = now.elapsed();
        (res, e)
    }
    fn run_p2(&self, setup: C) -> (S2, Duration) {
        let now = Instant::now();
        let res = self.part2(setup);
        let e = now.elapsed();
        (res, e)
    }
    fn run(&self, input: &'s str, parts: crate::RunPart) {
        let setup = self.setup(input);
        match parts {
            crate::RunPart::All => {
                let p1 = self.run_p1(Self::clone_setup(&setup));
                println!("Part 1: {}. Completed in {:?}", p1.0, p1.1);
                let p2 = self.run_p2(setup);
                println!("Part 2: {}. Completed in {:?}", p2.0, p2.1);
            }
            crate::RunPart::Part1 => {
                let p1 = self.run_p1(setup);
                println!("Part 1: {}. Completed in {:?}", p1.0, p1.1);
            }
            crate::RunPart::Part2 => {
                let p2 = self.run_p2(setup);
                println!("Part 2: {}. Completed in {:?}", p2.0, p2.1);
            }
        }
    }
}

pub(crate) struct SolutionP1<'s, S1>(fn(&'s str) -> S1)
where S1: Display;
impl<'s, S1> AocSolutions<'s, &'s str, S1, &'static str> for SolutionP1<'s, S1>
where
    S1: Display,
{
    fn setup(&self, input: &'s str) -> &'s str {
        input
    }
    fn clone_setup(setup: &&'s str) -> &'s str {
        setup
    }
    fn part1(&self, input: &'s str) -> S1 {
        self.0(input)
    }
    fn part2(&self, _input: &'s str) -> &'static str {
        "Not yet implemented"
    }
}
pub(crate) struct SolutionP1S<'s, C, S1>(fn(&'s str) -> C, fn(C) -> S1)
where
    S1: Display,
;
impl<'s, C, S1> AocSolutions<'s, C, S1, &'static str> for SolutionP1S<'s, C, S1>
where
    C: Clone,
    S1: Display,
{
    fn setup(&self, input: &'s str) -> C {
        self.0(input)
    }
    fn clone_setup(setup: &C) -> C {
        setup.clone()
    }
    fn part1(&self, input: C) -> S1 {
        self.1(input)
    }
    fn part2(&self, _input: C) -> &'static str {
        "Not yet implemented"
    }
}
pub(crate) struct SolutionP2<'s, S1, S2>(fn(&'s str) -> S1, fn(&'s str) -> S2)
where
    S1: Display,
    S2: Display,
;
impl<'s, S1, S2> AocSolutions<'s, &'s str, S1, S2> for SolutionP2<'s, S1, S2>
where
    S1: Display,
    S2: Display,
{
    fn setup(&self, input: &'s str) -> &'s str {
        input
    }
    fn clone_setup(setup: &&'s str) -> &'s str {
        setup
    }
    fn part1(&self, input: &'s str) -> S1 {
        self.0(input)
    }
    fn part2(&self, input: &'s str) -> S2 {
        self.1(input)
    }
}
pub(crate) struct SolutionP2S<'s, C, S1, S2>(fn(&'s str) -> C, fn(C) -> S1, fn(C) -> S2)
where
    S1: Display,
    S2: Display,
;
impl<'s, C, S1, S2> AocSolutions<'s, C, S1, S2> for SolutionP2S<'s, C, S1, S2>
where
    C: Clone,
    S1: Display,
    S2: Display,
{
    fn setup(&self, input: &'s str) -> C {
        self.0(input)
    }
    fn clone_setup(setup: &C) -> C {
        setup.clone()
    }
    fn part1(&self, input: C) -> S1 {
        self.1(input)
    }
    fn part2(&self, input: C) -> S2 {
        self.2(input)
    }
}
#[macro_export]
macro_rules! input_path {
    () => {
        if let Some(m) = ::regex::Regex::new(r"^(?:.*::)?(\d{4})_(d(?:ay)?)?(\d+)(?:::p(?:art)?(\d+))?$").unwrap().captures(module_path!()) {
            input_path!(@matched_all: m.get(1), m.get(2), m.get(3), m.get(4))
        } else {
            panic!("Module path: \"{}\" does not match format.", ::std::module_path!())
        }
    };
    ($part:ident) => {
        input_path!(stringify!($part))
    };
    ($meta:expr) => {
        if let Some(m) = ::regex::Regex::new(r"^(?:.*::)?(\d{4})_(d(?:ay)?)?(\d+)$").unwrap().captures(module_path!()) {
            input_path!(@meta: m.get(1).unwrap().as_str(), m.get(2), m.get(3).unwrap().as_str(), $meta)
        } else {
            panic!("Module path: \"{}\" does not match format.", ::std::module_path!())
        }
    };
    (@matched_all: $year:expr, $dayl:expr, $dayn:expr, $part:expr) => {
        concat!("input/", $year.unwrap().as_str(), if let Some(d) = $dayl {
            d.as_str()
        } else {
            "day"
        }, $dayn.unwrap().as_str(), if let Some(p) = $part {
            concat!("_", p.as_str())
        } else {
            "".to_string()
        }, ".txt")
    };
    (@meta: $year:expr, $dayl:expr, $dayn:expr$(, $part:expr $(,)?)?) => {
        format!("input/{}/{}{:02}{}.txt", $year, if let Some(d) = $dayl {
            d.as_str()
        } else {
            "day"
        }, $dayn, format!("_{}", $($part)?))
    };
}
#[macro_export]
macro_rules! input_data {
    ($($meta:tt)?) => {
        include_str!(concat!("../../../", input_path!($($meta)?)))
    };
}
#[macro_export]
macro_rules! aoc {
    (
        $f1:expr $(,)?
    ) => {
        pub(super) fn main(parts: crate::RunPart) {
            crate::SolutionP1($f1).run(crate::input_data!(), parts);
        }
    };
    (
        setup: $setup:expr,
        $f1:expr$(,)?
    ) => {
        pub(super) fn main(parts: crate::RunPart) {
            crate::SolutionP1S($setup, $f1).run(crate::input_data!(), parts);
        }
    };
    (
        $f1:expr,
        $f2:expr $(,)?
    ) => {
        pub(super) fn main(parts: crate::RunPart) {
            crate::SolutionP2($f1, $f2).run(crate::input_data!(), parts);
        }
    };
    (
        setup: $setup:expr,
        $f1:expr,
        $f2:expr $(,)?
    ) => {
        pub(super) fn main(parts: crate::RunPart) {
            crate::SolutionP2S($setup, $f1, $f2).run(crate::input_data!(), parts);
        }
    };
}
