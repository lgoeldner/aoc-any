use std::{collections::HashMap, vec};

use anyhow::Ok;
use aoc_any::{Info, ProblemResult, Solution};

pub const SOLUTION: Solution = Solution {
    info: Info {
        bench: aoc_any::BenchTimes::Default,
        day: 7,
        name: "No Space Left On Device",
        year: 2022,
    },
    other: &[],
    part1: || ProblemResult::Number(part1().try_into().unwrap()),
    part2: Some(|| ProblemResult::Number(part2().try_into().unwrap())),
};

pub fn part1() -> u64 {
    let parsed = parse(get_data()).unwrap();
    let map = build_treemap(parsed);
    sum_dir_size_under(map, 100_000)
}

pub fn part2() -> u64 {
    let parsed = parse(get_data()).unwrap();
    let map = build_treemap(parsed);
    smalles_del_to_free(map)
}

fn smalles_del_to_free(treemap: HashMap<Vec<String>, Dir>) -> u64 {
    let root = &treemap[&vec!["/".into()]];
    const DISK_SPACE: u64 = 70_000_000;
    const SPACE_REQ: u64 = 30_000_000;
    let space_available = DISK_SPACE - root.size;
    let needs_deleted = SPACE_REQ - space_available;

    treemap
        .values()
        .filter_map(|dir| {
            if dir.size >= needs_deleted {
                Some(dir.size)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

fn sum_dir_size_under(treemap: HashMap<Vec<String>, Dir>, cap: u64) -> u64 {
    treemap
        .values()
        .filter_map(|dir| {
            if dir.size <= cap {
                Some(dir.size)
            } else {
                None
            }
        })
        .sum()
}

#[derive(Debug, Default, Clone)]
struct Dir {
    size: u64,
    files: Vec<File>,
    // children: Vec<String>,
}

fn build_treemap(inp: Vec<Command>) -> HashMap<Vec<String>, Dir> {
    let mut treemap = HashMap::from([(vec!["/".to_string()], Dir::default())]);
    let mut curr_pos = vec!["/".to_string()];
    for cmd in inp {
        match cmd {
            Command::Cd { to } => match to {
                CdTarget::Name(target_name) => curr_pos.push(target_name),
                CdTarget::Root => drop(curr_pos.drain(1..)),
                CdTarget::Parent => {
                    curr_pos.pop().unwrap();
                    if curr_pos.is_empty() {
                        curr_pos.push("/".to_string());
                    }
                }
            },
            Command::Ls { result } => {
                use std::collections::hash_map::Entry;
                for dir in result {
                    let new_dir: Dir = dir.clone().into();
                    // merge new dir and maybe old one
                    let dir_entry = treemap.entry(curr_pos.clone());
                    match dir_entry {
                        Entry::Vacant(vacant) => {
                            vacant.insert(new_dir.clone());
                        }
                        Entry::Occupied(mut occ) => {
                            let old = occ.get();
                            let mut new = new_dir.clone();
                            new.files.append(&mut old.files.clone());
                            occ.insert(Dir {
                                files: new.files,
                                size: old.size + new.size,
                            });
                        }
                    }
                    // treemap.insert(curr_pos.clone(), new_dir.clone());
                    let _ = update_parents(&mut treemap, &curr_pos, new_dir.size);
                }
            }
        }
    }
    treemap
}

fn update_parents(
    treemap: &mut HashMap<Vec<String>, Dir>,
    pos: &[String],
    size: u64,
) -> anyhow::Result<()> {
    let len = pos.len();

    for i in (0..len).rev().skip(1) {
        let curr_pos = &pos[..=i];
        let curr_pos = treemap.get_mut(curr_pos).unwrap();
        curr_pos.size += size;
    }

    Ok(())
}

impl From<LsDir> for Dir {
    fn from(val: LsDir) -> Self {
        Self {
            // take the name from self or the last element of curr_pos
            size: val.files.iter().fold(0u64, |acc, item| acc + item.size),
            files: val.files,
        }
    }
}

fn parse(inp: &str) -> anyhow::Result<Vec<Command>> {
    Ok(inp
        .split("$ ")
        .skip(1)
        .map(|cmd| match &cmd[..2] {
            "cd" => Command::Cd {
                to: parse_cd(&cmd[3..]),
            },
            "ls" => Command::Ls {
                result: parse_ls(&cmd[3..]),
            },
            _ => panic!("unknown command"),
        })
        .collect::<Vec<_>>())
}

fn parse_ls(cmd: &str) -> Vec<LsDir> {
    cmd.split("dir ")
        // .skip(1)
        .filter(|line| !line.is_empty())
        .map(|cmd| {
            let mut lines = cmd
                .lines()
                .map(str::trim)
                // .inspect(|item| {
                //     let _ = dbg!(item);
                // })
                .peekable();
            let mut name: Option<String> = None;
            if let Some(line) = lines.peek() {
                if !line.is_empty() && line.chars().next().unwrap().is_alphabetic() {
                    name = Some(lines.next().unwrap().to_owned());
                } else if line.is_empty() {
                    lines.next().unwrap();
                }
            }

            let files = lines
                .enumerate()
                .map(|(_, line)| {
                    let (size, name) = line.split_once(' ').unwrap_or_else(|| {
                        //  eprintln!("@l {i}: {line}");
                        panic!();
                    });
                    File {
                        size: size.parse().unwrap(),
                        name: name.to_owned(),
                    }
                })
                .collect();

            LsDir { name, files }
        })
        .collect::<Vec<_>>()
}

fn parse_cd(cmd: &str) -> CdTarget {
    match cmd.trim() {
        "/" => CdTarget::Root,
        ".." => CdTarget::Parent,
        _ => CdTarget::Name(cmd.trim_end().to_string()),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Command {
    Ls { result: Vec<LsDir> },
    Cd { to: CdTarget },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum CdTarget {
    Root,
    Parent,
    Name(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct LsDir {
    name: Option<String>,
    files: Vec<File>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct File {
    name: String,
    size: u64,
}

const fn get_data() -> &'static str {
    #[cfg(test)]
    return include_str!("../inputs/day7-test.txt");

    #[cfg(not(test))]
    return include_str!("../inputs/day7-inp.txt");
}

#[test]
fn test_part1() -> anyhow::Result<()> {
    let parsed = parse(get_data())?;
    assert_eq!(sum_dir_size_under(build_treemap(parsed), 100_000), 95437);

    Ok(())
}

#[test]
fn test_part2() -> anyhow::Result<()> {
    let parsed = parse(get_data())?;
    assert_eq!(smalles_del_to_free(build_treemap(parsed)), 24_933_642);

    Ok(())
}
