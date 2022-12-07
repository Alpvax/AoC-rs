use std::time::SystemTime;

use crate::RunPart;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Solutions<S1, S2> {
    All(S1, S2),
    Part1(S1),
    Part2(S2),
}

enum Solution<C, S1, S2> {
    Only1(fn() -> S1),
    Comm1(fn() -> C, fn(C) -> S1),
    All(Option<fn() -> C>, fn(C) -> S1, fn(C) -> S2),
}
impl<C, S1, S2> Solution<C, S1, S2> {
    fn part1(f: fn() -> S1) -> Self {
        Self::Only1(f)
    }
    fn part1_common(common: fn() -> C, f: fn(C) -> S1) -> Self {
        Self::Comm1(common, f)
    }
    fn part2(self, p2: fn(C) -> S2) -> Self {
        if let Self::Only1(p1) = self {
            Self::All(none, p1, p2)
        } else if let Self::Comm1(c, p1) = self {
            Self::All(Some(c), p1, p2)
        } else {
            panic!("Already 2 parts specified");
        }
    }
    fn common(&self) -> C {
        match self {
            Solution::Only1(_) => <() as C>,
            Solution::Comm1(c, _) => c(),
            Solution::All(c, _, _) => c.unwrap()(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem<C, S1, S2> {
    common: fn() -> C,
    p1: fn(C) -> S1,
    p2: Option<fn(C) -> S2>,
}
impl<C, S1, S2> Problem<C, S1, S2>
where C: Clone {
    pub fn run_parts(&self, parts: RunPart) -> Solutions<S1, S2> {
        let common = self.common();
        match parts {
            RunPart::All => Solutions::All(self.p1(common.clone()), ()),
            RunPart::Part1 => todo!(),
            RunPart::Part2 => todo!(),
        }
    }
    fn run_p1(&self, common: C) -> (S1, SystemTime) {
        let p1 = self.p1;
        let s = SystemTime::now();
        let res = p1(common);
        let e = SystemTime::now();
        (res, e - s)
    }
    fn run_p2(&self, common: C) -> (S1, SystemTime) {
        let p1 = self.p1;
        let s = SystemTime::now();
        let res = p1(common);
        let e = SystemTime::now();
        (res, e - s)
    }
}