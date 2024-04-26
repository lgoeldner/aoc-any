#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::reversed_empty_ranges)]
#![allow(clippy::cast_possible_truncation)]

use aoc_any::types::{AocRuntime, Solution};

const DAYS: &[Solution] = &[
    yr2022_day1::SOLUTION,
    yr2022_day3::SOLUTION,
    yr2022_day4::SOLUTION,
    yr2022_day7::SOLUTION,
    yr2022_day8::SOLUTION,
    yr2022_day9::SOLUTION,
    yr2022_day10::SOLUTION,
    yr2022_day11::SOLUTION,
    yr2022_day12::SOLUTION,
    yr2022_day13::SOLUTION,
];

fn main() -> anyhow::Result<()> {
    AocRuntime::new()?.run(DAYS)
}

mod template;
mod yr2022_day1;
mod yr2022_day10;
mod yr2022_day11;
mod yr2022_day12;
mod yr2022_day13;
mod yr2022_day3;
mod yr2022_day4;
mod yr2022_day7;
mod yr2022_day8;
mod yr2022_day9;
