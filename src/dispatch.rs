use std::str::FromStr;

use ::chrono::{Datelike, Utc};
use clap::{builder::OsStr, command, value_parser, Arg};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunPart {
    All,
    Part1,
    Part2,
}
impl RunPart {
    pub fn run_p1(&self) -> bool {
        if let Self::Part2 = self {
            false
        } else {
            true
        }
    }
    pub fn run_p2(&self) -> bool {
        if let Self::Part1 = self {
            false
        } else {
            true
        }
    }
}
impl FromStr for RunPart {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('1') => Ok(Self::Part1),
            Some('2') => Ok(Self::Part2),
            _ => Err(format!("Invalid part number: {}", s)),
        }
    }
}

pub fn main() {
    let today = Utc::now().date_naive();
    let args = command!("aoc")
        .args([
            Arg::new("year")
                .required(today.month() != 12)
                .requires("day")
                .default_value(OsStr::from(today.format("%Y").to_string()))
                .value_parser(
                    value_parser!(u16).range(
                        2015..=if today.month() < 12 {
                            today.year() - 1
                        } else {
                            today.year()
                        }
                        .try_into()
                        .unwrap(),
                    ),
                ),
            Arg::new("day")
                .short('d')
                .long("day")
                .default_value(OsStr::from(today.format("%d").to_string()))
                .value_parser(value_parser!(u8).range(1..=25)),
            Arg::new("part")
                .short('p')
                .long("part")
                .value_parser(["1", "2"]),
        ])
        .get_matches();
    let &year = args.get_one::<u16>("year").unwrap();
    let &day = args.get_one::<u8>("day").unwrap();
    let parts = args
        .get_one::<String>("part")
        .map(|s| s.parse::<RunPart>().unwrap())
        .unwrap_or(RunPart::All);
    println!(
        "Running: year = {:?}, day = {:?}, parts = {:?}",
        year, day, parts
    );
    crate::run(year, day, parts);
}
