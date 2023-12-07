use std::{cmp, collections::HashMap};

macro_rules! make_cards {
    ($($name:ident = $c:literal),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum Card {
            $(
                $name,
            )+
        }
        impl TryFrom<char> for Card {
            type Error = char;
            fn try_from(val: char) -> Result<Self, Self::Error> {
                Ok(match val {
                    $($c => Self::$name,)+
                    c => Err(c)?,
                })
            }
        }
        impl TryFrom<&char> for Card {
            type Error = char;
            fn try_from(val: &char) -> Result<Self, Self::Error> {
                Ok(match val {
                    $(&$c => Self::$name,)+
                    c => Err(*c)?,
                })
            }
        }
        impl core::fmt::Display for Card {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(&Self::$name => $c,)+
                }.fmt(f)
            }
        }
    };
}

make_cards! {
    Two = '2',
    Three = '3',
    Four = '4',
    Five = '5',
    Six = '6',
    Seven = '7',
    Eight = '8',
    Nine = '9',
    Ten = 'T',
    Jack = 'J',
    Queen = 'Q',
    King = 'K',
    Ace = 'A',
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Score {
    HighCard,
    Pair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// enum Score {
//     HighCard(u8),
//     Pair(u8),
//     TwoPair(u8, u8),
//     ThreeKind(u8),
//     FullHouse(u8, u8),
//     FourKind(u8),
//     FiveKind(u8),
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand {
    score: Score,
    cards: [Card; 5],
    bid: u16,
}
impl Hand {
    fn part1_order(&self) -> (Score, Vec<u8>) {
        (
            self.score,
            self.cards
                .iter()
                .map(|c| match c {
                    Card::Two => 0,
                    Card::Three => 1,
                    Card::Four => 2,
                    Card::Five => 3,
                    Card::Six => 4,
                    Card::Seven => 5,
                    Card::Eight => 6,
                    Card::Nine => 7,
                    Card::Ten => 8,
                    Card::Jack => 9,
                    Card::Queen => 10,
                    Card::King => 11,
                    Card::Ace => 12,
                })
                .collect(),
        )
    }
    fn part2_order(&self) -> (Score, Vec<u8>) {
        (
            self.score,
            self.cards
                .iter()
                .map(|c| match c {
                    Card::Two => 1,
                    Card::Three => 2,
                    Card::Four => 3,
                    Card::Five => 4,
                    Card::Six => 5,
                    Card::Seven => 6,
                    Card::Eight => 7,
                    Card::Nine => 8,
                    Card::Ten => 9,
                    Card::Jack => 0,
                    Card::Queen => 10,
                    Card::King => 11,
                    Card::Ace => 12,
                })
                .collect(),
        )
    }
}

crate::aoc! {
    include_str!("../../../input/2023/07.txt"),
//     r"
// 32T3K 765
// T55J5 684
// KK677 28
// KTJJT 220
// QQQJA 483",
    |i| i.split("\n").filter(|s| s.len() > 0).map(|s| {
        let (cards, bid) = s.split_once(" ").unwrap();
        let mut cards = cards.chars().map(|c| Card::try_from(c).unwrap());
        let cards = [
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
        ];
        let bid = bid.parse::<u16>().unwrap();
        (cards, bid)
    }).collect::<Vec<_>>(),
    |data| {
        let mut hands = data.iter().map(|&(cards, bid)| {
            let map = cards.iter().fold(HashMap::<Card, u8>::new(), |mut map, card| {
                let count = map.entry(*card).or_default();
                *count += 1;
                map
            });
            Hand {
                score: match map.len() {
                    1 => Score::FiveKind,
                    2 => if map.values().any(|count| count == &4) { Score::FourKind } else { Score::FullHouse },
                    3 => if map.values().any(|count| count == &3) { Score::ThreeKind } else { Score::TwoPair },
                    4 => Score::Pair,
                    5 => Score::HighCard,
                    _ => unreachable!(),
                },
                cards,
                bid,
            }
        }).collect::<Vec<_>>();
        hands.sort_by_key(|h| h.part1_order());
        hands.into_iter().enumerate().map(|(i, hand)| (i + 1) * hand.bid as usize).sum::<usize>()
    },
    |data| {
        let mut hands = data.iter().map(|&(cards, bid)| {
            let map = cards.iter().fold(HashMap::<Card, u8>::new(), |mut map, card| {
                let count = map.entry(*card).or_default();
                *count += 1;
                map
            });
            let joker_count = match map.get(&Card::Jack) {
                Some(c) => *c,
                None => 0,
            };
            let mut non_j_counts = map.iter().filter_map(|(card, c)| if card == &Card::Jack { None } else { Some(*c) }).collect::<Vec<_>>();
            non_j_counts.sort_by_key(|c| cmp::Reverse(*c));
            let max_count = *non_j_counts.first().unwrap_or(&0);
            let score = match joker_count + max_count {
                5 => Score::FiveKind,
                4 => Score::FourKind,
                3 => {
                    if non_j_counts[1] == 2 {
                        Score::FullHouse
                    } else {
                        Score::ThreeKind
                    }
                },
                2 => if non_j_counts[1] == 2 {
                    Score::TwoPair
                } else {
                    Score::Pair
                },
                1 => Score::HighCard,
                count => unreachable!("{}", count),
            };
            Hand { score, cards, bid }
        }).collect::<Vec<_>>();
        hands.sort_by_key(|h| h.part2_order());
        hands.into_iter().enumerate().map(|(i, hand)| (i + 1) * hand.bid as usize).sum::<usize>()
    },
}
