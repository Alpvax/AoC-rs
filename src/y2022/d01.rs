pub fn main(parts: crate::RunPart) {
    let mut data: Vec<u32> = include_str!("../../../input/2022/01.txt")
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .map(|n| n.parse::<u32>().expect("Error parsing u32"))
                .sum()
        })
        .collect();
    if parts.run_p1() {
        println!("Part 1: {}", data.iter().max().unwrap());
    }
    if parts.run_p2() {
        data.sort_by(|a, b| b.cmp(a)); // Reverse order
        println!("Part 2: {}", data[0..3].iter().sum::<u32>());
    }
}
