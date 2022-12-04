fn priority(c: char) -> u8 {
    let ord = u8::try_from(c).unwrap();
    if ord >= 97 {
        // 97 = 'a'
        ord - 96
    } else {
        // 65 = 'A'
        ord - 38
    }
}

fn contents(s: &str) -> (&str, &str) {
    let l = s.len() / 2;
    (&s[0..l], &s[l..])
}

fn intersection<T: From<u8>>(s: &str) -> T {
    let (a, b) = contents(s);
    for c in a.chars() {
        if b.contains(c) {
            return priority(c).into();
        }
    }
    0.into()
}

fn intersection_group<T: From<u8>>(s1: &str, s2: &str, s3: &str) -> T {
    for c in s1.chars() {
        if s2.contains(c) && s3.contains(c) {
            return priority(c).into();
        }
    }
    0.into()
}

#[allow(dead_code)]
const TEST_INPUT: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

pub fn main(parts: crate::RunPart) {
    let data: u16 = include_str!("../../../input/2022/03.txt")
        .split("\n")
        .map(intersection::<u16>)
        .sum();
    if parts.run_p1() {
        println!("Part 1: {}", data);
    }
    if parts.run_p2() {
        let mut lines = include_str!("../../../input/2022/03.txt").split("\n");
        let mut total = 0u16;
        println!(
            "Part 2: {}",
            loop {
                let n = lines.next();
                if let Some(a) = n {
                    total +=
                        intersection_group::<u16>(a, lines.next().unwrap(), lines.next().unwrap());
                } else {
                    break total;
                }
            },
        );
    }
}
