#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::reversed_empty_ranges)]
#![allow(clippy::cast_possible_truncation)]

use std::ops::BitOr;

use itertools::Itertools;
use nd::prelude::*;
use ndarray as nd;
use rayon::prelude::*;

use aoc_any::{zip, BenchTimes, Info, ProblemResult, Run};

pub const SOLUTION: aoc_any::Solution = aoc_any::Solution {
    info: Info {
        name: "Treetop Tree House",
        day: 8,
        year: 2022,
        bench: BenchTimes::Many(100),
    },
    part1: |data| part1nd(data).into(),
    part2: Some(|data| ProblemResult::Number(part2(data).try_into().unwrap())),
    other: &[
        ("part1 legacy", |_| part1().into(), Run::No),
        (
            "heavy input, 1 + 2",
            |_| ProblemResult::Other(Box::new(big_inp_1and2())),
            Run::No,
        ),
    ],
};

#[derive(Clone, PartialEq, Eq)]
struct TreeVis(u8, bool, Reason);

#[derive(Clone, PartialEq, Eq, Default)]
struct TreeVisNd(u8, bool);

impl std::fmt::Debug for TreeVisNd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{} {}}}", self.0, if self.1 { "#" } else { "." },)
    }
}

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

fn do_part1nd(data: Array2<TreeVisNd>) -> u32 {
    let data: Array2<TreeVisNd> = mark_visible_trees_nd(data);
    let inner_view: ArrayView2<TreeVisNd> = slice_treevis_nd(&data);

    // sum the number of visible trees
    let res = inner_view
        .iter()
        // sum the number of visible trees inside the inner slice
        .fold(0, |acc, item| acc + u32::from(item.1));

    res + {
        // calculates the overhead for the outer two rows and columns
        let (x, y) = data.dim();
        // double each dimension for top and bottom row/column,
        // and subtract 4 for overlapping corners
        (x * 2 + y * 2 - 4) as u32
    }
}

pub fn part1nd(data: &str) -> u32 {
    do_part1nd(parse_nd(data))
}

pub fn part1() -> u32 {
    let data = parse(get_data()).0;
    to_visible_treecount(data)
}

pub fn part2(data: &str) -> usize {
    max_scenic_score(&parse_nd(data))
}

pub fn big_inp_1and2() -> (u32, usize) {
    let data = include_str!("../inputs/aoc_2022_day08_sparse.txt");

    let part2_res = if cfg!(debug_assertions) {
        par_max_scenic_score(&parse_nd(data))
    } else {
        max_scenic_score(&parse_nd(data))
    };
    (do_part1nd(parse_nd(data)), part2_res)
}

fn max_scenic_score(data: &Array2<TreeVisNd>) -> usize {
    data.indexed_iter()
        .map(|((i, j), pos)| {
            // construct slices in every direction, reversing left and up views
            let (left, right) = &data.row(i).split_at(Axis(0), j);
            // slice away the current position, reverse the before slice
            let left = left.slice(s![..;-1]);
            let right = right.slice(s![1..]);

            let (up, down) = &data.column(j).split_at(Axis(0), i);
            let up = up.slice(s![..;-1]);
            let down: ArrayView1<_> = down.slice(s![1..]);

            let traverse = |row: &ArrayView1<TreeVisNd>| {
                row.iter().take_while_inclusive(|x| x.0 < pos.0).count()
            };

            [up, right, down, left]
                .iter()
                .map(traverse)
                .product::<usize>()
        })
        .max()
        .unwrap()
}

/// same thing as above but in parallel
fn par_max_scenic_score(data: &Array2<TreeVisNd>) -> usize {
    data.indexed_iter()
        .par_bridge()
        .map(|((i, j), pos)| {
            // construct slices in every direction, reversing left and up views
            let (left, right) = &data.row(i).split_at(Axis(0), j);
            // slice away the current position, reverse the before slice
            let left = left.slice(s![..;-1]);
            let right = right.slice(s![1..]);

            let (up, down) = &data.column(j).split_at(Axis(0), i);
            let up = up.slice(s![..;-1]);
            let down: ArrayView1<_> = down.slice(s![1..]);

            let traverse = |row: &ArrayView1<TreeVisNd>| {
                row.iter().take_while_inclusive(|x| x.0 < pos.0).count()
            };

            [up, right, down, left]
                .iter()
                .map(traverse)
                .product::<usize>()
        })
        .max()
        .unwrap()
}

