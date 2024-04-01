use std::{fmt::Debug, time::Instant};

fn main() {
    timeit(yr2022_day3::part1);
    timeit(yr2022_day3::part2);

    timeit(yr2022_day4::part1).unwrap();
    timeit(yr2022_day4::part2).unwrap();

    timeit(yr2022_day7::part1).unwrap();
}

fn timeit<F, R>(f: F) -> R
where
    F: Fn() -> R,
    R: Debug
{
    let time = Instant::now();
    let result = f();
    eprintln!("result: {result:?}, elapsed: {:?}", time.elapsed());
    result
}

mod yr2022_day3;
mod yr2022_day4;
mod yr2022_day7;
