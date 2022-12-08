use std::str::Chars;

struct Forest<'s> {
    rows: Vec<&'s str>,
}
impl<'s> Forest<'s> {
    fn row(&self, index: usize) -> Chars<'s> {
        self.rows[index].chars()
    }
    fn col<'a>(&'a self, index: usize) -> impl Iterator<Item = char> + 'a
    where
        's: 'a,
    {
        self.rows
            .iter()
            .map(move |&r| r.chars().nth(index).expect("Error getting char from row"))
    }
}

fn setup(input: &str) -> Forest {
    Forest {
        rows: input.trim().split("\n").collect(),
    }
}

fn part1(forest: &Forest) -> u16 {
    let mut count = 0;
    for (r, &row) in forest.rows.iter().enumerate() {
        for (c, tree) in row.chars().enumerate() {
            if c == 0
                || c == row.len() - 1
                || r == 0
                || r == forest.rows.len() - 1
                || tree > row[0..c].chars().max().unwrap()
                || tree > row[c + 1..].chars().max().unwrap()
            {
                count += 1;
            } else {
                let mut col = forest.col(c);
                let mut larger = true;
                for _ in 0..r {
                    if tree < col.next().unwrap() {
                        larger = false;
                        break;
                    }
                }
                if larger || tree > col.skip(1).max().unwrap() {
                    count += 1;
                }
            }
        }
    }
    count
}

crate::aoc! {
// r"30373
// 25512
// 65332
// 33549
// 35390",
    include_str!("../../../input/2022/08.txt"),
    setup: setup,
    part1: part1,
}
