use std::{cmp::Ordering, str::FromStr};

use serde::Deserialize;

type V = u8;

#[derive(Clone, Eq, Deserialize)]
#[serde(untagged)]
enum Value {
    Num(V),
    List(Vec<Self>),
}
impl Value {
    fn values(&self) -> Vec<Self> {
        match self {
            Value::Num(n) => vec![Value::Num(*n)],
            Value::List(v) => v.iter().cloned().collect(),
        }
    }
}
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        if let (Self::Num(a), Self::Num(b)) = (self, other) {
            a == b
        } else {
            let mut a = self.values().into_iter();
            let mut b = other.values().into_iter();
            loop {
                match (a.next(), b.next()) {
                    (None, None) => break true,
                    (None, Some(_)) | (Some(_), None) => break false,
                    (Some(na), Some(nb)) => {
                        if na != nb {
                            break false;
                        }
                    }
                }
            }
        }
    }
}
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        if let (Self::Num(a), Self::Num(b)) = (self, other) {
            a.cmp(b)
        } else {
            let mut a = self.values().into_iter();
            let mut b = other.values().into_iter();
            loop {
                match (a.next(), b.next()) {
                    (None, None) => break Ordering::Equal,
                    (None, Some(_)) => break Ordering::Less,
                    (Some(_), None) => break Ordering::Greater,
                    (Some(na), Some(nb)) => {
                        if na != nb {
                            break na.cmp(&nb);
                        }
                    }
                }
            }
        }
    }
}
impl FromStr for Value {
    type Err = ::serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::serde_json::from_str(s)
    }
}
impl core::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Num(n) => n.fmt(f),
            Value::List(vals) => {
                let l = vals.len();
                if l < 1 {
                    write!(f, "[]")
                } else {
                    write!(f, "[")?;
                    for (i, v) in vals.iter().enumerate() {
                        v.fmt(f)?;
                        if i < l - 1 {
                            write!(f, ", ")?;
                        }
                    }
                    write!(f, "]")
                }
            }
        }
    }
}

#[allow(dead_code)]
const TEST_INPUT: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

pub fn main(parts: crate::RunPart) {
    use crate::dispatcher::*;
    DispatcherBuilder::setup(|input| {
        input.split("\n\n").filter_map(|lines| {
            lines
                .split_once("\n")
                .and_then(|(a, b)| Some((a.parse::<Value>().ok()?, b.parse::<Value>().ok()?)))
        })
    })
    .part1(|pairs| {
        pairs
            .clone()
            .enumerate()
            .filter_map(|(i, (a, b))| if a <= b { Some(i + 1) } else { None })
            .sum::<usize>()
    })
    .part2(|pairs| {
        let div1: Value = "[[2]]".parse().unwrap();
        let div2: Value = "[[6]]".parse().unwrap();
        let mut packets = pairs
            .flat_map(|(a, b)| [a, b])
            .chain([div1.clone(), div2.clone()])
            .collect::<Vec<_>>();
        packets.sort();
        packets
            .into_iter()
            .enumerate()
            .filter_map(|(i, v)| {
                if v == div1 || v == div2 {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .product::<usize>()
    })
    // .run(TEST_INPUT, parts);
    .run(include_str!("../../../input/2022/13.txt"), parts);
}
