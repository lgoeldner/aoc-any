use std::{
    ops::{BitAnd, BitOr},
    vec,
};

use itertools::Itertools;

pub fn part1() -> anyhow::Result<u32> {
    let data = dbg!(parse(get_data()));

    let sum1: u32 = data.0[1..data.0.len() - 1]
        .iter()
        .map(|line| visible_in_line(&line))
        .sum::<u32>()
        + data.0[0].len() as u32 * 2;
    let data = Data(transpose2(data.0));
    let sum2: u32 = data.0[1..data.0.len() - 1]
        .iter()
        .map(|line| visible_in_line(&line))
        .sum::<u32>()
        + data.0[0].len() as u32 * 2;
    let x = to_visible_treegrid(data.0);
    dbg!(x);
    dbg!(sum1, sum2);
    todo!()
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct TreeVis(u8, bool);

impl std::fmt::Debug for TreeVis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, if self.1 { "#" } else { "." })
    }
}

fn to_visible_treegrid(data: Vec<Vec<Tree>>) -> Vec<Vec<TreeVis>> {
    let data1 = data
        .iter()
        .map(|line| to_visible_treeline(line))
        .collect::<Vec<Vec<TreeVis>>>();

    let data2 = transpose2(data)
        .iter()
        .map(|line| to_visible_treeline(&line))
        .collect::<Vec<Vec<TreeVis>>>();
    dbg!(data1, data2);
    todo!()
}

impl BitOr for TreeVis {
    type Output = bool;
    fn bitor(self, rhs: Self) -> Self::Output {
        self.1 || rhs.1
    }
}

#[test]
fn test_visible_treeline() {
    let data = &parse("1233471").0[0];
    assert_eq!(
        to_visible_treeline(data)
            .iter()
            .fold(0, |acc, x| acc + x.1 as u32),
        5
    )
}

fn to_visible_treeline(inp: &[Tree]) -> Vec<TreeVis> {
	fn traverse<'a>(iter: impl Iterator<Item = &'a Tree>) -> Vec<TreeVis> {
		iter.scan(0, |max_treeheight, tree| {
            if tree.0 > *max_treeheight {
                *max_treeheight = tree.0;
                Some(TreeVis(tree.0, true))
            } else {
                Some(TreeVis(tree.0, false))
            }
        })
        .collect::<Vec<_>>()
	}

	let data = traverse(inp.iter());
	let data2 = traverse(inp.iter().rev());
	
    let zipped = data
        .iter()
        .zip(data2.iter())
        .map(|(lhs, rhs)| {
            // assert_eq!(lhs.0, rhs.0);
            if lhs.1 || rhs.1 {
                TreeVis(lhs.0, true)
            } else {
                *lhs
            }
        })
        .collect::<Vec<_>>();
    dbg!(zipped)
}

fn visible_in_line(data: &[Tree]) -> u32 {
    println!();
    // iterate over the data in pairs, from front to back and in reverse
    // at the same time
    // keeping track of the maximum height for either
    data.iter()
        .zip(data.iter().rev())
        .fold(
            // first and last tree are max, sum is 0
            ((data[0].0, data.last().unwrap().0), 0),
            |((fwd_max, rev_max), sum), (fwd, rev)| {
                // printing
                match (fwd.0 > fwd_max, rev.0 > rev_max) {
                    (true, _) | (_, true) => {
                        print!("X")
                    }
                    _ => print!("0"),
                }
                // check max for each, update sum accordingly
                match (fwd.0 > fwd_max, rev.0 > rev_max) {
                    (true, true) => ((fwd.0, rev.0), sum + 2),
                    (true, false) => ((fwd.0, rev_max), sum + 1),
                    (false, true) => ((fwd_max, rev.0), sum + 1),
                    (false, false) => ((fwd_max, rev_max), sum),
                }
            },
        )
        // get the maximum
        .1
}
/// https://stackoverflow.com/a/64499219/24086138
fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
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
        Tree(val.to_digit(10).unwrap() as u8)
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
