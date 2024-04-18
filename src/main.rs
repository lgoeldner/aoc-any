#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::reversed_empty_ranges)]
#![allow(clippy::cast_possible_truncation)]

use aoc_any::Solution;
use cli_table::WithTitle;

const DAYS: &[&Solution] = &[
    &yr2022_day3::SOLUTION,
    &yr2022_day4::SOLUTION,
    &yr2022_day7::SOLUTION,
    &yr2022_day8::SOLUTION,
];

fn main() {
    let runs = aoc_any::bench_solutions(DAYS);
    cli_table::print_stdout(runs.with_title()).unwrap();
}

mod template;
mod yr2022_day3;
mod yr2022_day4;
mod yr2022_day7;
mod yr2022_day8;
mod yr2022_day9;
