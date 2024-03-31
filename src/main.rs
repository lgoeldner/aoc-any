use std::time::Instant;

fn main() {
    // timeit(yr2022_day3::part1);
    // timeit(yr2022_day3::part2);

    dbg!(timeit(yr2022_day4::part1).unwrap());
}

fn timeit<F, R>(f: F) -> R
where
    F: Fn() -> R,
{
    let time = Instant::now();
    let result = f();
    eprintln!("{:?}", time.elapsed());
    result
}

mod yr2022_day3;
mod yr2022_day4;
