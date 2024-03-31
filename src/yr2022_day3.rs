use itertools::Itertools;
use std::collections::HashSet;

pub fn part1() {
    let data = read_input();
    let diffs: u32 = parse_1(&data)
        .map(|data: Data1<'_>| l_r_diff(&data))
        .map(char_to_prio)
        .sum();
    dbg!(&diffs);
}

fn char_to_prio(c: char) -> u32 {
    (c as u8 - if c.is_lowercase() { 96 } else { 38 }) as u32
}

fn l_r_diff(data: &Data1) -> char {
    let set1 = data.left.chars().collect::<HashSet<_>>();
    let set2 = data.right.chars().collect::<HashSet<_>>();
    *set1.intersection(&set2).collect::<Vec<_>>()[0]
}

fn read_input() -> String {
    std::fs::read_to_string("inp.txt").unwrap()
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
struct Data1<'a> {
    left: &'a str,
    right: &'a str,
}

fn parse_1<'a>(data: &'a str) -> impl Iterator<Item = Data1<'a>> {
    data.lines().map(|line| {
        // let line = line.to_owned();
        let (left, right) = line.split_at(line.len() / 2);
        Data1 { left, right }
    })
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
struct Group<'a>([&'a str; 3]);

fn parse_2<'a>(data: &'a str) -> Vec<Group> {
    const GROUP_SIZE: usize = 3;

    data.lines()
        .chunks(GROUP_SIZE)
        .into_iter()
        .map(|group| Group(group.collect::<Vec<_>>().try_into().unwrap()))
        .collect()
}

pub fn part2() {
    let input = read_input();
    let groups = parse_2(&input);
    let result: u32 = groups.iter().map(group_diff).map(char_to_prio).sum();
    dbg!(&result);
}

fn group_diff(group: &Group) -> char {
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
