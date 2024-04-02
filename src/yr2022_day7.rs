use std::{
    borrow::BorrowMut, cell::RefCell, collections::HashMap, default, fs::DirBuilder, ops::DerefMut,
    rc::Rc,
};

use anyhow::{anyhow, Context, Ok};

pub fn part1() -> anyhow::Result<()> {
    let parsed = parse(get_data())?;

    todo!()
}

// #[derive(Default, Debug)]
// enum Dir {
//     Node(Node),
//     #[default]
//     Nil,
// }

#[derive(Debug, Default)]
struct Dir {
    size: u64,
    files: Vec<File>,
    children: Vec<String>,
}

fn build_treemap(mut inp: Vec<Command>) {
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
                        curr_pos.push("/".to_string())
                    }
                }
            },
            Command::Ls { result } => {
                for dir in result {
                    let new_dir: Dir = dir.into();
                    update_parents(&mut treemap, &curr_pos, new_dir.size);
                    treemap.insert(curr_pos.clone(), new_dir);

                    // dbg!(&treemap);
                }
            }
        }
    }
}

fn update_parents(
    treemap: &mut HashMap<Vec<String>, Dir>,
    pos: &Vec<String>,
    size: u64,
) -> anyhow::Result<()> {
    fn do_update_parents(
        treemap: &mut HashMap<Vec<String>, Dir>,
        pos: &[String],
        size: u64,
    ) -> anyhow::Result<()> {
        if pos.is_empty() {
            return Ok(());
        }
        // if pos.len() == 1 {
        //     // reached root
        //     let root_node = dbg!(treemap.get(["/".to_owned()].as_slice()).context(""))?;
        // } else

        let [parent_path @ .., curr_path] = dbg!(pos) else {
            dbg!("irrefutable reached", pos);
            return Err(anyhow!("empty list"));
        };

        let parent_entry = dbg!(treemap.get_mut(dbg!(parent_path))).context("Hello")?;
        parent_entry.children.push(curr_path.to_owned());
        parent_entry.size += size;
        dbg!(parent_entry, parent_path);
        dbg!(&treemap);

        do_update_parents(treemap, parent_path, size)
    }

    // let len = pos.len() - 1;
    do_update_parents(treemap, pos, size)
}

impl Into<Dir> for LsDir {
    fn into(self) -> Dir {
        Dir {
            // take the name from self or the last element of curr_pos
            size: self.files.iter().fold(0u64, |acc, item| acc + item.size),
            files: self.files,
            children: Default::default(),
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
        .filter(|line| !line.is_empty())
        .map(|cmd| {
            let mut lines = cmd.lines().map(|line| line.trim()).peekable();
            let mut name: Option<String> = None;
            if let Some(line) = lines.peek() {
                if line.len() == 1 {
                    name = Some(lines.next().unwrap().to_owned());
                }
            }

            let files = lines
                .map(|line| {
                    let (size, name) = line.split_once(' ').unwrap();
                    File {
                        size: size.parse().unwrap(),
                        name: name.to_owned(),
                    }
                })
                .collect();

            LsDir { files, name }
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

fn get_data() -> &'static str {
    include_str!("../inputs/day7-test.txt")
}

#[test]
fn test_part1() -> anyhow::Result<()> {
    let _parsed = dbg!(parse(get_data())?);
    let x = build_treemap(_parsed);
    todo!()
}
