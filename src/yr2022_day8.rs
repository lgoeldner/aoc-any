// use rayon::prelude::*;
use std::{fmt::Display, ops::BitOr};

use itertools::Itertools;

pub fn part1() -> anyhow::Result<u32> {
    Ok(to_visible_treecount(parse(get_data()).0))
}

pub fn part2() -> u32 {
    process_part2(parse(get_data()))
}

fn process_part2(data: Data) -> u32 {
    let data = data.0;

    // go over each tree, map it to its "scenic score"

    todo!()
}

fn to_scenic_score(mut data: Vec<Vec<Tree>>) -> Vec<Vec<ScenicScore>> {
    for (i, line) in data.iter().enumerate() {
        for (j, tree) in line.iter().enumerate() {}
    }

    todo!()
}

struct ScenicScore(u32);

#[derive(Clone, PartialEq, Eq)]
struct TreeVis(u8, bool, Reason);

#[derive(Clone, PartialEq, Eq, Debug)]
enum Reason {
    Horizontal,
    Vertical,
    HorizontalRev,
    VerticalRev,
    Two(Box<(Reason, Reason)>),
}

impl std::fmt::Debug for TreeVis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "[{} {} R: {:?}]",
                self.0,
                if self.1 { "#" } else { "." },
                self.2
            )
        } else {
            write!(f, "[{} {}]", self.0, if self.1 { "#" } else { "." },)
        }
    }
}

fn to_visible_treecount(data: Vec<Vec<Tree>>) -> u32 {
    let mut data1 = data
        .iter()
        .map(|line| to_visible_treeline(line, true))
        .collect::<Vec<Vec<TreeVis>>>();

    let data2 = transpose2(data)
        .iter()
        .map(|line| to_visible_treeline(line, false))
        .collect::<Vec<Vec<TreeVis>>>();

    // transpose data2 again and join with data1
    let data2 = transpose2(data2);

    for (row1, row2) in zip(data1.iter_mut(), data2.iter()) {
        for (lcell, rcell) in zip(row1.iter_mut(), row2.iter()) {
            lcell.1 |= rcell.1;
            if rcell.1 {
                lcell.2 = Reason::Two(Box::new((lcell.2.clone(), rcell.2.clone())));
            }
        }
    }

    drop(data2);

    let data_width = data1[0].len();

    // #[cfg(debug_assertions)]
    // dbg_vistreegrid(&data1);

    let inner = data1[1..data1.len() - 1]
        .iter()
        .map(|line| &line[1..data_width - 1]);

    #[cfg(debug_assertions)]
    dbg_vistreegrid(&inner.clone().collect::<Vec<_>>());

    let sum: u32 = inner
        .map(|line| line.iter().fold(0, |sum, item| sum + item.1 as u32))
        .sum::<u32>()
        + data1[0].len() as u32 * 2
        + data1.len() as u32 * 2
        - 4; // account for corners

    let y: Staticlist<u32, Staticlist<String, Staticlist<u32, ()>>> =
        Staticlist(10, Staticlist("HELLO".to_owned(), Staticlist(10, ())));


    sum
}

struct Staticlist<A, B>(A, B);

#[allow(unused)]
fn dbg_vistreegrid(inp: &[impl AsRef<[TreeVis]>]) {
    const ENABLED: bool = true;
    #[cfg(debug_assertions)]
    if ENABLED {
        eprintln!("<<===>>");
        for line in inp {
            for cell in line.as_ref() {
                eprint!(" {:?} ", cell);
            }
            eprintln!()
        }
    }
}

impl BitOr for TreeVis {
    type Output = bool;
    fn bitor(self, rhs: Self) -> Self::Output {
        self.1 || rhs.1
    }
}

fn to_visible_treeline(inp: &[Tree], horizontal: bool) -> Vec<TreeVis> {
    fn traverse<'a>(iter: impl Iterator<Item = &'a Tree>, reason: Reason) -> Vec<TreeVis> {
        iter.scan(0, |max_treeheight, tree| {
            if tree.0 > *max_treeheight {
                *max_treeheight = tree.0;
                Some(TreeVis(tree.0, true, reason.clone()))
            } else {
                Some(TreeVis(tree.0, false, reason.clone()))
            }
        })
        .collect()
    }

    let (reason1, reason2) = match horizontal {
        true => (Reason::Horizontal, Reason::HorizontalRev),
        false => (Reason::Vertical, Reason::VerticalRev),
    };

    let data = traverse(inp.iter(), reason1);

    let data2 = traverse(inp.iter().rev(), reason2);

    let zipped = data
        .iter()
        .zip(data2.iter().rev())
        .map(|(lhs, rhs)| {
            // assert_eq!(lhs.0, rhs.0);
            if lhs.1 || rhs.1 {
                TreeVis(
                    lhs.0,
                    true,
                    match (lhs.1, rhs.1) {
                        (true, true) => Reason::Two(Box::new((lhs.2.clone(), rhs.2.clone()))),
                        (false, true) => rhs.2.clone(),
                        (true, false) => lhs.2.clone(),
                        (false, false) => unreachable!(),
                    },
                )
            } else {
                lhs.clone()
            }
        })
        .collect::<Vec<_>>();
    zipped
}

/// https://stackoverflow.com/a/64499219/24086138
fn transpose2<T: Send + Sync>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    debug_assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters = v.into_iter().map(|n| n.into_iter()).collect::<Vec<_>>();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

struct Tree(u8);

impl std::fmt::Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<char> for Tree {
    fn from(val: char) -> Self {
        Tree(
            val.to_digit(10)
                .unwrap_or_else(|| panic!("invalid char: {}", val)) as u8,
        )
    }
}

impl From<Vec<Vec<Tree>>> for Data {
    fn from(value: Vec<Vec<Tree>>) -> Data {
        Data(value)
    }
}

struct Data(Vec<Vec<Tree>>);

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n{}",
            self.0
                .iter()
                .map(|line| line.iter().map(|tree| tree.0.to_string()).join(""))
                .join("\n")
        )
    }
}

fn parse(data: &str) -> Data {
    data.lines()
        .map(|line| line.chars().map(<Tree>::from).collect())
        .collect::<Vec<Vec<Tree>>>()
        .into()
}

fn get_data() -> &'static str {
    if cfg!(debug_assertions) {
        include_str!("../inputs/day8-test.txt")
    } else {
        include_str!("../inputs/day8-inp.txt")
    }
}

/// utility function
fn zip<A: Iterator, B: Iterator>(a: A, b: B) -> impl Iterator<Item = (A::Item, B::Item)> {
    a.into_iter().zip(b.into_iter())
}
