use std::{
    cmp::max,
    collections::{BTreeSet, HashSet},
    hash::Hash,
    ops::Sub,
    str::FromStr,
};

use anyhow::anyhow;
use aoc_any::{BenchTimes, Info, Run, Solution};

#[rustfmt::skip]
const EXAMPLE: &str = 
"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

pub const SOLUTION: Solution = Solution {
    info: Info {
        name: "Rope Bridge",
        day: 9,
        year: 2022,
        bench: BenchTimes::Once,
    },
    part1: || part1().into(),
    part2: None,
    other: &[
        ("BTreeSet part1", || part1_btreeset().into(), Run::No),
        (
            "part 1 example",
            || do_part1(parse(EXAMPLE).unwrap(), BTreeSet::new()).into(),
            Run::No,
        ),
    ],
};

fn part1() -> u32 {
    let data = parse(get_data()).unwrap();
    do_part1(data, HashSet::new())
}

fn part1_btreeset() -> u32 {
    let data = parse(get_data()).unwrap();
    do_part1(data, BTreeSet::new())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl Sub for Pos {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

trait Set<T> {
    fn insert(&mut self, item: T);
    fn len(&self) -> usize;
}

impl<T: std::cmp::Eq + std::hash::Hash> Set<T> for HashSet<T> {
    fn insert(&mut self, item: T) {
        HashSet::insert(self, item);
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<T: std::cmp::Eq + std::hash::Hash + Ord> Set<T> for BTreeSet<T> {
    fn insert(&mut self, item: T) {
        BTreeSet::insert(self, item);
    }

    fn len(&self) -> usize {
        BTreeSet::len(self)
    }
}

fn do_part1(data: Vec<Data>, mut set: impl Set<Pos>) -> u32 {
    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };
    for (dir, times) in data.into_iter() {
        // eprintln!("{dir:?} {times}");
        for _ in 0..times {
            head.apply(&dir);
            // update the tails state
            // if the tail is two steps away horizontally or vertically
            // move in the same direction
            // else move diagonally

            update_tail(head, &mut tail, &mut set);
        }
    }
    set.len() as u32
}

fn update_tail(head: Pos, tail: &mut Pos, set: &mut impl Set<Pos>) {
    match head - *tail {
        Pos { x: 0, y: 2 } | Pos { x: 2, y: 0 } => tail.y += 1,
        Pos { x: -2, y: 0 } | Pos { x: 0, y: -2 } => tail.x -= 1,

        // diagonal cases
        Pos { x: _, y: 2 } | Pos { x: 2, y: _ } | Pos { x: -2, y: _ } | Pos { x: _, y: -2 } => {
            let diagonal = head - *tail;
            match diagonal {
                Pos { x: 1, y: 2 } | Pos { x: 2, y: 1 } => {
                    tail.x += 1;
                    tail.y += 1
                }

                Pos { x: -2, y: 1 } | Pos { x: -1, y: 2 } => {
                    tail.x -= 1;
                    tail.y += 1
                }
                Pos { x: 2, y: -1 } | Pos { x: 1, y: -2 } => {
                    tail.x += 1;
                    tail.y -= 1
                }
                Pos { x: -2, y: -1 } | Pos { x: -1, y: -2 } => {
                    tail.x -= 1;
                    tail.y -= 1
                }
                _ => {
                    unreachable!()
                }
            }
        }

        // any => {dbg!(any);}
        _ => {}
    }
    // eprintln!("{i}");
    // print_grid(&head, &tail);
    set.insert(tail);
}

impl Pos {
    fn apply(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

type Data = (Direction, u8);

fn parse(data: &str) -> Result<Vec<Data>, anyhow::Error> {
    data.lines()
        .map(|line| -> Result<Data, anyhow::Error> {
            let (dir, n) = line.split_once(' ').unwrap();
            Ok((dir.parse()?, n.parse()?))
        })
        .collect()
}

fn print_grid(head: &Pos, tail: &Pos) {
    let max_x = max(max(head.x, tail.x), 5);
    let max_y = max(max(head.y, tail.y), 5);

    for y in (0..=max_y).rev() {
        let mut buf: Vec<&str> = Vec::with_capacity(max_x as usize);
        for x in 0..=max_x {
            let pos = Pos { x, y };
            if *head == pos {
                buf.push("H");
            } else if *tail == pos {
                buf.push("T");
            } else {
                buf.push(".");
            }
        }
        println!("{}", &buf.join(""));
    }
    println!();
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => return Err(anyhow!("Invalid Char")),
        })
    }
}

fn get_data() -> &'static str {
    include_str!("../inputs/day9-inp.txt")
}
