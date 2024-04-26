use aoc_any::{BenchTimes, Info, Solution};
use itertools::Itertools;
use std::collections::HashSet;

pub const SOLUTION: Solution = Solution {
    info: Info {
        name: "Rucksack Reorganization",
        day: 3,
        year: 2022,
        bench: BenchTimes::Default,
    },
    other: &[],
    part1: |data| part1(data).into(),
    part2: Some(|data| part2(data).into()),
};

const GROUP_SIZE: usize = 3;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
struct Group<'a>([&'a str; GROUP_SIZE]);

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
struct Data1<'a> {
    left: &'a str,
    right: &'a str,
}

pub fn part1(data: &str) -> u32 {
    parse_1(data)
        .map(|data: Data1<'_>| l_r_diff(&data))
        .map(char_to_prio)
        .sum()
}

fn parse_2(data: &str) -> Vec<Group> {
    data.lines()
        .chunks(GROUP_SIZE)
        .into_iter()
        .map(|group| Group(group.collect::<Vec<_>>().try_into().unwrap()))
        .collect()
}

fn char_to_prio(c: char) -> u32 {
    u32::from(c as u8 - if c.is_lowercase() { 96 } else { 38 })
}

fn l_r_diff(data: &Data1) -> char {
    let set1 = data.left.chars().collect::<HashSet<_>>();
    let set2 = data.right.chars().collect::<HashSet<_>>();
    *set1.intersection(&set2).next().unwrap()
}

fn parse_1(data: &str) -> impl Iterator<Item = Data1> {
    data.lines().map(|line| {
        let (left, right) = line.split_at(line.len() / 2);
        Data1 { left, right }
    })
}

pub fn part2(data: &str) -> u32 {
    parse_2(data)
        .iter()
        // the intersection of the three sets
        .map(diff_char_in_group)
        // then to priority
        .map(char_to_prio)
        .sum()
}

fn diff_char_in_group(group: &Group) -> char {
    // destructure the groups array into tree sets, using heavy type gymnastics
    let [set0, set1, set2]: [HashSet<char>; 3] = group
        .0
        .iter()
        .map(|str| str.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    // get the intersections of the sets, BitAnd is syntactic sugar for intersection and clone.
    // then iterate and return the first element
    *(&(&set0 & &set1) & &set2).iter().next().unwrap()
}
