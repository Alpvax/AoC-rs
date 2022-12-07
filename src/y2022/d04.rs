#[allow(dead_code)]
const TEST_DATA: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

crate::aoc! {
    include_str!("../../../input/2022/04.txt"),
    |input| input
        .split("\n")
        .filter_map(|p| {
            if p.len() > 0 {
                let mut elves = p.split(",").map(|s| {
                    let (a, b) = s.split_once("-").unwrap();
                    a.parse::<u16>().unwrap()..=b.parse::<u16>().unwrap()
                });
                Some((elves.next().unwrap(), elves.next().unwrap()))
            } else {
                None
            }
        }),
    |pairs| pairs
        .clone()
        .filter(|pair| {
            if (pair.0.contains(pair.1.start()) && pair.0.contains(pair.1.end()))
                || (pair.1.contains(pair.0.start()) && pair.1.contains(pair.0.end()))
            {
                true
            } else {
                false
            }
        })
        .count(),
    |pairs| pairs
        .filter(|pair| {
            if pair.0.contains(pair.1.start())
                || pair.0.contains(pair.1.end())
                || pair.1.contains(pair.0.start())
                || pair.1.contains(pair.0.end())
            {
                true
            } else {
                false
            }
        })
        .count(),
}
