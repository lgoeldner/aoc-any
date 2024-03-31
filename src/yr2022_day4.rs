use std::str::FromStr;

use anyhow::{anyhow, Context};
use itertools::Itertools;

fn part1_withdata(data: String) -> anyhow::Result<u32> {
    let parsed = dbg!(parse1(&data)?);

    todo!()
}

pub fn part1() -> anyhow::Result<u32> {
    part1_withdata(get_data())
}

pub fn part2() -> anyhow::Result<u32> {
    todo!()
}

fn get_data() -> String {
    include_str!("../day4-inp.txt").to_string()
}

type Data1 = Vec<Range2>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Range2 {
    fst: Range,
    snd: Range,
}

impl Range2 {
    fn contains_self(&self) -> bool {
        let start_diff: i32 = self.fst.start as i32 - self.snd.start as i32;
        let end_diff = self.fst.end as i32 - self.snd.end as i32;
        (start_diff >= 0) == (end_diff <= 0)
    }
}

impl FromStr for Range2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges: [&str; 2] = s.split_once(",").unwrap().into();
        let [fst, snd]: [_; 2] = ranges
            .iter()
            .map(|range| range.parse::<Range>())
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .expect("Range always has two numbers");
        Ok(Self { fst, snd })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: u32,
    end: u32,
}

impl FromStr for Range {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once("-").unwrap();
        Ok(Self {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

fn parse1(inp: &str) -> anyhow::Result<Data1> {
    inp.lines().map(<str>::parse).collect()
}

#[test]
fn test_part1() -> anyhow::Result<()> {
    let data = include_str!("../day4-test.txt").to_owned();
    part1_withdata(data).map(|_| ())
}
