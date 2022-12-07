use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Rock,
    Paper,
    Scissors,
}
impl Action {
    fn beat(self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
    fn lose_to(self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}
impl FromStr for Action {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(s.to_string()),
        }
    }
}
impl core::ops::Add for Action {
    type Output = u32;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Action::Rock, Action::Paper)
            | (Action::Paper, Action::Scissors)
            | (Action::Scissors, Action::Rock) => RoundResult::Win + rhs,
            (Action::Rock, Action::Scissors)
            | (Action::Paper, Action::Rock)
            | (Action::Scissors, Action::Paper) => RoundResult::Lose + rhs,
            _ => RoundResult::Draw + rhs,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RoundResult {
    Win,
    Draw,
    Lose,
}
impl core::ops::Add<Action> for RoundResult {
    type Output = u32;

    fn add(self, rhs: Action) -> Self::Output {
        (match self {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Lose => 0,
        }) + match rhs {
            Action::Rock => 1,
            Action::Paper => 2,
            Action::Scissors => 3,
        }
    }
}
impl core::ops::Add<RoundResult> for Action {
    type Output = u32;

    fn add(self, rhs: RoundResult) -> Self::Output {
        rhs + match rhs {
            RoundResult::Win => self.beat(),
            RoundResult::Draw => self,
            RoundResult::Lose => self.lose_to(),
        }
    }
}

struct Round {
    a: Action,
    b: char,
}
impl Round {
    fn score1(&self) -> u32 {
        self.a
            + match self.b {
                'X' => Action::Rock,
                'Y' => Action::Paper,
                'Z' => Action::Scissors,
                c => panic!("Invalid character: '{}'", c),
            }
    }
    fn score2(&self) -> u32 {
        self.a
            + match self.b {
                'X' => RoundResult::Lose,
                'Y' => RoundResult::Draw,
                'Z' => RoundResult::Win,
                c => panic!("Invalid character: '{}'", c),
            }
    }
}
impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        Ok(Self {
            a: match chars.next() {
                Some('A') => Ok(Action::Rock),
                Some('B') => Ok(Action::Paper),
                Some('C') => Ok(Action::Scissors),
                _ => Err(s.to_string()),
            }?,
            b: chars.nth(1).unwrap(),
        })
    }
}

crate::aoc! {
    include_str!("../../../input/2022/02.txt"),
    |input| input.split("\n").map(|s| s.parse::<Round>()).filter_map(|r| r.ok()),
    |rounds| rounds.clone().fold(0, |t, r| t + r.score1()),
    |rounds| rounds.fold(0, |t, r| t + r.score2())
}
