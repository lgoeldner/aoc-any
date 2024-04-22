#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::missing_errors_doc,
    missing_docs,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::cast_possible_truncation,
    clippy::reversed_empty_ranges
)]

use core::time;
use std::fmt::{Debug, Display};
use std::time::Instant;

use rayon::iter::ParallelBridge;
use rayon::prelude::*;

pub use types::*;

mod get_input;

pub mod types {
    use core::time;
    use std::fmt::{self, Debug, Display};

    use cli_table::{format::Justify, Color, Table, WithTitle};
    use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

	use anyhow::anyhow;

    use crate::{get_input::InputCache, time_bench_solution};

    pub type SolutionFn = fn(&str) -> ProblemResult;

    pub struct AocRuntime {
        pub input_cache: InputCache,
    }

    impl AocRuntime {
        /// Creates a new `AocRuntime`
        ///
        /// # Errors
        /// - `InputCache` fails to build
        ///     => dotenv fails to load
        pub fn new() -> anyhow::Result<Self> {
            Ok(Self {
                input_cache: InputCache::new()?,
            })
        }

        pub fn run(&mut self, days: &'static [Solution]) -> anyhow::Result<()> {
            if let Some(usr_query) = std::env::args().nth(1) {
                let matcher = SkimMatcherV2::default();

                let matched_benches = days
                    .iter()
                    .flat_map(get_names)
                    // if the name is matched, return only the name
                    .filter_map(|name| matcher.fuzzy_match(&name.0, &usr_query).map(|_| name))
                    .collect::<Vec<_>>();

                if matched_benches.is_empty() {
                    anyhow::bail!("No Matches found!");
                }

                let result = matched_benches
                    .into_iter()
                    .map(|(label, f, info)| {
                        let input = self.input_cache.get(info).unwrap();
                        time_bench_solution(&input, info, label, f)
                    })
                    .collect::<Vec<_>>();

                return cli_table::print_stdout(result.with_title())
                    .map_err(|_| anyhow!("Failed to print table"));
            }

            let runs = crate::bench_solutions(days, self);
            cli_table::print_stdout(runs.with_title()).map_err(|_| anyhow!("Failed to print table"))
        }
    }

    /// formats the names for each function available for the Solution, returns a Vec of (name, fn)
    // avoids type golfing
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn get_names(
        inp: &'static Solution,
    ) -> Vec<(String, crate::SolutionFn, &'static crate::Info)> {
        let x = inp.part2.map_or_else(
            || vec![("part1", inp.part1)],
            |part2| vec![("part2", part2), ("part1", inp.part1)],
        );

        let others = inp.other.iter().map(|(a, b, _)| (*a, *b));

        // x.into_iter()
        //     .chain(others)

        others
            .into_iter()
            .chain(x)
            .map(|s| {
                (
                    format!("{} day{:0>2}: {}", inp.info.year, inp.info.day, s.0),
                    s.1,
                    &inp.info,
                )
            })
            .collect::<Vec<_>>()
    }

    pub struct Solution {
        pub part1: fn(&str) -> ProblemResult,
        pub part2: Option<fn(&str) -> ProblemResult>,
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

    #[derive(Debug)]
    pub struct Info {
        pub name: &'static str,
        pub day: u8,
        pub year: u16,
        pub bench: BenchTimes,
    }

    #[derive(Debug)]
    pub enum BenchTimes {
        None,
        Default,
        Once,
        Many(usize),
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum Part {
        One,
        Two,
        Other(String),
    }

    #[derive(Table)]
    #[non_exhaustive]
    pub struct BenchRun {
        #[table(title = "year", justify = "Justify::Right")]
        pub year: u16,
        #[table(title = "day")]
        pub day: u8,

        #[table(title = "name")]
        pub name: &'static str,
        #[table(title = "label")]
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

    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn _display_times(inp: &usize) -> impl Display {
        format!("{inp}x")
    }

    pub trait DateProvider {
        fn get_datetuple(&self) -> (u16, u8);
        fn day(&self) -> u8;
        fn year(&self) -> u16;
    }

    impl DateProvider for Solution {
        fn get_datetuple(&self) -> (u16, u8) {
            (self.info.year, self.info.day)
        }

        fn day(&self) -> u8 {
            self.info.day
        }

        fn year(&self) -> u16 {
            self.info.year
        }
    }

    impl DateProvider for Info {
        fn get_datetuple(&self) -> (u16, u8) {
            (self.year, self.day)
        }

        fn day(&self) -> u8 {
            self.day
        }

        fn year(&self) -> u16 {
            self.year
        }
    }

    impl Display for ProblemResult {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Number(n) => write!(f, "{n}"),
                Self::Other(any) => write!(f, "{any:?}"),
            }
        }
    }

