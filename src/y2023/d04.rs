use std::collections::{HashMap, HashSet};

crate::aoc! {
    include_str!("../../../input/2023/04.txt"),
//     r"
// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    |i| i.split("\n").filter(|s| s.len() > 0).map(|s| {
        let (win_s, have_s) = s[10..].split_once("|").unwrap();
        let winners = win_s.trim().split(" ").filter_map(|i| if i.is_empty() { None } else { Some(i.trim().parse::<u32>().unwrap()) }).collect::<HashSet<_>>();
        let mut have = have_s.trim().split(" ").filter_map(|i| if i.is_empty() { None } else { Some(i.trim().parse::<u32>().unwrap()) }).collect::<HashSet<_>>();
        have.retain(|n| winners.contains(n));
        have.len()
    }).collect::<Vec<_>>(),
    |data| data.iter().map(|&matches| {
        if matches > 0 {
            1 << (matches - 1)
        } else {
            0
        }
    }
    ).sum::<u32>(),
    |data| data.iter().enumerate().fold((HashMap::<usize, u32>::new(), 0u32), |(mut map, mut total), (i, matches)| {
        let count = map.entry(i).or_default();
        *count += 1;
        let count = *count;
        for d_i in 1..=*matches {
            let c = map.entry(i + d_i).or_default();
            *c += count;
        }
        total += count;
        (map, total)
    }).1,
}
