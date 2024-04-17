#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::reversed_empty_ranges)]
#![allow(clippy::cast_possible_truncation)]

use aoc_any::*;
use cli_table::WithTitle;

const DAYS: &[&'static Solution] = &[&yr2022_day7::SOLUTION, &yr2022_day8::SOLUTION];

fn main() {
    old_runs();

    let mut runs = Vec::new();
    for day in DAYS.iter().rev() {
        runs.push(time_bench_solution(&day.info, "part1", day.part1));
        if let Some(part2) = day.part2 {
            runs.push(time_bench_solution(&day.info, "part2", part2));
        }

        let iter = day
            .other
            .iter()
            .map(|(label, f, run)| {
                some_if! {
                    !matches!(run, Run::No) => time_bench_solution(&day.info, label, f)
                }
            })
            .filter_map(|opt| opt);

        runs.extend(iter)
    }
	// runs.sort_by_key(|c| c.day);
	cli_table::print_stdout(runs.with_title()).unwrap();
}

fn old_runs() {
    time_dbg("day3part1", yr2022_day3::part1);
    time_dbg("day3part2", yr2022_day3::part2);

    time_dbg("day4part1", yr2022_day4::part1).unwrap();
    time_dbg("day4part2", yr2022_day4::part2).unwrap();
}

mod template;
mod yr2022_day3;
mod yr2022_day4;
mod yr2022_day7;
mod yr2022_day8;
mod yr2022_day9;