    macro_rules! impl_from_problem_num {
    ( $($t:ty),* ) => {
        $(
        impl From<$t> for ProblemResult {
            fn from(value: $t) -> Self {
                ProblemResult::Number(value.try_into().unwrap())
            }
        }
        )*
    };
}

    impl_from_problem_num! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }
}

pub fn time_dbg<R: Debug>(label: impl Display, f: impl Fn() -> R) -> R {
    let time = Instant::now();
    let result = f();
    eprintln!("{label} result: {result:?}, elapsed: {:?}", time.elapsed());
    result
}

pub fn time_bench<const TIMES: usize, R>(label: impl Display, f: impl Fn() -> R + Send + Sync) -> R
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

pub fn bench_solutions(days: &'static [Solution], runtime: &mut AocRuntime) -> Vec<BenchRun> {
    let mut runs = Vec::new();

    for day in days.iter().rev() {
        let input = &runtime.input_cache.get(day).unwrap();
        // part1
        runs.push(time_bench_solution(
            input,
            &day.info,
            "part1".to_owned(),
            day.part1,
        ));

        // part2
        if let Some(part2) = day.part2 {
            runs.push(time_bench_solution(
                input,
                &day.info,
                "part2".to_owned(),
                part2,
            ));
        }

        // others
        let iter = day.other.iter().filter_map(|(label, f, run)| {
            crate::some_if! {
                matches!(run, Run::Yes) => time_bench_solution(input, &day.info, (*label).to_string(), f)
            }
        });

        runs.extend(iter);
    }
    runs
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
    input: &str,
    info: &Info,
    label: String,
    f: impl Fn(&str) -> ProblemResult + Send + Sync,
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
        let output = f(input);
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
                core::hint::black_box(f(input));
                time.elapsed()
            })
            .collect::<Vec<_>>()
    } else {
        (0..times)
            .map(|_| {
                let time = Instant::now();
                core::hint::black_box(f(input));
                time.elapsed()
            })
            .collect::<Vec<_>>()
    };

    let alt_start = Instant::now();
    let output = f(input);
    let avg_time = runs
        .iter()
        .sum::<time::Duration>()
        .checked_div(runs.len() as u32)
        .unwrap_or_else(|| alt_start.elapsed());

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

pub fn time_bench_runt<R>(label: impl Display, times: usize, f: impl Fn() -> R + Send + Sync) -> R
where
    R: Send + Sync + Debug,
{
    let start = Instant::now();
    let benched = (0..times)
        .par_bridge()
        .map(|_| {
            let time = Instant::now();
            let _ = f();
            time.elapsed()
        })
        .collect::<Vec<_>>();

    eprintln!(
        "Over {times} Runs, average time Was: {:?}, elapsed runtime in function: {:?}, actual elapsed: {:?}",
        benched.iter().sum::<time::Duration>() / times as u32,
        benched.iter().sum::<time::Duration>(),
		start.elapsed()
    );

    let result = f();
    eprintln!("{label} resulted in {result:?}",);
    result
}

/// utility function
pub fn zip<A: Iterator, B: Iterator>(a: A, b: B) -> impl Iterator<Item = (A::Item, B::Item)> {
    a.zip(b)
}

pub mod set_trait {
    use std::collections::{BTreeSet, HashSet};
    use std::hash::{BuildHasher, Hash};

    pub trait Set<T> {
        fn insert(&mut self, item: T);
        fn len(&self) -> usize;

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    impl<T: Eq + Hash, S: BuildHasher> Set<T> for HashSet<T, S> {
        fn insert(&mut self, item: T) {
            Self::insert(self, item);
        }

        fn len(&self) -> usize {
            self.len()
        }
    }

    impl<T: Eq + Hash + Ord> Set<T> for BTreeSet<T> {
        fn insert(&mut self, item: T) {
            Self::insert(self, item);
        }

        fn len(&self) -> usize {
            Self::len(self)
        }
    }
}

pub mod map_trait {
    use std::collections::{BTreeMap, HashMap};
    use std::hash::{BuildHasher, Hash};

    pub trait Map<K, V> {
        fn insert(&mut self, key: K, value: V);
        fn len(&self) -> usize;

        fn get(&self, key: &K) -> Option<&V>;

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    impl<K: Eq + Hash, V, S: BuildHasher> Map<K, V> for HashMap<K, V, S> {
        fn insert(&mut self, key: K, value: V) {
            Self::insert(self, key, value);
        }

        fn len(&self) -> usize {
            self.len()
        }

        fn get(&self, key: &K) -> Option<&V> {
            Self::get(self, key)
        }
    }

    impl<K: Eq + Hash + Ord, V> Map<K, V> for BTreeMap<K, V> {
        fn insert(&mut self, key: K, value: V) {
            Self::insert(self, key, value);
        }

        fn len(&self) -> usize {
            Self::len(self)
        }

        fn get(&self, key: &K) -> Option<&V> {
            Self::get(self, key)
        }
    }
}
