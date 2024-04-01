use std::{any, fmt};

use anyhow::{anyhow, Ok};

pub fn part1() -> anyhow::Result<()> {
    let parsed = parse(get_data())?;

    todo!()
}

fn parse(inp: &str) -> anyhow::Result<Vec<Command>> {
    Ok(inp
        .split("$ ")
        .skip(1)
        .map(|cmd| match &cmd[..2] {
            "cd" => Command::Cd {
                target: parse_cd(&cmd[3..]),
            },
            "ls" => Command::Ls {
                result: parse_ls(&cmd[3..]).unwrap(),
            },
            _ => panic!("unknown command"),
        })
        .collect::<Vec<_>>())

    // for cmd in  {
    //     let cmd = match &cmd[..2] {
    //         "cd" => Command::Cd {
    //             target: parse_cd(&cmd[3..]),
    //         },
    //         "ls" => Command::Ls {
    //             out: parse_ls(&cmd[3..]),
    //         },
    //         _ => panic!("unknown command"),
    //     };
    //     dbg!(cmd);
    // }

    // dbg!(inp.split("$ ").skip(1).collect::<Vec<&str>>());
}

fn parse_ls(cmd: &str) -> anyhow::Result<Vec<LsDir>> {
    "dir a
    14848514 b.txt
    8504156 c.dat
    dir d";
    cmd.split("dir ")
        // .skip(1)
        .filter(|line| !line.is_empty())
        .map(|cmd| {
            let mut lines = cmd.lines().peekable();
            let name = if let Some(line) = lines.peek() {
                if line.starts_with("dir") {
                    lines.next().unwrap().to_owned()
                } else {
                    "".to_owned()
                }
            } else {
                "".to_owned()
            };

            let files = lines
                .map(|line| {
                    let Some((size, name)) = line.split_once(' ') else {
                        panic!()
                    };
                    Ok(File {
                        size: size.parse()?,
                        name: name.to_string(),
                    })
                })
                .collect::<anyhow::Result<Vec<_>>>()?;
            Ok(LsDir { files, name })
        })
        .collect::<anyhow::Result<Vec<_>>>()
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
    Cd { target: CdTarget },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum CdTarget {
    Root,
    Parent,
    Name(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct LsDir {
    name: String,
    files: Vec<File>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct File {
    name: String,
    size: u32,
}

fn get_data() -> &'static str {
    include_str!("../inputs/day7-test.txt")
}

#[test]
fn test_part1() -> anyhow::Result<()> {
    let _parsed = dbg!(parse(get_data())?);

    todo!()
}
