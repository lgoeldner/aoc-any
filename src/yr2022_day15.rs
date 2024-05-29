use anyhow::anyhow;
use itertools::Itertools;
use aoc_any::{BenchTimes, Info, Solution};
use math::Range;

pub const SOLUTION: Solution = Solution {
    info: Info {
        day: 15,
        name: "Beacon Exclusion Zone",
        year: 2022,
        bench: BenchTimes::None,
    },
    part1: |data| part1(parse(if TEST { EXAMPLE } else { data }).unwrap()).into(),
    part2: None,
    other: &[],
};

const TEST: bool = false;
const HEIGHT: i64 = if TEST { 10 } else { 2_000_000 };

mod math {

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Range {
        pub from: i64,
        pub to: i64,
    }

    impl Range {
        pub const fn intersects(&self, other: &Self) -> bool {
            // if sorted in non-descending order
            if self.from >= other.from {
                self.to >= other.from
            } else {
                other.to >= self.from
            }
        }

        pub const fn spanned(&self) -> i64 {
            (self.to - self.from).abs() + 1
        }
    }

    impl From<core::ops::Range<i64>> for Range {
        fn from(range: core::ops::Range<i64>) -> Self {
            Self {
                from: range.start,
                to: range.end,
            }
        }
    }

    impl super::Point {
        pub const fn manhattan_distance(&self, other: &Self) -> i64 {
            (self.x - other.x).abs() + (self.y - other.y).abs()
        }
    }

    impl super::Line {
        pub const fn new(sensor: super::Point, closest_beacon: super::Point) -> Self {
            Self {
                sensor,
                closest_beacon,
            }
        }

        pub fn width_at_height(&self) -> Option<Range> {
            let radius = self.sensor.manhattan_distance(&self.closest_beacon);
            let height_diff = (super::HEIGHT - self.sensor.y).abs();

            (height_diff <= radius).then(|| {
                let half_width = radius - height_diff;

                Range {
                    from: self.sensor.x - half_width,
                    to: self.sensor.x + half_width,
                }
            })
        }
    }

    #[test]
    pub fn test_width_at_height() {
        use super::{Line, Point};

        assert_eq!(
            Line {
                sensor: Point { x: 8, y: 7 },
                closest_beacon: Point { x: 2, y: 10 },
            }
            .width_at_height(),
            Some((2..14).into())
        );
    }
}

#[rustfmt::skip]
const EXAMPLE: &str =
   "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
    Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
    Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
    Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
    Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
    Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
    Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
    Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
    Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
    Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
    Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
    Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
    Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
    Sensor at x=20, y=1: closest beacon is at x=15, y=3";

type Parsed = (Vec<Line>, u32);

#[derive(Debug, Eq, PartialEq)]
struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug)]
struct Line {
    pub sensor: Point,
    pub closest_beacon: Point,
}

#[test]
fn test_part1() {
    let data = [1..3, 2..4, 2..5, 4..5, 5..5, 7..8, 9..10]
        .into_iter()
        .map(core::convert::Into::into)
        .collect::<Vec<math::Range>>();

    assert_eq!(
        flatten_spanned_len(dbg!(data[..data.len() - 3].to_vec())),
        5
    );
    assert_eq!(flatten_spanned_len(data), 9);
}

fn part1((data, sub): Parsed) -> i64 {
    fn cmp<T: std::cmp::Ord>(a: &T, b: &T) -> std::cmp::Ordering {
        a.cmp(b)
    }

    let mut ranges = data
        .iter()
        .filter_map(Line::width_at_height)
        .collect::<Vec<_>>();

    ranges.sort_by(|a, b| cmp(&a.from, &b.from).then(cmp(&a.to, &b.to)));

    flatten_spanned_len(ranges) - i64::from(sub)
}

fn flatten_spanned_len(ranges: Vec<Range>) -> i64 {
    fn min<T: Ord>(a: T, b: T) -> T {
        a.min(b)
    }

    fn max<T: Ord>(a: T, b: T) -> T {
        a.max(b)
    }

    let mut iter = ranges.into_iter();
    let mut sum = 0;
    loop {
        let mut state = match iter.next() {
            None => break,
            Some(state) => state,
        };

        for el in iter.by_ref() {
            if !state.intersects(&el) {
                sum += state.spanned();
                continue;
            }

            state = Range {
                from: min(el.from, state.from),
                to: max(state.to, el.to),
            };
        }

        sum += state.spanned();
    }

    sum
}

fn parse(data: &str) -> anyhow::Result<Parsed> {
    let res: Vec<_> = data
        .lines()
        .map(|it| {
            sscanf::sscanf!(
                it,
                "Sensor at x={i64}, y={i64}: closest beacon is at x={i64}, y={i64}",
            )
            .map(|(x_sensor, y_sensor, x_beacon, y_beacon)| Line {
                sensor: Point {
                    x: x_sensor,
                    y: y_sensor,
                },
                closest_beacon: Point {
                    x: x_beacon,
                    y: y_beacon,
                },
            })
        })
        .collect::<Result<_, _>>()
        .map_err(|it| anyhow!("failed to parse {it}"))?;

    let x = res
        .iter()
        .filter_map(|it| (it.closest_beacon.y == HEIGHT).then_some(&it.closest_beacon))
        .dedup()
        .count();

    Ok((res, x.try_into()?))
}
