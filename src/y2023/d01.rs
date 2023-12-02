use logos::{Lexer, Logos};

macro_rules! make_enum {
    ($($v:ident = $n:literal, $ns:literal, $fwd:literal, $rev:literal),+ $(,)?) => {
        paste::paste!{
            #[derive(Logos, Debug, PartialEq, Eq, PartialOrd, Ord)]
            enum Number {
                $(
                    #[token($ns)]
                    [<$v:camel S>],
                    #[token($fwd)]
                    [<$v:camel Fwd>],
                    #[token($rev)]
                    [<$v:camel Rev>],
                )+
                #[regex(".", |_| logos::Skip)]
                Junk
            }
            impl Number {
                fn is_short(&self) -> bool {
                    match self {
                        $(Self::[<$v:camel S>])|+ => true,
                        _ => false,
                    }
                }
            }
            impl From<Number> for u32 {
                fn from(value: Number) -> Self {
                    match value {
                        $(
                            Number::[<$v:camel S>] | Number::[<$v:camel Fwd>] | Number::[<$v:camel Rev>] => $n,
                        )+
                        Number::Junk => unreachable!("Junk should be skipped"),
                    }
                }
            }
        }
    };
}
make_enum! {
    one = 1, "1", "one", "eno",
    two = 2, "2", "two", "owt",
    three = 3, "3", "three", "eerht",
    four = 4, "4", "four", "ruof",
    five = 5, "5", "five", "evif",
    six = 6, "6", "six", "xis",
    seven = 7, "7", "seven", "neves",
    eight = 8, "8", "eight", "thgie",
    nine = 9, "9", "nine", "enin",
}

enum Num {
    /// digit value
    Short(u32),
    /// word value, digit value
    Long(u32, u32),
}
impl Num {
    fn short(&self) -> u32 {
        match self {
            Num::Short(n) | Num::Long(_, n) => *n,
        }
    }
    fn value(&self) -> u32 {
        match self {
            Num::Short(n) | Num::Long(n, _) => *n,
        }
    }
}

crate::aoc! {
    include_str!("../../../input/2023/01.txt"),
    |i| i.split("\n").filter(|s| s.len() > 0).map(|s| {
        fn find<'s>(mut lex: Lexer<'s, Number>) -> Num {
            let mut short: Option<u32> = None;
            let mut long: Option<u32> = None;
            while let Some(Ok(num)) = lex.next() {
                if num.is_short() {
                    short = Some(num.into());
                    break;
                } else if long.is_none() {
                    long = Some(num.into());
                }
            }
            if let Some(long) = long {
                Num::Long(long, short.unwrap())
            } else {
                Num::Short(short.unwrap())
            }
        }
        (find(Number::lexer(s)), find(Number::lexer(&s.chars().rev().collect::<String>())))
    }).collect::<Vec<_>>(),
    |data| data.iter().map(|(f, l)| f.short() * 10 + l.short()).sum::<u32>(),
    |data| data.iter().map(|(f, l)| f.value() * 10 + l.value()).sum::<u32>(),
}
