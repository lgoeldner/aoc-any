
use aoc_any::{BenchTimes, Info, Solution};
use gxhash::GxHashMap;

use self::parse::Point;



pub const SOLUTION: Solution = Solution {
    info: Info {
        name: "Regolith Reservoir",
        day: 14,
        year: 2022,
        bench: BenchTimes::None,
    },
    part1: |_data| part1(EXAMPLE).into(),
    part2: None,
    other: &[],
};

const EXAMPLE: &str = include_str!("../inputs/2022-day14-test.txt");

fn part1(data: &str) -> u32 {
    let (map, deepest) = parse::part1(data);

    todo!()
}

fn add_sand(map: &mut GxHashMap<Point, u32>, deepest: u32) -> bool {
	
	let mut sand = Point {
		x: 500,
		y: 0
	};

	loop {
		// try down

		// then try down-left

		// then try down-right

		// else rest


	}


	todo!()
}

mod parse {

    use std::{hash::Hash, str::FromStr};

    use anyhow::Context;

    use gxhash::GxHashMap;
    use itertools::Itertools;

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

    /// returns `(Map(Point => Tile), max_y key of Map)`
    pub(super) fn part1(data: &str) -> (GxHashMap<Point, Tile>, u32) {
        let paths = data.lines().map(|line| {
            line.split(" -> ")
                .map(str::parse::<Point>)
                .map(std::result::Result::unwrap)
        });

        let mut map = GxHashMap::default();

        for connected_path in paths {
            for (from, to) in connected_path.tuple_windows() {
                map.extend(from.path_to(to).map(|it| (it, Tile::Rock)));
            }
        }

        let max_y = map.keys().map(|Point { x: _, y }| *y).max().unwrap();

        (map, max_y)
    }
}
