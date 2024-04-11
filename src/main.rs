use std::{fmt::Debug, time::Instant};

fn main() {
    time_dbg(yr2022_day3::part1);
    time_dbg(yr2022_day3::part2);

    time_dbg(yr2022_day4::part1).unwrap();
    time_dbg(yr2022_day4::part2).unwrap();

    time_dbg(yr2022_day7::part1).unwrap();

    time_dbg(yr2022_day7::part1).unwrap();
    time_dbg(yr2022_day7::part2).unwrap();

    time_dbg(yr2022_day8::part1).unwrap();
}

fn time_dbg<R: Debug>(f: impl Fn() -> R) -> R {
    let time = Instant::now();
    let result = f();
    eprintln!("result: {result:?}, elapsed: {:?}", time.elapsed());
    result
}

mod yr2022_day3;
mod yr2022_day4;
mod yr2022_day7;
mod yr2022_day8;
