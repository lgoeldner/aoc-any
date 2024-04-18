use cli_table::Color;
use cli_table::Table;
use core::time;
use rayon::iter::ParallelBridge;
use rayon::prelude::*;
use std::fmt::{Debug, Display};
use std::{fmt, time::Instant};

pub type SolutionFn = fn() -> ProblemResult;

pub struct Solution {
    pub part1: fn() -> ProblemResult,
    pub part2: Option<fn() -> ProblemResult>,
    pub info: Info,
    pub other: &'static [(&'static str, SolutionFn, Run)],
}

pub enum Run {
    No,
    Yes,
}

#[derive(Debug)]
pub enum ProblemResult {
    Number(i64),
    Other(Box<dyn Debug + Send + Sync>),
}

impl From<u32> for ProblemResult {
    fn from(value: u32) -> Self {
        ProblemResult::Number(i64::from(value))
    }
}

impl std::fmt::Display for ProblemResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProblemResult::Number(n) => write!(f, "{n}"),
            ProblemResult::Other(any) => write!(f, "{any:?}"),
        }
    }
}

#[derive(Debug)]
pub struct Info {
    pub name: &'static str,
    pub day: u8,
    pub year: u16,
    /// default if None, else number of times to run. 0 if only run
    pub bench: BenchTimes,
    // enable if part 2 should be run
}

#[derive(Debug)]
pub enum BenchTimes {
    None,
    Default,
    Once,
    Many(usize),
}

pub fn time_dbg<R: Debug>(label: impl fmt::Display, f: impl Fn() -> R) -> R {
    let time = Instant::now();
    let result = f();
    eprintln!("{label} result: {result:?}, elapsed: {:?}", time.elapsed());
    result
}

pub fn time_bench<const TIMES: usize, R>(
    label: impl fmt::Display,
    f: impl Fn() -> R + Send + Sync,
) -> R
where
    R: Send + Sync + Debug,
{
    let start = Instant::now();
    let times = (0..TIMES)
        .par_bridge()
        .map(|_| {
            let time = Instant::now();
            let _ = f();
            time.elapsed()
        })
        .collect::<Vec<_>>();

    eprintln!(
        "Over {TIMES} Runs, average time Was: {:?}, elapsed runtime in function: {:?}, actual elapsed: {:?}",
        times.iter().sum::<time::Duration>() / TIMES as u32,
        times.iter().sum::<time::Duration>(),
		start.elapsed()
    );

    let result = f();
    eprintln!("{label} resulted in {result:?}",);
    result
}

// pub struct BenchRun {
//     pub output: ProblemResult,
//     pub avg_time: time::Duration,
//     pub elapsed: time::Duration,
//     pub times: usize,
// }

#[derive(Table)]
#[non_exhaustive]
pub struct BenchRun {
    #[table(title = "year")]
    pub year: u16,
    #[table(title = "day")]
    pub day: u8,

    #[table(title = "name", bold)]
    pub name: &'static str,
    #[table(title = "label", bold)]
    pub label: String,

    #[table(display_fn = "display_duration", title = "avg", color = "Color::Cyan")]
    pub avg_time: time::Duration,
    #[table(display_fn = "display_duration", title = "elapsed", skip)]
    pub elapsed: time::Duration,
    #[table(title = "Benchmarked", display_fn = "_display_times", skip)]
    pub times: usize,

    #[table(title = "result", color = "Color::Green")]
    pub output: ProblemResult,
}

fn display_duration(inp: &time::Duration) -> impl Display {
    format!("{inp:?}")
}

fn _display_times(inp: &usize) -> impl Display {
    format!("{inp}x")
}

#[macro_export]
macro_rules! some_if {
	{$y:expr => $then:expr} => {
		if $y {
			Some($then)
		} else {
			None
		}
	};
}

pub fn time_bench_solution(
    info: &Info,
    label: String,
    f: impl Fn() -> ProblemResult + Send + Sync,
) -> BenchRun {
    let times = match info.bench {
        BenchTimes::None => 0,
        BenchTimes::Many(n) => n,
        BenchTimes::Default => 100,
        BenchTimes::Once => 1,
    };

    if label.contains("heavy") {
        eprintln!("Running heavy benchmark");
        let start = Instant::now();
        let output = f();
        return BenchRun {
            avg_time: start.elapsed(),
            elapsed: start.elapsed(),
            times: 1,
            output,
            day: info.day,
            year: info.year,
            name: info.name,
            label,
        };
    }

    let start = Instant::now();

    let runs = if times > 90 {
        (0..times)
            .par_bridge()
            .map(|_| {
                let time = Instant::now();
                let _ = f();
                time.elapsed()
            })
            .collect::<Vec<_>>()
    } else {
        (0..times)
            .map(|_| {
                let time = Instant::now();
                let _ = f();
                time.elapsed()
            })
            .collect::<Vec<_>>()
    };

    let output = f();
    let avg_time = runs.iter().sum::<time::Duration>() / runs.len() as u32;

    BenchRun {
        output,
        avg_time,
        elapsed: start.elapsed(),
        times,
        day: info.day,
        year: info.year,
        name: info.name,
        label,
    }
}

pub fn time_bench_runt<R>(
    label: impl fmt::Display,
    times: usize,
    f: impl Fn() -> R + Send + Sync,
) -> R
where
    R: Send + Sync + Debug,
{
    let start = Instant::now();
    let timed = (0..times)
        .par_bridge()
        .map(|_| {
            let time = Instant::now();
            let _ = f();
            time.elapsed()
        })
        .collect::<Vec<_>>();

    eprintln!(
        "Over {times} Runs, average time Was: {:?}, elapsed runtime in function: {:?}, actual elapsed: {:?}",
        timed.iter().sum::<time::Duration>() / times as u32,
        timed.iter().sum::<time::Duration>(),
		start.elapsed()
    );

    let result = f();
    eprintln!("{label} resulted in {result:?}",);
    result
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Part {
    One,
    Two,
    Other(String),
}

pub fn bench_solutions(days: &'static [&'static Solution]) -> Vec<BenchRun> {
    let mut runs = Vec::new();
    for day in days.iter().rev() {
        runs.push(time_bench_solution(&day.info, "part1".to_owned(), day.part1));
        if let Some(part2) = day.part2 {
            runs.push(time_bench_solution(&day.info, "part2".to_owned(), part2));
        }

        let iter = day.other.iter().filter_map(|(label, f, run)| {
            some_if! {
                matches!(run, Run::Yes) => time_bench_solution(&day.info, label.to_string(), f)
            }
        });

        runs.extend(iter)
    }
    runs
}