/// return a view into the array with the outer two rows and colums removed
fn slice_treevis_nd(data: &Array2<TreeVisNd>) -> ArrayView2<TreeVisNd> {
    data.slice(s![1..-1, 1..-1])
}

/// marks every visible tree in the matrix by going over row and columns forward and reverse
fn mark_visible_trees_nd(mut data: Array2<TreeVisNd>) -> Array2<TreeVisNd> {
    /// two helper functions
    fn mark_visible_trees(inp: nd::iter::LanesMut<'_, TreeVisNd, ndarray::Dim<[usize; 1]>>) {
        for row in inp {
            let mut max_treeheight = 0;
            for cell in row {
                if cell.0 > max_treeheight {
                    max_treeheight = cell.0;
                    cell.1 = true;
                }
            }
        }
    }

    fn mark_visible_trees_rev(inp: nd::iter::LanesMut<'_, TreeVisNd, ndarray::Dim<[usize; 1]>>) {
        for row in inp {
            let mut max_treeheight = 0;
            for cell in row.into_iter().rev() {
                if cell.0 > max_treeheight {
                    max_treeheight = cell.0;
                    cell.1 = true;
                }
            }
        }
    }

    mark_visible_trees(data.rows_mut());
    mark_visible_trees_rev(data.rows_mut());

    mark_visible_trees(data.columns_mut());
    mark_visible_trees_rev(data.columns_mut());

    data
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
        .map(|line| line.iter().fold(0, |sum, item| sum + u32::from(item.1)))
        .sum::<u32>()
        + data1[0].len() as u32 * 2
        + data1.len() as u32 * 2
        - 4; // account for corners

    sum
}

#[allow(unused)]
fn dbg_vistreegrid(inp: &[impl AsRef<[TreeVis]>]) {
    const ENABLED: bool = false;
    #[cfg(debug_assertions)]
    if ENABLED {
        eprintln!("<<===>>");
        for line in inp {
            for cell in line.as_ref() {
                eprint!(" {cell:?} ");
            }
            eprintln!();
        }
        eprintln!("<<===>>");
    }
}

impl BitOr for TreeVis {
    type Output = bool;
    fn bitor(self, rhs: Self) -> Self::Output {
        self.1 || rhs.1
    }
}

fn to_visible_treeline(inp: &[Tree], horizontal: bool) -> Vec<TreeVis> {
    fn traverse<'a>(iter: impl Iterator<Item = &'a Tree>, reason: &Reason) -> Vec<TreeVis> {
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

    let (reason1, reason2) = if horizontal {
        (Reason::Horizontal, Reason::HorizontalRev)
    } else {
        (Reason::Vertical, Reason::VerticalRev)
    };

    let data = traverse(inp.iter(), &reason1);

    let data2 = traverse(inp.iter().rev(), &reason2);

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

/// <https://stackoverflow.com/a/64499219/24086138>
fn transpose2<T: Send + Sync>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    debug_assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters = v
        .into_iter()
        .map(std::iter::IntoIterator::into_iter)
        .collect::<Vec<_>>();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[derive(Clone, Copy, Default)]
struct Tree(u8);

impl std::fmt::Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<char> for Tree {
    fn from(val: char) -> Self {
        Self(
            val.to_digit(10)
                .unwrap_or_else(|| panic!("invalid char: {val}")) as u8,
        )
    }
}

impl From<Vec<Vec<Tree>>> for Data {
    fn from(value: Vec<Vec<Tree>>) -> Self {
        Self(value)
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

fn parse_nd(data: &str) -> Array2<TreeVisNd> {
    let arr_size_y = data.lines().count();
    let arr_size_x = data.lines().next().unwrap().chars().count();

    let mut arr: Array2<TreeVisNd> = Array2::default((arr_size_y, arr_size_x));

    let mut data_flat_iter = data.lines().flat_map(|line| line.chars().map(<Tree>::from));

    for cell in &mut arr {
        *cell = TreeVisNd(data_flat_iter.next().unwrap().0, false);
    }

    arr
}

fn parse(data: &str) -> Data {
    data.lines()
        .map(|line| line.chars().map(<Tree>::from).collect())
        .collect::<Vec<Vec<Tree>>>()
        .into()
}

const fn get_data() -> &'static str {
    include_str!("../inputs/day8-inp.txt")
}
