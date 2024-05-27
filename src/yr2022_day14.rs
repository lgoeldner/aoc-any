use aoc_any::{BenchTimes, Info, Solution};
use gxhash::GxHashMap;

use parse::{Point, Tile};

pub const SOLUTION: Solution = Solution {
    info: Info {
        name: "Regolith Reservoir",
        day: 14,
        year: 2022,
        bench: BenchTimes::None,
    },
    part1: |_data| part1(_data).into(),
    part2: Some(|data| part2(data).into()),
    other: &[],
};

const EXAMPLE: &str = include_str!("../inputs/2022-day14-test.txt");

fn part1(data: &str) -> u32 {
    let (map, deepest) = parse::part1(data);

    FallingSand { map, deepest }.count() as u32
}

fn part2(data: &str) -> u32 {
    let (map, deepest) = parse::part1(data);

    let mut falling_sand = FallingSand { map, deepest };

    while !falling_sand.origin_blocked() {
        let _ = falling_sand.add_sand(true);
    }
    todo!()
}

struct FallingSand {
    map: parse::Map,
    deepest: u32,
}

impl Iterator for FallingSand {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.add_sand(false).ok()
    }
}

impl FallingSand {
    fn origin_blocked(&self) -> bool {
        self.map.contains_key(&Point { x: 500, y: 0 })
    }

    fn add_sand(&mut self, part2: bool) -> Result<(), ()> {
        let mut sand = Point { x: 500, y: 0 };

        loop {
            if part2 {
                if sand.y > self.deepest + 1 {
                    // dbg!("bottom reached");

                    return Err(());
                }
            } else if sand.y > self.deepest {
                return Err(());
            }

            // try down

            if !self.map.contains_key(&Point {
                y: sand.y + 1,
                x: sand.x,
            }) {
                sand.y += 1;
                continue;
            }

            // then try down-left

            if !self.map.contains_key(&Point {
                y: sand.y + 1,
                x: sand.x - 1,
            }) {
                sand.y += 1;
                sand.x -= 1;
                continue;
            }

            // then try down-right

            if !self.map.contains_key(&Point {
                y: sand.y + 1,
                x: sand.x + 1,
            }) {
                sand.y += 1;
                sand.x += 1;
                continue;
            }

            // else rest

            self.map.insert(sand, Tile::Sand);
            break;
        }

        Ok(())
    }
}

mod parse {

    use std::{collections::BTreeMap, hash::Hash, str::FromStr};

    use anyhow::Context;

    use gxhash::GxHashMap;
    use itertools::Itertools;
    use tinyvec::{tiny_vec, TinyVec};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Point {
        pub x: u32,
        pub y: u32,
    }

    impl Hash for Point {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            // manual perfect hash, not secure
            state.write_u64(u64::from(self.x) << 32 | u64::from(self.y));
        }
    }

    impl Point {
        fn path_to(&self, other: Self) -> Box<dyn Iterator<Item = Self> + '_> {
            let dx = i64::from(other.x) - i64::from(self.x);
            let dy = i64::from(other.y) - i64::from(self.y);

            match (dy, dx) {
                // left
                (0, i64::MIN..=-1) => Box::new((0..=dx.abs()).rev().map(|it| Self {
                    x: (i64::from(self.x) - it).try_into().unwrap(),
                    y: self.y,
                })),
                // right
                (0, 1..=i64::MAX) => Box::new((0..=dx).map(|it| Self {
                    x: (i64::from(self.x) + it).try_into().unwrap(),
                    y: self.y,
                })),
                // up
                (i64::MIN..=-1, 0) => Box::new((0..=dy.abs()).map(|dy| Self {
                    x: self.x,
                    y: (i64::from(self.y) - dy).try_into().unwrap(),
                })),
                // down
                (1..=i64::MAX, 0) => Box::new((0..=dy).map(|dy| Self {
                    x: self.x,
                    y: (i64::from(self.y) + dy).try_into().unwrap(),
                })),

                _ => unreachable!(),
            }
        }
    }

    impl FromStr for Point {
        type Err = anyhow::Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (l, r) = s.split_once(',').context("invalid point: no comma")?;
            Ok(Self {
                x: l.parse()?,
                y: r.parse()?,
            })
        }
    }

    pub enum Tile {
        Empty,
        Rock,
        Sand,
    }

    impl std::fmt::Debug for Tile {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Self::Empty => '.',
                    Self::Rock => '#',
                    Self::Sand => 'o',
                }
            )
        }
    }

    pub type Map = GxHashMap<Point, Tile>;

    /// returns `(Map(Point => Tile), max_y key of Map)`
    pub(super) fn part1(data: &str) -> (Map, u32) {
        let paths = data.lines().map(|line| {
            line.split(" -> ")
                .map(str::parse::<Point>)
                .map(Result::unwrap)
        });

        let mut map = GxHashMap::default();

        let mut horizontal_map: GxHashMap<u32, TinyVec<[[u32; 2]; 4]>> = GxHashMap::default();

        for connected_path in paths {
            for (from, to) in connected_path.tuple_windows() {
                map.extend(from.path_to(to).map(|it| (it, Tile::Rock)));

                if from.y == to.y {
                    let y = [from.x.min(to.x), from.x.max(to.x)];

                    horizontal_map
                        .entry(from.y)
                        .and_modify(|it| it.push(y))
                        .or_insert(tiny_vec!([[u32; 2]; 4] => y));
                }
            }
        }

        // dbg!(horizontal_map);

        let max_y = map.keys().map(|Point { x: _, y }| *y).max().unwrap();

        (map, max_y)
    }
}
