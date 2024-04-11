use std::io::stdin;

use itertools::Itertools;

pub fn part1() -> anyhow::Result<u32> {
    let data = dbg!(parse(get_data()));

    let sum1: u32 = data.0[1..data.0.len() - 1]
        .iter()
        .map(|line| visible_in_line(&line))
        .sum::<u32>() + data.0[0].len() as u32 * 2;
	let data = Data(transpose2(data.0));
	let sum2: u32 = data.0[1..data.0.len() - 1]
        .iter()
        .map(|line| visible_in_line(&line))
        .sum::<u32>() + data.0[0].len() as u32 * 2;
	dbg!(sum1, sum2);
    todo!()
}

struct TreeVis(u32, bool);

fn to_visible_treegrid(data: &[Tree]) -> Vec<Vec<TreeVis>> {
	todo!()
}

fn visible_in_line(data: &[Tree]) -> u32 {
	println!();
    // iterate over the data in pairs, from front to back and in reverse
    // at the same time
    // keeping track of the maximum height for either
    data.iter()
        .zip(data.iter().rev())
        .fold(
            // first and last tree are max, sum is 0
            ((data[0].0, data.last().unwrap().0), 0),
            |((fwd_max, rev_max), sum), (fwd, rev)| {
				// printing
				match (fwd.0 > fwd_max, rev.0 > rev_max) {
					(true, _) | (_, true) => {
						print!("X")
					}
					_ => print!("0")
				}
                // check max for each, update sum accordingly
                match (fwd.0 > fwd_max, rev.0 > rev_max) {
                    (true, true) => ((fwd.0, rev.0), sum + 2),
                    (true, false) => ((fwd.0, rev_max), sum + 1),
                    (false, true) => ((fwd_max, rev.0), sum + 1),
                    (false, false) => ((fwd_max, rev_max), sum),
                }
            },
        )
        // get the maximum
        .1
}
/// https://stackoverflow.com/a/64499219/24086138
fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters = v.into_iter().map(|n| n.into_iter()).collect::<Vec<_>>();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

struct Tree(u8);

impl std::fmt::Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)   
    }
}

impl From<char> for Tree {
    fn from(val: char) -> Self {
        Tree(val.to_digit(10).unwrap() as u8)
    }
}

impl From<Vec<Vec<Tree>>> for Data {
    fn from(value: Vec<Vec<Tree>>) -> Data {
        Data(value)
    }
}

struct Data(Vec<Vec<Tree>>);

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n{}",
            self.0
                .iter()
                .map(|line| line.iter().map(|tree| tree.0.to_string()).join(""))
                .join("\n")
        )
    }
}

fn parse(data: &str) -> Data {
    data.lines()
        .map(|line| dbg!(line.chars().map(<Tree>::from).collect()))
        .collect::<Vec<Vec<Tree>>>()
        .into()
}

fn get_data() -> &'static str {
    if cfg!(debug_assertions) {
        include_str!("../inputs/day8-test.txt")
    } else {
        include_str!("../inputs/day8-inp.txt")
    }
}
