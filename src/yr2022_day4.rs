use anyhow::Ok;
use std::cmp::{max, min};
use std::str::FromStr;

fn part1_withdata(data: String) -> anyhow::Result<u32> {
    let parsed = parse1(&data)?;
    Ok(parsed.iter().filter(|r2| r2.contains_self()).count() as u32)
}


fn part2_withdata(data: String) -> anyhow::Result<u32> {
    let parsed = parse1(&data)?;
    Ok(parsed.iter().filter(|r2| r2.overlaps()).count() as u32)
}

pub fn part1() -> anyhow::Result<u32> {
    part1_withdata(get_data())
}

pub fn part2() -> anyhow::Result<u32> {
    part2_withdata(get_data())
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
        let start_diff: i64 = self.fst.start as i64 - self.snd.start as i64;
        let end_diff = self.fst.end as i64 - self.snd.end as i64;
        ((start_diff >= 0) == (end_diff <= 0)) || ((start_diff <= 0) == (end_diff >= 0))
    }

    fn overlaps(&self) -> bool {
        //! if the two ranges together are "fatter" than the full range,
        //! the ranges overlap
        //! 
        //! see [here](https://i.stack.imgur.com/6iULg.png)

        let start = min(self.fst.start, self.snd.start);
        let end = max(self.fst.end, self.snd.end);
        // the minimum range required to overlap
        let fst_width = self.fst.end - self.fst.start;
        let snd_width = self.snd.end - self.snd.start;
        fst_width + snd_width > (end - start)
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

fn parse1(inp: &str) -> anyhow::Result<Data1> {
    inp.lines().map(<str>::parse).collect()
}

#[test]
fn test_part1() -> anyhow::Result<()> {
    let data = include_str!("../day4-test.txt").to_owned();
    dbg!(part1_withdata(data)).map(|_| ())?;

    Ok(())
}

#[test]
fn test() {
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
        )
    });
}
