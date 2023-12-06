use std::iter;

use regex::Regex;

crate::aoc! {
    include_str!("../../../input/2023/06.txt"),
//     r"
// Time:      7  15   30
// Distance:  9  40  200",
    |i| {
        let re = Regex::new(r"\s+").unwrap();
        let (time, record) = i.trim().split_once("\n").unwrap();
        iter::zip(
            re.split(time).skip(1).map(|s| s.parse::<u64>().unwrap()),
            re.split(record).skip(1).map(|s| s.parse::<u64>().unwrap()),
        ).map(|(time, record)| {
            let mut i = 1;
            while i * (time - i) <= record {
                i += 1;
            }
            time + 1 - i * 2
        }).reduce(|a, b| a * b).unwrap()
    },
    |i| {
        let (time, record) = i.trim().split_once("\n").unwrap();
        let time = time.split_once(":").unwrap().1.replace(" ", "").parse::<u64>().unwrap();
        let record = record.split_once(":").unwrap().1.replace(" ", "").parse::<u64>().unwrap();
        let mut i = 1;
        while i * (time - i) <= record {
            i += 1;
        }
        time + 1 - i * 2
    },
}
