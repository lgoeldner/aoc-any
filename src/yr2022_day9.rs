use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
    ops::Sub,
    str::FromStr,
};

use anyhow::anyhow;
use gxhash::GxHashSet;

use aoc_any::{set_trait::Set, BenchTimes, Info, Run, Solution};

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
    part1: |data| do_part1(parse(data).unwrap(), GxHashSet::default()).into(),
    part2: Some(|data| part2(data).into()),
    other: &[
        ("BTreeSet part1", |_| part1_btreeset().into(), Run::No),
        (
            "GxHash part1",
            |data| do_part1(parse(data).unwrap(), GxHashSet::default()).into(),
            Run::No,
        ),
        (
            "StdHash part1",
            |data| do_part1(parse(data).unwrap(), HashSet::new()).into(),
            Run::No,
        ),
        (
            "part1 example gxhash",
            |_| do_part1(parse(EXAMPLE).unwrap(), GxHashSet::default()).into(),
            Run::No,
        ),
    ],
};

fn part2(data: &str) -> u32 {
    let data = parse(data).unwrap();
    do_part2(data, GxHashSet::default())
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

impl Sub for Pos {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn do_part1(data: Vec<Data>, mut set: impl Set<Pos>) -> u32 {
    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };
    for (dir, times) in data {
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

fn do_part2(data: Vec<Data>, mut set: impl Set<Pos>) -> u32 {
    let mut snake = [Pos { x: 0, y: 0 }; 10];

    for (dir, times) in data {
        for _ in 0..times {
            snake[0].apply(&dir);

            for (head, snd) in (0..10).zip(1..10) {
                update_tail_match(snake[head], &mut snake[snd]);
            }

            set.insert(snake[9]);
        }
    }
    set.len() as u32
}

fn update_tail(head: Pos, tail: &mut Pos, set: &mut impl Set<Pos>) {
    update_tail_match(head, tail);
    // eprintln!("{i}");
    // print_grid(&head, &tail);
    set.insert(*tail);
}

fn update_tail_match(head: Pos, tail: &mut Pos) {
    match head - *tail {
        Pos { x: 0, y: 2 } => tail.y += 1,
        Pos { x: 2, y: 0 } => tail.x += 1,
        Pos { x: -2, y: 0 } => tail.x -= 1,
        Pos { x: 0, y: -2 } => tail.y -= 1,

        // diagonal cases
        Pos { x: _, y: 2 | -2 } | Pos { x: 2 | -2, y: _ } => {
            let diagonal = head - *tail;

            if diagonal.y.is_positive() {
                tail.y += 1;
            } else {
                tail.y -= 1;
            }

            if diagonal.x.is_positive() {
                tail.x += 1;
            } else {
                tail.x -= 1;
            }
        }

        // any => {dbg!(any);}
        _ => {}
    }
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

fn parse(data: &str) -> anyhow::Result<Vec<Data>> {
    data.lines()
        .map(|line| {
            let (dir, n) = line.split_once(' ').unwrap();
            Ok((dir.parse()?, n.parse()?))
        })
        .collect()
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

const fn get_data() -> &'static str {
    include_str!("../inputs/day9-inp.txt")
}
