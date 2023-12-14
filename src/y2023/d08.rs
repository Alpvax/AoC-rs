use std::collections::HashMap;

use fancy_regex::Regex;

crate::aoc! {
    include_str!("../../../input/2023/08.txt"),
//     r"
// Time:      7  15   30
// Distance:  9  40  200",
    |i| {
        let (instructions, tree) = i.trim().split_once("\n\n").unwrap();
        let re = Regex::new(r"(?P<parent>[A-Z]{3}) = \((?P<left>[A-Z]{3}), (?P<right>[A-Z]{3})\)").unwrap();
        let tree = tree.split("\n").filter_map(|s| if s.trim().len() > 0 {
            let caps = re.captures(s).unwrap().expect("Line does not match regex");
            let parent = caps.name("parent").unwrap().as_str();
            let left = caps.name("left").unwrap().as_str();
            let right = caps.name("right").unwrap().as_str();
            Some((parent, (left, right)))
        } else { None }).collect::<HashMap<_, _>>();
        (instructions, tree)
    },
    |(instructions, tree)| {
        let mut instructions = instructions.chars().cycle();
        let mut node = "AAA";
        let mut count = 0;
        while node != "ZZZ" {
            count += 1;
            let (left, right) = *tree.get(node).unwrap();
            node = match instructions.next().unwrap() {
                'L' => left,
                'R' => right,
                _ => unreachable!(),
            }
        }
        count
    },
    |(instructions, tree)| {
        let mut instructions = instructions.chars().cycle();
        let mut nodes = tree.keys().filter(|s| s.chars().last().unwrap() == 'A').cloned().collect::<Vec<_>>();
        let mut count = 0usize;
        while !nodes.iter().all(|s| s.chars().last().unwrap() == 'Z') {
            count += 1;
            let inst = instructions.next().unwrap();
            nodes = nodes.into_iter().map(|node| if inst == 'L' { tree.get(node).unwrap().0 } else { tree.get(node).unwrap().1 }).collect();
        }
        count
    },
}
