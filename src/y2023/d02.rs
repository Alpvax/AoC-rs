use std::cmp;

use fancy_regex::Regex;

#[derive(Debug)]
struct Game {
    id: u16,
    max_r: u8,
    max_g: u8,
    max_b: u8,
}
impl Game {
    fn new(id: u16) -> Self {
        Self {
            id,
            max_r: 0,
            max_g: 0,
            max_b: 0,
        }
    }
    fn add_counter_view(&mut self, red: u8, green: u8, blue: u8) {
        self.max_r = cmp::max(self.max_r, red);
        self.max_g = cmp::max(self.max_g, green);
        self.max_b = cmp::max(self.max_b, blue);
    }
}

lazy_static::lazy_static! {
    static ref RED_RE: Regex = Regex::new(r"(\d+) red").unwrap();
    static ref GREEN_RE: Regex = Regex::new(r"(\d+) green").unwrap();
    static ref BLUE_RE: Regex = Regex::new(r"(\d+) blue").unwrap();
}

crate::aoc! {
    include_str!("../../../input/2023/02.txt"),
//     r"
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    |i| i.split("\n").filter(|s| s.len() > 0).map(|s| {
        let (gs, cs) = s.split_once(":").unwrap();
        let mut game = Game::new(gs[5..].parse().unwrap());
        for view in cs.split(";") {
            game.add_counter_view(
                RED_RE.captures(view).unwrap().map(|c| c.get(1).unwrap().as_str().parse().unwrap()).unwrap_or_default(),
                GREEN_RE.captures(view).unwrap().map(|c| c.get(1).unwrap().as_str().parse().unwrap()).unwrap_or_default(),
                BLUE_RE.captures(view).unwrap().map(|c| c.get(1).unwrap().as_str().parse().unwrap()).unwrap_or_default(),
            );
        }
        game
    }).collect::<Vec<_>>(),
    |data| data.iter().filter_map(|g| if g.max_r <= 12 && g.max_g <= 13 && g.max_b <= 14 { Some(g.id) } else { None }).sum::<u16>(),
    |data| data.iter().map(|g| u32::from(g.max_r) * u32::from(g.max_g) * u32::from(g.max_b)).sum::<u32>(),
}
