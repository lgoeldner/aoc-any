use std::collections::VecDeque;
use std::convert::Into;
use std::fmt::{Debug, Formatter};

use gxhash::GxHashSet;
use ndarray::prelude::*;

use aoc_any::{BenchTimes, Info, Run, Solution};
use tinyvec::{array_vec, ArrayVec};

pub const SOLUTION: Solution = Solution {
    info: Info {
        name: "Hill Climbing Algorithm",
        day: 12,
        year: 2022,
        bench: BenchTimes::Default,
    },
    part1: |data| part1(data).into(),
    part2: Some(|data| part2(data).into()),
    other: &[(
        "recursive part2",
        |data| {
            let (data, start_point) = parse_c::<true>(data);
            bfs2(
                &data,
                VecDeque::from([QueuedPoint {
                    pos: start_point,
                    dist: 0,
                }]),
            )
            .into()
        },
        Run::No,
    )],
};

const _EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct QueuedPoint {
    pos: (usize, usize),
    dist: u32,
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

fn part2(data: &str) -> u32 {
    let (data, start_point) = parse_c::<true>(data);
    let queue = VecDeque::from([QueuedPoint {
        pos: start_point,
        dist: 0,
    }]);

    loop_bfs_part2(&data, queue)
}

fn part1(data: &str) -> u32 {
    let (data, start_point) = parse_c::<false>(data);
    let queue = VecDeque::from([QueuedPoint {
        pos: start_point,
        dist: 0,
    }]);

    let x = loop_bfs(&data, queue);
    x.unwrap().dist
}

fn loop_bfs(data: &Array2<Point>, mut queue: VecDeque<QueuedPoint>) -> Option<QueuedPoint> {
    let mut visited = GxHashSet::from_iter([queue[0].pos]);

    'outer_loop: loop {
        let elem = queue.pop_front().unwrap();
        let adj = get_adjacent_part2(data, elem.pos, elem.dist, false);

        for node in adj {
            if data[node.pos] == Point::End {
                break 'outer_loop Some(node);
            }

            if !visited.contains(&node.pos) {
                queue.push_back(node.clone());
                visited.insert(node.pos);
            }
        }
    }
}

fn loop_bfs_part2(data: &Array2<Point>, mut queue: VecDeque<QueuedPoint>) -> u32 {
    let mut visited = GxHashSet::from_iter([queue[0].pos]);

    let mut min_dist = u32::MAX;

    'outer: loop {
        let elem = queue.pop_front().unwrap();

        let adj = get_adjacent_part2(data, elem.pos, elem.dist, true);
        for node in adj {
            if data[node.pos].get_height() == 0 {
                min_dist = min_dist.min(node.dist);
                break 'outer min_dist;
            }

            if !visited.contains(&node.pos) {
                queue.push_back(node.clone());
                visited.insert(node.pos);
            }
        }
    }
}

fn bfs2(data: &Array2<Point>, queue: VecDeque<QueuedPoint>) -> u32 {
    fn bfs_part2_rec(
        data: &Array2<Point>,
        mut queue: VecDeque<QueuedPoint>,
        mut visited: GxHashSet<(usize, usize)>,
        min_dist: u32,
    ) -> u32 {
        let elem = queue.pop_front();
        match elem {
            Some(elem) => {
                let adj = get_adjacent_part2(data, elem.pos, elem.dist, true);
                for node in adj {
                    if data[node.pos].get_height() == 0 {
                        return min_dist.min(node.dist);
                    }

                    if !visited.contains(&node.pos) {
                        queue.push_back(node.clone());
                        visited.insert(node.pos);
                    }
                }

                bfs_part2_rec(data, queue, visited, min_dist)
            }
            None => unreachable!(),
        }
    }

    let visited = GxHashSet::from_iter([queue[0].pos]);

    let min_dist = u32::MAX;

    bfs_part2_rec(data, queue, visited, min_dist)
}

fn get_adjacent_part2(
    data: &Array2<Point>,
    (x, y): (usize, usize),
    dist: u32,
    part_2: bool,
) -> ArrayVec<[QueuedPoint; 4]> {
    //  let view = data.slice(s![x - 1..x + 1, y - 1..y + 1]);
    let elem = data[(x, y)];

    let mut adj = array_vec![];

    let mut push_if = |pos| {
        if let Some(p) = data.get(pos) {
            let test = if part_2 {
                p.in_step(elem)
            } else {
                elem.in_step(*p)
            };

            if test {
                adj.push(QueuedPoint {
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

fn parse_c<const PART2: bool>(data: &str) -> (Array2<Point>, (usize, usize)) {
    let shape = (data.lines().count(), data.lines().next().unwrap().len());

    let mut arr = Array2::from_elem(shape, Point::Height(0));

    let mut start = None;

    arr.indexed_iter_mut()
        .zip(data.lines().flat_map(|line| line.chars().map(Point::from)))
        .for_each(|((idx, cell), iter)| {
            *cell = iter;
            if *cell == if PART2 { Point::End } else { Point::Start } {
                start = Some(idx);
            }
        });

    let start = {
        let (x, y) = start.expect("Grid should have a start point");
        (y, x)
    };

    (arr.permuted_axes([1, 0]), start)
}
