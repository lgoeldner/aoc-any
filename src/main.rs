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
];

fn main() -> anyhow::Result<()> {
    let mut runtime = AocRuntime::new()?;

    // if let Some(usr_query) = std::env::args().nth(1) {
    //     let matcher = SkimMatcherV2::default();

    //     let matched_benches = DAYS
    //         .iter()
    //         .flat_map(get_names)
    //         // if the name is matched, return only the name
    //         .filter_map(|name| matcher.fuzzy_match(&name.0, &usr_query).map(|_| name))
    //         .collect::<Vec<_>>();

    //     if matched_benches.is_empty() {
    //         anyhow::bail!("No Matches found!");
    //     }

    //     let result = matched_benches
    //         .into_iter()
    //         .map(|(label, f, info)| {
    //             let input = runtime.input_cache.get(info).unwrap();
    //             time_bench_solution(&input, info, label, f)
    //         })
    //         .collect::<Vec<_>>();

    //     return cli_table::print_stdout(result.with_title())
    //         .map_err(|_| anyhow!("Failed to print table"));
    // }

    // let runs = aoc_any::bench_solutions(DAYS, &mut runtime);
    // cli_table::print_stdout(runs.with_title()).map_err(|_| anyhow!("Failed to print table"))

    runtime.run(DAYS)
}

mod template;
mod yr2022_day1;
mod yr2022_day10;
mod yr2022_day11;
mod yr2022_day12;
mod yr2022_day3;
mod yr2022_day4;
mod yr2022_day7;
mod yr2022_day8;
mod yr2022_day9;
