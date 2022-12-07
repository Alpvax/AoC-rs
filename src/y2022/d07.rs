use std::collections::HashMap;

const ROOT: InodeName<'static> = InodeName(Vec::new());

#[derive(PartialEq, Eq, Hash)]
pub struct InodeName<'s>(Vec<&'s str>);
impl<'s> InodeName<'s> {
    pub fn parent(&self) -> Option<InodeName<'s>> {
        let l = self.0.len();
        if l > 1 {
            Some(self.0[0..l - 1].iter().collect())
        } else if l > 0 {
            Some(ROOT)
        } else {
            None
        }
    }
    /// Returns a new name, leaving this one unmodified
    pub fn with(&self, child: &'s str) -> InodeName<'s> {
        self.0
            .iter()
            .copied()
            .chain(core::iter::once(child))
            .collect()
    }
    /// Append child to the end of this path, modifying it
    pub fn push(&mut self, child: &'s str) {
        self.0.push(child);
    }
    /// Remove the end of this path, modifying it. No-op if the path is empty
    pub fn pop(&mut self) {
        self.0.pop();
    }
}
impl<'s> FromIterator<&'s str> for InodeName<'s> {
    fn from_iter<T: IntoIterator<Item = &'s str>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
impl<'s: 't, 't> FromIterator<&'t &'s str> for InodeName<'s> {
    fn from_iter<T: IntoIterator<Item = &'t &'s str>>(iter: T) -> Self {
        Self(iter.into_iter().copied().collect())
    }
}
impl<'s> core::fmt::Display for InodeName<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.len() > 0 {
            write!(
                f,
                "{}",
                self.0.iter().fold(String::new(), |mut acc, &s| {
                    acc.push('/');
                    acc.push_str(s);
                    acc
                })
            )
        } else {
            write!(f, "/")
        }
    }
}
impl<'s> core::fmt::Debug for InodeName<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("InodeName")
            .field(&format!("{}", self))
            .finish()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Inode {
    Dir,
    File(u32),
}
// impl Inode {
//     pub fn is_file(&self) -> bool {
//         match self {
//             Inode::Dir => false,
//             Inode::File(_) => true,
//         }
//     }
//     pub fn is_dir(&self) -> bool {
//         match self {
//             Inode::Dir => true,
//             Inode::File(_) => false,
//         }
//     }
// }

#[derive(Debug)]
enum CdArg<'s> {
    In(&'s str),
    Out,
    Root,
}
impl<'s> From<&'s str> for CdArg<'s> {
    fn from(s: &'s str) -> Self {
        match s {
            "/" => Self::Root,
            ".." => Self::Out,
            s => Self::In(s),
        }
    }
}
#[derive(Debug)]
enum LsItem<'s> {
    Dir(&'s str),
    File(&'s str, u32),
}
impl<'s> LsItem<'s> {
    // #[inline]
    // fn name(&self) -> &'s str {
    //     match self {
    //         LsItem::Dir(n) | LsItem::File(n, _) => *n,
    //     }
    // }
    // fn path(&self, parent: &InodeName<'s>) -> InodeName<'s> {
    //     parent.with(self.name())
    // }
    fn to_inode(self, parent: &InodeName<'s>) -> (InodeName<'s>, Inode) {
        match self {
            LsItem::Dir(n) => (parent.with(n), Inode::Dir),
            LsItem::File(n, s) => (parent.with(n), Inode::File(s)),
        }
    }
}
impl<'s> TryFrom<&'s str> for LsItem<'s> {
    type Error = String;

    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        s.split_once(" ")
            .ok_or(format!("Error parsing ls item: \"{}\"", s))
            .and_then(|(s, name)| {
                Ok(if s == "dir" {
                    Self::Dir(name)
                } else {
                    Self::File(
                        name,
                        s.parse()
                            .map_err(|e| format!("Error parsing file size: {:?}", e))?,
                    )
                })
            })
    }
}

#[derive(Debug)]
enum Command<'s> {
    Cd(CdArg<'s>),
    Ls(Vec<LsItem<'s>>),
}
impl<'s> TryFrom<&'s str> for Command<'s> {
    type Error = &'static str;

    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        if value.starts_with("cd") {
            value
                .split_whitespace()
                .nth(1)
                .map(|s| Self::Cd(s.into()))
                .ok_or("Error parsing cd arg")
        } else if value.starts_with("ls") {
            Ok(Self::Ls(
                value
                    .split("\n")
                    .skip(1)
                    .filter_map(|s| s.try_into().ok())
                    .collect(),
            ))
        } else {
            Err("Invalid command!")
        }
    }
}

struct FSParser<'s> {
    current_path: InodeName<'s>,
    files: HashMap<InodeName<'s>, Inode>,
}
impl<'s> FSParser<'s> {
    fn new() -> Self {
        Self {
            current_path: ROOT,
            files: HashMap::new(),
        }
    }
    fn cmd(&mut self, cmd: Command<'s>) {
        match cmd {
            Command::Cd(CdArg::In(child)) => self.current_path.push(child),
            Command::Cd(CdArg::Out) => self.current_path.pop(),
            Command::Cd(CdArg::Root) => self.current_path = ROOT,
            Command::Ls(contents) => {
                for item in contents {
                    let (path, inode) = item.to_inode(&self.current_path);
                    if self.files.contains_key(&path) {
                        eprintln!(
                            "Replacing inode \"{}\"!\n\tOld = {:?};\n\tNew = {:?};",
                            &path,
                            self.files.get(&path).unwrap(),
                            &inode
                        );
                    }
                    self.files.insert(path, inode);
                }
            }
        }
    }
    fn calc_sizes(&self) -> HashMap<InodeName<'s>, u32> {
        let mut map = HashMap::new();
        for (path, f_size) in self.files.iter().filter_map(|(p, n)| {
            if let Inode::File(s) = n {
                Some((p, s))
            } else {
                None
            }
        }) {
            let mut p = path.parent();
            while let Some(path) = p {
                p = path.parent();
                *map.entry(path).or_insert(0) += f_size;
            }
        }
        map
    }
}

#[allow(dead_code)]
const TEST_INPUT: &str = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

fn setup(input: &str) -> HashMap<InodeName, u32> {
    input
        .split("$ ")
        .filter_map(|s| Command::try_from(s).ok())
        .fold(FSParser::new(), |mut p, c| {
            p.cmd(c);
            p
        })
        .calc_sizes()
}
fn part1(dirs: &HashMap<InodeName, u32>) -> u32 {
    dirs.values().filter(|&s| s <= &100_000).sum::<u32>()
}
fn part2(dirs: HashMap<InodeName, u32>) -> u32 {
    let required = dirs.get(&ROOT).unwrap() - 40_000_000; // 70M (total) - 30M (required) = 40M (allowed)
    *dirs.values().filter(|&s| s >= &required).min().unwrap()
}
crate::aoc!(
    include_str!("../../../input/2022/07.txt"),
    setup,
    part1,
    part2
);
