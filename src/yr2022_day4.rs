use anyhow::Ok;
use aoc_any::{BenchTimes, ProblemResult};
use std::cmp::{max, min};
use std::str::FromStr;

pub const SOLUTION: aoc_any::Solution = aoc_any::Solution {
    info: aoc_any::Info {
        name: "Camp Cleanup",
        day: 8,
        year: 2022,
        bench: BenchTimes::Many(100),
    },
    part1: || ProblemResult::Number(i64::from(part1().unwrap())),
    part2: Some(|| ProblemResult::Number(i64::from(part2().unwrap()))),
    other: &[],
};

fn part1_withdata(data: &str) -> anyhow::Result<u32> {
    let parsed = parse1(data)?;
    Ok(parsed.iter().filter(|r2| r2.contains_self()).count() as u32)
}

fn part2_withdata(data: &str) -> anyhow::Result<u32> {
    let parsed = parse1(data)?;
    Ok(parsed.iter().filter(|r2| r2.overlaps()).count() as u32)
}

pub fn part1() -> anyhow::Result<u32> {
    part1_withdata(&get_data())
}

pub fn part2() -> anyhow::Result<u32> {
    part2_withdata(&get_data())
}

fn get_data() -> String {
    include_str!("../inputs/day4-inp.txt").to_string()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Range2 {
    fst: Range,
    snd: Range,
}

impl Range2 {
    // checks if any of the two ranges is fully contained in the other
    fn contains_self(&self) -> bool {
        let start_diff = i64::from(self.fst.start) - i64::from(self.snd.start);
        let end_diff = i64::from(self.fst.end) - i64::from(self.snd.end);
        ((start_diff >= 0) == (end_diff <= 0)) || ((start_diff <= 0) == (end_diff >= 0))
    }

    /// if the two ranges together are "fatter" than the full range,
    /// the ranges overlap
    ///
    /// see [here](https://i.stack.imgur.com/6iULg.png)
    fn overlaps(&self) -> bool {
        // the full range taken up by the two ranges
        let start = min(self.fst.start, self.snd.start);
        let end = max(self.fst.end, self.snd.end);
        // the widths of each range
        let fst_width = self.fst.end - self.fst.start;
        let snd_width = self.snd.end - self.snd.start;
        // if the sum of the widths is greater or equal to the full range, it has to overlap
        fst_width + snd_width >= (end - start)
    }
}

impl FromStr for Range2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges: [&str; 2] = s.split_once(',').unwrap().into();
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
        let (start, end) = s.split_once('-').unwrap();
        Ok(Self {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

fn parse1(inp: &str) -> anyhow::Result<Vec<Range2>> {
    inp.lines().map(<str>::parse::<Range2>).collect()
}

#[test]
fn test_part1() -> anyhow::Result<()> {
    let data = include_str!("../inputs/day4-test.txt").to_owned();
    part1_withdata(data).map(|_| ())?;

    Ok(())
}

#[test]
fn contains_self_works() {
    [
        ("2-4,6-8", false),
        ("2-3,4-5", false),
        ("5-7,7-9", false),
        ("2-8,3-7", true),
        ("6-6,4-6", true),
        ("2-6,4-8", false),
        ("0-200,1-199", true),
        ("1-200,1-200", true),
        ("1-20,1-1", true),
        ("1-1,1-20", true),
        ("1-20,2-21", false),
    ]
    .iter()
    .map(|(x, y)| (x.parse::<Range2>().unwrap(), y))
    .for_each(|(range, expected)| {
        assert_eq!(range.contains_self(), *expected);
        eprintln!(
            "for {range:?}: {}, expected: {}",
            range.contains_self(),
            range.contains_self() == *expected
        );
    });
}

#[test]
fn overlap_works() {
    [
        ("2-4,6-8", false),
        ("2-3,4-5", false),
        ("5-7,7-9", true),
        ("2-8,3-7", true),
        ("6-6,4-6", true),
        ("2-6,4-8", true),
        ("1-20,2-21", true),
    ]
    .iter()
    .map(|(x, y)| (x.parse::<Range2>().unwrap(), y))
    .for_each(|(range, expected)| {
        assert_eq!(range.overlaps(), *expected);
    });
}
