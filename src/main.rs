#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::reversed_empty_ranges)]
#![allow(clippy::cast_possible_truncation)]
use core::time;
use std::{fmt, fmt::Debug, time::Instant};

fn main() {
    time_dbg("day3part1", yr2022_day3::part1);
    time_dbg("day3part2", yr2022_day3::part2);

    time_dbg("day4part1", yr2022_day4::part1).unwrap();
    time_dbg("day4part2", yr2022_day4::part2).unwrap();

    time_dbg("day7part1", yr2022_day7::part1).unwrap();
    time_dbg("day7part2", yr2022_day7::part2).unwrap();

    time_dbg("day8part1", yr2022_day8::part1);
    time_bench::<1000, _>("day8part1nd", yr2022_day8::part1nd);
    time_bench::<1000, _>("day8part2", yr2022_day8::part2);
}

fn time_dbg<R: Debug>(label: impl fmt::Display, f: impl Fn() -> R) -> R {
    let time = Instant::now();
    let result = f();
    eprintln!("{label} result: {result:?}, elapsed: {:?}", time.elapsed());
    result
}

fn time_bench<const TIMES: usize, R: Debug>(label: impl fmt::Display, f: impl Fn() -> R) -> R {
	let start = Instant::now();
    let times = (0..TIMES)
        .map(|_| {
            let time = Instant::now();
            let _ = f();
            time.elapsed()
        })
        .collect::<Vec<_>>();

    // let mut times = Vec::with_capacity(TIMES);
    // for _ in 0..TIMES - 1 {
    //     let time = Instant::now();
    //     let _ = f();
    //     times.push(time.elapsed());
    // }

    eprintln!(
        "Over {TIMES} Runs, average time Was: {:?}, elapsed: {:?}",
        times.iter().sum::<time::Duration>() / TIMES as u32,
        start.elapsed()
    );
    let result = f();
    eprintln!("{label} resulted in {result:?}",);
    result
}

mod yr2022_day3;
mod yr2022_day4;
mod yr2022_day7;
mod yr2022_day8;
