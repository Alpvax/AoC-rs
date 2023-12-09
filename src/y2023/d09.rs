#[derive(Debug)]
struct Series {
    sequence: Vec<i64>,
    next: i64,
    prev: i64,
}
impl Series {
    fn new(sequence: Vec<i64>) -> Self {
        let first = *sequence.first().unwrap();
        let last = *sequence.last().unwrap();
        let mut interval_sequences = Vec::new();
        let mut curr_sequence = sequence.clone();
        let linear_inc = loop {
            let intervals = curr_sequence
                .iter()
                .fold((Vec::new(), None), |(mut vec, prev), &val| {
                    if let Some(prev) = prev {
                        vec.push(val - prev);
                    }
                    (vec, Some(val))
                })
                .0;
            match &intervals[..] {
                [head, tail @ ..] if tail.iter().all(|v| v == head) => break *head,
                [first_int, .., last_int] => {
                    interval_sequences.push((*first_int, *last_int));
                }
                _ => unreachable!("Empty sequence"),
            }
            curr_sequence = intervals;
        };
        let (start, end) = interval_sequences.into_iter().rev().fold(
            (linear_inc, linear_inc),
            |(start_inc, end_inc), (start, end)| (start - start_inc, end + end_inc),
        );
        Self {
            sequence,
            next: last + end,
            prev: first - start,
        }
    }
}

crate::aoc! {
    include_str!("../../../input/2023/09.txt"),
//     r"
// 0 3 6 9 12 15
// 1 3 6 10 15 21
// 10 13 16 21 30 45",
    |i| i.split("\n").filter(|s| s.len() > 0).map(|s| Series::new(s.split(" ").map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>())).collect::<Vec<_>>(),
    |data| data.iter().fold(0, |acc, s| acc + s.next),
    |data| data.iter().fold(0, |acc, s| acc + s.prev),
}
