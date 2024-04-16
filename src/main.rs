use std::{fmt::Debug, time::Instant, fmt};

fn main() {
    time_dbg("day3part1", yr2022_day3::part1);
    time_dbg("day3part2", yr2022_day3::part2);

    time_dbg("day4part1", yr2022_day4::part1).unwrap();
    time_dbg("day4part2", yr2022_day4::part2).unwrap();

    time_dbg("day7part1", yr2022_day7::part1).unwrap();
    time_dbg("day7part2", yr2022_day7::part2).unwrap();

    time_dbg("day8part1", yr2022_day8::part1).unwrap();
    time_dbg("day8part1nd", yr2022_day8::part1nd).unwrap();
    time_dbg("day8part2",yr2022_day8::part2);
}

fn time_dbg<R: Debug>(label: impl fmt::Display, f: impl Fn() -> R) -> R {
    let time = Instant::now();
    let result = f();
    eprintln!(
        "{label} result: {result:?}, elapsed: {:?}",
        time.elapsed()
    );
    result
}


mod yr2022_day3;
mod yr2022_day4;
mod yr2022_day7;
mod yr2022_day8;
