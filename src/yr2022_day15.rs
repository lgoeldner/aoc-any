use anyhow::anyhow;
use aoc_any::{BenchTimes, Info, Solution};

pub const SOLUTION: Solution = Solution {
    info: Info {
        day: 15,
        name: "Beacon Exclusion Zone",
        year: 2022,
        bench: BenchTimes::None,
    },
    part1: |_data| {
        part1(parse(EXAMPLE)).into()
    },
    part2: None,
    other: &[],
};

mod math {
    use crate::yr2022_day15::Point;

    use super::Line;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Range {
        from: i64,
        to: i64,
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
        pub fn width_at<const HEIGHT: i64>(&self) -> Option<Range> {
            let radius = self.sensor.manhattan_distance(&self.closest_beacon);
            let height_diff = (HEIGHT - self.sensor.y).abs();

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
        assert_eq!(
            Line {
                sensor: Point { x: 8, y: 7 },
                closest_beacon: Point { x: 2, y: 10 },
            }
            .width_at::<10>(),
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

type Parsed = Vec<Line>;

#[derive(Debug)]
struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug)]
struct Line {
    pub sensor: Point,
    pub closest_beacon: Point,
}

fn part1(data: Parsed) -> u32 {
    let ranges = data.iter().filter_map(Line::width_at::<10>).collect::<Vec<_>>();
    dbg!(&ranges);

    todo!()
}

fn parse(data: &str) -> anyhow::Result<Parsed> {
    data.lines()
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
        .map_err(|it| anyhow!("failed to parse {it}"))
}
