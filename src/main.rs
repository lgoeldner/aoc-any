#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::reversed_empty_ranges)]
#![allow(clippy::cast_possible_truncation)]


use aoc_any::{time_bench_solution, Solution};
use cli_table::WithTitle;

const DAYS: &[&Solution] = &[
    &yr2022_day3::SOLUTION,
    &yr2022_day4::SOLUTION,
    &yr2022_day7::SOLUTION,
    &yr2022_day8::SOLUTION,
];

fn main() -> Result<(), std::io::Error> {
    if let Some(usr_query) = std::env::args().nth(1) {
        use fuzzy_matcher::skim::SkimMatcherV2;
        use fuzzy_matcher::FuzzyMatcher;

        let matcher = SkimMatcherV2::default();

        let matched_benches = DAYS
            .iter()
            .flat_map(get_names)
            .filter_map(|name| matcher.fuzzy_match(&name.0, &usr_query).map(|_| name))
            .collect::<Vec<_>>();

        if matched_benches.is_empty() {
            // eprintln!("Not found!");
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No Matches found!",
            ));
        }

        let result = matched_benches
            .into_iter()
            .map(|(label, f, info)| time_bench_solution(info, label, f))
            .collect::<Vec<_>>();

        return cli_table::print_stdout(result.with_title());
    }

    let runs = aoc_any::bench_solutions(DAYS);
    cli_table::print_stdout(runs.with_title())
}

/// formats the names for each function available for the Solution, returns a Vec of (name, fn)
#[allow(clippy::trivially_copy_pass_by_ref)]
fn get_names(    
    inp: &&'static Solution,
) -> Vec<(String, aoc_any::SolutionFn, &'static aoc_any::Info)> {
    let x = inp.part2.map_or_else(
        || vec![("part1", inp.part1)],
        |part2| vec![("part1", inp.part1), ("part2", part2)],
    );

    let others = inp.other.iter().map(|(a, b, _)| (*a, *b));

    x.into_iter()
        .chain(others)
        .map(|s| {
            (
                format!(
                    "{} day{:0>2}: {}-{}",
                    inp.info.year, inp.info.day, inp.info.name, s.0
                ),
                s.1,
                &inp.info,
            )
        })
        .collect::<Vec<_>>()
}

mod template;
mod yr2022_day3;
mod yr2022_day4;
mod yr2022_day7;
mod yr2022_day8;
mod yr2022_day9;
