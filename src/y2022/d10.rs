#[allow(dead_code)]
const TEST_INPUT: &str = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

fn display<I>(mut it: I) -> String
where
    I: Iterator<Item = (i16, i16)>,
{
    let mut display = String::new();
    let mut row_str = String::new();
    let mut sprite_pos = 1;
    // let display_sprite = |pos: i16| {
    //     let s = if pos > 1 {
    //         usize::try_from(pos).unwrap() - 1
    //     } else {
    //         0
    //     };
    //     let e = if pos < 0 {
    //         1
    //     } else if pos < 39 {
    //         usize::try_from(pos).unwrap() + 1
    //     } else {
    //         40
    //     };
    //     let mut line = ".".repeat(40);
    //     line.replace_range(s..e, &"#".repeat(e - s));
    //     println!("Sprite position: {}", line);
    // };
    // display_sprite(sprite_pos);
    let mut current = it.next().unwrap();
    let mut n = 0;
    for cycle in 0..240 {
        let x = cycle % 40;
        if x == 0 {
            display.push_str(&row_str);
            row_str = "\n".to_string();
        }
        row_str.push(if x >= sprite_pos - 1 && x <= sprite_pos + 1 {
            '#'
        } else {
            ' '
        });
        // println!(
        //     "During cycle  {}: CRT draws pixel in position {}\nCurrent CRT row: {}",
        //     cycle, x, row_str
        // );
        n += 1;
        if n >= current.0 {
            sprite_pos += current.1;
            // display_sprite(sprite_pos);
            current = it.next().unwrap_or((0, 0));
            n = 0;
        }
    }
    display.push_str(&row_str);
    display.push('\n');
    display
}

pub fn main(parts: crate::RunPart) {
    use crate::dispatcher::*;
    DispatcherBuilder::setup(|input| {
        input.split("\n").filter_map(|s| {
            if s.len() < 4 {
                None
            } else {
                match &s[0..4] {
                    "addx" => Some((2, (&s[5..]).parse().unwrap())),
                    "noop" => Some((1, 0i16)),
                    _ => None,
                }
            }
        })
    })
    .part1(|input| {
        let mut cycles = 0;
        let mut x = 1i16;
        let mut target = 20i16;
        let mut answer = 0;
        for (c, dx) in input.clone() {
            let cyc = cycles + c;
            if cyc >= target {
                answer += x * target;
                if target == 220 {
                    break;
                } else {
                    target += 40;
                }
            }
            cycles = cyc;
            x += dx;
        }
        answer
    })
    .part2(|input| display(input))
    // .run(TEST_INPUT, parts);
    .run(include_str!("../../../input/2022/10.txt"), parts);
}
