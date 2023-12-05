use std::cmp;

type Id = u64;

crate::aoc! {
    include_str!("../../../input/2023/05.txt"),
//     r"
// seeds: 79 14 55 13

// seed-to-soil map:
// 50 98 2
// 52 50 48

// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15

// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4

// water-to-light map:
// 88 18 7
// 18 25 70

// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13

// temperature-to-humidity map:
// 0 69 1
// 1 0 69

// humidity-to-location map:
// 60 56 37
// 56 93 4",
    |i| {
        let mut map_inputs = i.split("\n\n");
        let seeds = map_inputs.next().unwrap()[7..].trim().split(" ").map(|s| s.parse::<Id>().unwrap()).collect::<Vec<_>>();
        let mut maps: [Vec<(Id, Id, Id)>; 7] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for (i, input) in map_inputs.enumerate() {
            for line in input.split("\n").skip(1).filter(|l| !l.is_empty()) {
                let mut nums = line.split(" ").map(|s| s.parse::<Id>().unwrap());
                let dst_start = nums.next().unwrap();
                let src_start = nums.next().unwrap();
                let len = nums.next().unwrap();
                maps[i].push((dst_start, src_start, len));
            }
        }
        (seeds, maps)
    },
    |(seeds, maps)| seeds.iter().map(|seed| maps.iter().fold(*seed, |id, map|
        if let Some((dst_start, src_start, _)) = map.iter().find(|(_, src_start, len)| src_start <= &id && src_start + len > id) {
            dst_start + id - src_start
        } else {
            id
        })).reduce(|a, b| cmp::min(a, b)).unwrap(),
    |(seeds, maps)| 0,
}
