crate::aoc! {
    include_str!("../../../input/2022/01.txt"),
    |i| i.split("\n\n").map(|s| {
        s.split('\n')
            .map(|n| n.parse::<u32>().expect("Error parsing u32"))
            .sum()
    }).collect::<Vec<u32>>(),
    |data| *data.iter().max().unwrap(),
    |mut data| {
        data.sort_by(|a, b| b.cmp(a)); // Reverse order
        data[0..3].iter().sum::<u32>()
    }
}
