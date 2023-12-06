use std::cmp;

type Id = u64;

fn part1(seeds: &Vec<Id>, maps: &[Vec<(Id, Id, Id)>; 7]) -> Id {
    seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |id, map| {
                if let Some((dst_start, src_start, _)) = map
                    .iter()
                    .find(|(_, src_start, len)| src_start <= &id && src_start + len > id)
                {
                    dst_start + id - src_start
                } else {
                    id
                }
            })
        })
        .reduce(|a, b| cmp::min(a, b))
        .unwrap()
}

fn part2(seeds: Vec<Id>, maps: [Vec<(Id, Id, Id)>; 7]) -> Id {
    let mut ids: Vec<_> = seeds
        .chunks(2)
        .map(|pair| (pair[0], pair[0] + pair[1]))
        .collect();
    for map in maps {
        ids = ids
            .into_iter()
            .flat_map(|(start, end)| {
                let (remaining, processed) = map.iter().fold(
                    (vec![(start, end)], Vec::new()),
                    |(remaining, mut processed), &(dst_start, src_start, src_len)| {
                        let mut rem = Vec::new();
                        let mut any_proc = false;
                        dbg!((dst_start, src_start, src_len), &remaining);
                        for (start, end) in remaining.iter().cloned() {
                            if src_start > end || start > src_start + src_len {
                                continue;
                            }
                            any_proc = true;
                            let src_end = src_start + src_len;
                            let dst_end = dst_start + src_len;
                            match dbg!((start < src_start, end > src_end)) {
                                (true, true) => {
                                    processed.push((dst_start, dst_end));
                                    rem.push((start, src_start));
                                    rem.push((src_end, end));
                                }
                                (true, false) => {
                                    processed.push((dst_start, dst_start + src_start - start));
                                    rem.push((start, src_start));
                                }
                                (false, true) => {
                                    processed.push((dst_start + start - src_start, dst_end));
                                    rem.push((src_end, end));
                                }
                                (false, false) => {
                                    processed.push((
                                        dst_start + start - src_start,
                                        dst_start + end - src_start,
                                    ));
                                }
                            }
                        }
                        dbg!((if any_proc { rem } else { remaining }, processed))
                    },
                );
                dbg!(processed.into_iter().chain(remaining.into_iter()))
            })
            .collect();
    }
    ids.into_iter()
        .reduce(|(a, _), (b, _)| (cmp::min(a, b), 0))
        .unwrap()
        .0
}

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
    |(seeds, maps)| part1(seeds, maps),
    |(seeds, maps)| part2(seeds, maps),
}
