use std::{collections::VecDeque, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Op {
    Add(u32),
    Mul(u32),
    Sqr,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u32>,
    op: Op,
    divisor: u32,
    true_index: usize,
    false_index: usize,
}
#[derive(Debug)]
enum MonkeyParseError {
    Int(std::num::ParseIntError, String),
    Empty,
    MissingItems(String),
    MissingOp(String),
    BadOp(String),
    MissingTest(String),
    MissingTrue(String),
    MissingFalse(String),
}
impl FromStr for Monkey {
    type Err = MonkeyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MonkeyParseError as E;
        let mut iter = s.split("\n").map(|s| s.trim());
        let mut line = iter.next().ok_or(E::Empty)?;
        if line.starts_with("Monkey") {
            line = iter.next().ok_or(E::Empty)?;
        }
        let items = if line.starts_with("Starting items: ") {
            line[16..].split(", ").map(|i| i.parse().unwrap()).collect()
        } else {
            Err(E::MissingItems(line.to_string()))?
        };
        line = iter.next().ok_or(E::MissingOp(line.to_string()))?;
        let op = if line.starts_with("Operation: new = old ") {
            match line[21..].split_whitespace().collect::<Vec<_>>()[0..2] {
                ["*", "old"] => Op::Sqr,
                ["+", b] => Op::Add(b.parse::<u32>().map_err(|e| E::Int(e, line.to_string()))?),
                ["*", b] => Op::Mul(b.parse::<u32>().map_err(|e| E::Int(e, line.to_string()))?),
                _ => Err(E::BadOp(line.to_string()))?,
            }
        } else {
            Err(E::MissingOp(line.to_string()))?
        };
        line = iter.next().ok_or(E::MissingTest(line.to_string()))?;
        let divisor = if line.starts_with("Test: divisible by ") {
            line[19..]
                .parse()
                .map_err(|e| E::Int(e, line.to_string()))?
        } else {
            Err(E::MissingTest(line.to_string()))?
        };
        line = iter.next().ok_or(E::MissingTrue(line.to_string()))?;
        let true_index = if line.starts_with("If true: throw to monkey ") {
            line[25..]
                .parse()
                .map_err(|e| E::Int(e, line.to_string()))?
        } else {
            Err(E::MissingTrue(line.to_string()))?
        };
        line = iter.next().ok_or(E::MissingFalse(line.to_string()))?;
        let false_index = if line.starts_with("If false: throw to monkey ") {
            line[26..]
                .parse()
                .map_err(|e| E::Int(e, line.to_string()))?
        } else {
            Err(E::MissingFalse(line.to_string()))?
        };
        Ok(Self {
            items,
            op,
            divisor,
            true_index,
            false_index,
        })
    }
}

struct Monkeys {
    items: Vec<VecDeque<u32>>,
    monkeys: Vec<Monkey>,
}
impl FromIterator<Monkey> for Monkeys {
    fn from_iter<T: IntoIterator<Item = Monkey>>(iter: T) -> Self {
        let (items, monkeys) = iter.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut items, mut monkeys), mut m| {
                items.push(m.items);
                m.items = VecDeque::new();
                monkeys.push(m);
                (items, monkeys)
            },
        );
        Self { items, monkeys }
    }
}

#[allow(dead_code)]
const TEST_INPUT: &str = r"Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 1";

fn run_monkeys(input: &str, num: usize, decrease_worry: bool) -> u32 {
    let m = input
        .split("\n\n")
        .filter_map(|s| s.parse::<Monkey>().ok())
        .collect::<Monkeys>();
    let mut monkeys: Vec<Monkey> = m.monkeys;
    let mut items = m.items;
    let l = monkeys.len();

    let mut counts = Vec::new();
    for _ in 0..l {
        counts.push(0);
    }

    for _ in 0..num {
        for i in 0..l {
            let monkey = &mut monkeys[i];
            while let Some(item) = items[i].pop_front() {
                let mut worry = match monkey.op {
                    Op::Add(x) => x + item,
                    Op::Mul(x) => x * item,
                    Op::Sqr => item * item,
                };
                if decrease_worry {
                    worry /= 3;
                }
                if worry % monkey.divisor == 0 {
                    items[monkey.true_index].push_back(worry);
                } else {
                    items[monkey.false_index].push_back(worry);
                }
                counts[i] += 1;
            }
        }
    }
    let top_2 = counts.into_iter().fold((0, 0), |(a, b), c| {
        if c > a {
            (c, a)
        } else if c > b {
            (a, c)
        } else {
            (a, b)
        }
    });
    top_2.0 * top_2.1
}

pub fn main(parts: crate::RunPart) {
    use crate::dispatcher::*;
    DispatcherBuilder::part1(|input| run_monkeys(input, 20, true))
        .part2(|input| run_monkeys(input, 10000, false))
        // .run(TEST_INPUT, parts);
        .run(include_str!("../../../input/2022/11.txt"), parts);
}
