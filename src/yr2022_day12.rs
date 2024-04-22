use std::convert::Into;
use std::str::FromStr;
use std::str::FromStr;

use ndarray::prelude::*;

use aoc_any::{BenchTimes, Info, Solution};

pub const SOLUTION: Solution = Solution {
    info: Info {
        name: "Hill Climbing Algorithm",
        day: 12,
        year: 2022,
        bench: BenchTimes::Default,
    },
    part1: |_| part1(EXAMPLE).into(),
    part2: None,
    other: &[],
};

const EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

fn part1(data: &str) -> u32 {
    todo!()
}

enum Point {
    Start,
    End,
    Height(u8),
}

impl From<char> for Point {
    fn from(s: char) -> Result<Self, Self::Err> {
        Ok(match s {
            'S' => Self::Start,
            'E' => Self::End,
            'a'..='z' => Self::Height(s as u8 - b'a'),
            _ => anyhow::bail!("Invalid point: {s}"),
        })
    }
}

fn parse(data: &str) -> Array2<Point> {
    Array2::from(data
        .lines()
        .map(|line| line.chars().map(Point::from).collect::<Vec<_>>())
        .collect::<Vec<_>>())
}
