use std::str::FromStr;

#[derive(Debug)]
enum Cd<'s> {
    In(&'s str),
    Out,
    Root,
}
impl<'s> From<&'s str> for Cd<'s> {
    fn from(s: &'s str) -> Self {
        match s {
            "/" => Self::Root,
            ".." => Self::Out,
            s => Self::In(s),
        }
    }
}

#[derive(Debug)]
enum Command<'s> {
    Cd(Cd<'s>),
    Ls(Vec<Inode<'s>>),
}
// impl FromStr for Command<'_> {
//     type Err = String;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let s = match s.strip_prefix("$ ") {
//             Some(s) => s,
//             None => s,
//         };

//     }
// }

#[derive(Debug)]
enum Inode<'s> {
    Dir(&'s str, Option<Vec<Inode<'s>>>),
    File(&'s str, u32),
}
impl<'s> Inode<'s> {
    fn name(&self) -> &'s str {
        match self {
            Inode::Dir(n, _) | Inode::File(n, _) => n,
        }
    }
    fn size(&self) -> u32 {
        match self {
            Inode::Dir(_, None) => 0,
            Inode::Dir(_, Some(nodes)) => nodes.iter().map(|n| n.size()).sum(),
            Inode::File(_, s) => *s,
        }
    }
    fn child(&self, name: &'s str) -> Option<&Inode<'s>> {
        if let Self::Dir(_, Some(children)) = self {
            children.iter().find(|n| n.name() == name)
        } else {
            None
        }
    }
    fn child_mut(&mut self, name: &'s str) -> Option<&mut Inode<'s>> {
        if let Self::Dir(_, Some(children)) = self {
            for node in children {
                if node.name() == name {
                    return Some(node);
                }
            }
        }
        None
    }
}
impl std::fmt::Display for Inode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pad = f.width().unwrap_or(0) * 2;
        let fmt_name = format!("{: >pad$} {}", "-", self.name());
        match self {
            Inode::Dir(_, None) => write!(f, "{}\n{: >pad_child$}\n", fmt_name, "Unknown", pad_child = pad + 2),
            Inode::Dir(_, Some(nodes)) => {
                let pad = pad + 2;
                writeln!(f, "{}", fmt_name)?;
                for child in nodes {
                    writeln!(f, "{: >pad$}", child, pad = pad)?;
                }
                Ok(())
            },
            Inode::File(_, s) => write!(f, "{} (size={})", fmt_name, s),
        }
    }
}

struct PartialTree<'s> {
    root: Inode<'s>,
    path: Vec<&'s str>,
}
impl<'s> PartialTree<'s> {
    fn new() -> Self {
        Self {
            root: Inode::Dir("/", None),
            path: Vec::new(),
        }
    }
    fn cd<P>(&mut self, arg: P)
    where
        P: Into<Cd<'s>>,
    {
        match P::into(arg) {
            Cd::In(p) => self.path.push(p),
            Cd::Out => {
                self.path.pop();
            }
            Cd::Root => self.path.clear(),
        }
    }
    fn ls(&mut self, contents: Vec<Inode<'s>>) -> Result<(), String> {
        match self.inode_mut(self.path.iter().map(|s| *s)) {
            Ok(Inode::Dir(_, c)) => {
                if c.is_some() {
                    Err(format!(
                        "Directory \"/{}\" has already been listed",
                        self.path.join("/")
                    ))
                } else {
                    *c = Some(contents);
                    Ok(())
                }
            }
            Ok(_) => unreachable!("Current path does not point to a directory"),
            Err(s) => Err(s),
        }
    }
    fn inode_mut<P>(&mut self, path: P) -> Result<&mut Inode<'s>, String> where P: Iterator<Item=&'s str> {
        let mut node = &mut self.root;
        let mut processed_path = Vec::new();
        for seg in path {
            processed_path.push(seg);
            if let Some(n) = node.child_mut(seg) {
                node = n;
            } else {
                return Err(format!(
                    "Missing dir node: \"/{}\"",
                    processed_path.join("/")
                ));
            }
        }
        Ok(&mut node)
    }
    fn inode<P>(&self, path: P) -> Result<&Inode<'s>, String> where P: Iterator<Item=&'s str> {
        let mut node = &self.root;
        let mut processed_path = Vec::new();
        for seg in path {
            processed_path.push(seg);
            if let Some(n) = node.child(seg) {
                node = n;
            } else {
                return Err(format!(
                    "Missing dir node: \"/{}\"",
                    processed_path.join("/")
                ));
            }
        }
        Ok(&node)
    }
}
impl<'a> FromStr for PartialTree<'a> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tree = Self::new();
        for cmd in s.split("$ ").filter_map(|s| {
            let s = s.trim();
            if s.len() > 0 {
                if s.starts_with("cd") {
                    Some(Command::Cd(s.split_once(" ").unwrap().1.into()))
                } else if s.starts_with("ls") {
                    Some(Command::Ls(
                        s.split_whitespace()
                            .skip(1)
                            .filter_map(|line| {
                                let line = line.trim();
                                if line.len() > 0 {
                                    line.split_once(" ").map(|(s, name)| {
                                        if s == "dir" {
                                            Inode::Dir(name, None)
                                        } else {
                                            Inode::File(
                                                name,
                                                s.parse().expect("Error parsing file size"),
                                            )
                                        }
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    ))
                } else {
                    None
                }
            } else {
                None
            }
        }) {
            match cmd {
                Command::Cd(arg) => tree.cd(arg),
                Command::Ls(contents) => tree.ls(contents)?,
            }
        }
        Ok(tree)
    }
}

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

pub fn main(parts: crate::RunPart) {
    let tree = TEST_INPUT.parse::<PartialTree>();

}
