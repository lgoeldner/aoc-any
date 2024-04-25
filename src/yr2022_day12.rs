use std::collections::VecDeque;
use std::convert::Into;
use std::fmt::{Debug, Formatter};

use gxhash::GxHashSet;
use ndarray::prelude::*;

use aoc_any::{BenchTimes, Info, Solution};

pub const SOLUTION: Solution = Solution {
    info: Info {
        name: "Hill Climbing Algorithm",
        day: 12,
        year: 2022,
        bench: BenchTimes::None,
    },
    part1: |data| part1(data).into(),
    part2: None,
    other: &[],
};

const _EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

#[derive(Debug, Clone, PartialEq, Eq)]
struct BfsQ {
    pos: (usize, usize),
    dist: u32,
}

#[derive(PartialEq, Eq, Clone)]
struct DPoint {
    dist: u32,
    previous: Option<(usize, usize)>,
    pos: (usize, usize),
}

impl std::cmp::PartialOrd for DPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for DPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist).reverse()
    }
}

fn part1(data: &str) -> u32 {
    let (data, start_point) = parse(data);
    let queue = VecDeque::from([BfsQ {
        pos: start_point,
        dist: 0,
    }]);

    let x = loop_bfs(&data, queue);
    x.unwrap().dist
}

fn loop_bfs(data: &Array2<Point>, mut queue: VecDeque<BfsQ>) -> Option<BfsQ> {
    let mut visited = queue
        .iter()
        .map(|it| it.pos)
        .collect::<GxHashSet<_>>();
    'outer_loop: loop {
        let elem = queue.pop_front();
        match elem {
            Some(elem) => {
                let adj = get_adjacent(data, elem.pos, elem.dist);
                for node in adj {
                    // match visited.entry(node.pos) {
                    //     Entry::Occupied(mut entry) if entry.get() > &node.dist => {
                    //         //if entry.get() > &node.dist {
                    //         entry.insert(node.dist);
                    //         queue.push_back(node);
                    //         //}
                    //     }
                    //     Entry::Occupied(_) => {}
                    //     Entry::Vacant(entry) => {
                    //         entry.insert(node.dist);
                    //         queue.push_back(node);
                    //     }
                    // }

					if data[node.pos] == Point::End {
						break 'outer_loop Some(node);
					}

                    
                    if !visited.contains(&node.pos) {
                        queue.push_back(node.clone());
                        visited.insert(node.pos);
                    }
                }
            }
            None => break None,
        }
    }
}

fn _bfs(data: &Array2<Point>, queue: &mut VecDeque<BfsQ>) -> Option<BfsQ> {
    match queue.pop_front() {
        Some(end) if data[end.pos] == Point::End => Some(end),
        Some(elem) => {
            let adj = get_adjacent(data, elem.pos, elem.dist);
            queue.extend(adj);
            _bfs(dbg!(data), dbg!(queue))
            // None
        }
        None => None,
    }
}

#[test]
fn test_adj() {
	// let mut x = AocRuntime::new().unwrap();
	// let data = x.input_cache.get(&SOLUTION.info).unwrap();
    let data = parse(_EXAMPLE).0;
    dbg!(data.shape());

    assert_eq!(
        get_adjacent(&data, (4, 4), 0),
        vec![
            BfsQ {
                dist: 1,
                pos: (3, 4),
            },
            BfsQ {
                dist: 1,
                pos: (5, 4),
            }
        ]
    );
}

fn get_adjacent(data: &Array2<Point>, (x, y): (usize, usize), dist: u32) -> Vec<BfsQ> {
    //  let view = data.slice(s![x - 1..x + 1, y - 1..y + 1]);
    let elem = data[(x, y)];

    let mut adj = vec![];

    let mut push_if = |pos| {
        if let Some(p) = data.get(pos) {
            if elem.in_step(*p) {
                adj.push(BfsQ {
                    dist: dist + 1,
                    pos,
                });
            }
        }
    };

    if x > 0 {
        push_if((x - 1, y));
    }

    if x + 1 < data.shape()[0] {
        push_if((x + 1, y));
    }

    if y > 0 {
        push_if((x, y - 1));
    }

    if y + 1 < data.shape()[1] {
        push_if((x, y + 1));
    }

    adj
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Point {
    Start,
    End,
    Height(u8),
}

impl Point {
    const fn height(inp: Self) -> u8 {
        match inp {
            Self::End => Self::from('z').get_height(),
            Self::Start => Self::from('a').get_height(),
            Self::Height(n) => n,
        }
    }

    const fn get_height(self) -> u8 {
        Self::height(self)
    }

    fn in_step(self, other: Self) -> bool {
        (i16::from(Self::height(other)) - i16::from(Self::height(self))) <= 1
    }
}

#[allow(clippy::fallible_impl_from)]
impl Point {
    const fn from(s: char) -> Self {
        match s {
            'S' => Self::Start,
            'E' => Self::End,
            'a'..='z' => Self::Height(s as u8 - b'a'),
            _ => panic!("Invalid point"),
        }
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::End => 'E',
                Self::Start => 'S',
                Self::Height(n) => (b'a' + *n) as char,
            }
        )
    }
}

fn parse(data: &str) -> (Array2<Point>, (usize, usize)) {
    let shape = (data.lines().count(), data.lines().next().unwrap().len());

    let mut arr = Array2::from_elem(shape, Point::Height(0));

    let mut start = None;

    arr.indexed_iter_mut()
        .zip(data.lines().flat_map(|line| line.chars().map(Point::from)))
        .for_each(|((idx, cell), iter)| {
            *cell = iter;
            if *cell == Point::Start {
                start = Some(idx);
            }
        });

	let start = {
		let (x, y) = start.expect("Grid should have a start point");
		(y, x)
	};

    (
        arr.permuted_axes([1, 0]),
        start,
    )
}
