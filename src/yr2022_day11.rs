use std::cell::RefCell;
use std::convert::Into;

use once_cell::sync::Lazy;
use regex::Regex;

use aoc_any::{Info, Solution};

use crate::yr2022_day11::ops::Op;

pub const SOLUTION: Solution = Solution {
    info: Info {
        name: "Monkey in the Middle",
        day: 11,
        year: 2022,
        bench: aoc_any::BenchTimes::None,
    },
    part1: || part1(DATA).into(),
    part2: Some(|| part2(TEST_EXAMPLE).into()),
    other: &[],
};

const TEST_EXAMPLE: &str = include_str!("../inputs/2022-day11-test.txt");
const DATA: &str = include_str!("../inputs/2022-day11-inp.txt");

fn part1(data: &str) -> u64 {
    let parsed = parse(data);

    for _ in 0..20 {
        do_round(&parsed);
    }

    let mut parsed = parsed
        .into_iter()
        .map(|it| it.borrow().inspected)
        .collect::<Vec<_>>();
    parsed.sort_unstable();
    // parsed.reverse();

    // take the last two and multiply
    parsed.into_iter().rev().take(2).product()
}

fn part2(data: &str) -> u64 {
    let parsed = parse(data);

    for round_idx in 1..=10_000 {
        do_round2(&parsed);
        if round_idx == 20 || round_idx == 1000 {
            dbg!(&parsed
                .iter()
                .map(|it| {
                    let m = it.borrow();
                    (m.idx, m.inspected)
                })
                .collect::<Vec<_>>());
        }
    }

    let mut parsed = parsed
        .into_iter()
        .map(|it| it.borrow().inspected)
        .collect::<Vec<_>>();
    parsed.sort_unstable();
    dbg!(&parsed);
    // parsed.reverse();

    // take the last two and multiply
    parsed.into_iter().rev().take(2).product()
}

fn do_round2(parsed: &Vec<RefCell<Monkey>>) {
    let supermodulo: u64 = parsed.iter().map(|it| it.borrow().test.0).product();
    let mut monkey_items;
    for monkey in parsed {
        // take the items from the monkey
        monkey_items = std::mem::take(&mut monkey.borrow_mut().items);
        monkey.borrow_mut().inspected += monkey_items.len() as u64;
        // for each item, stored by its worrylevel:
        // perform the monkeys operation,
        // divide by three
        // move the item to the appropriate monkey
        for mut item_worrylevel in monkey_items {
            // monkey.borrow_mut().inspected += 1;
            let monkey = monkey.borrow();

            item_worrylevel %= supermodulo;

            let throw_to_monkey = if monkey.test.check(item_worrylevel) {
                monkey.true_target
            } else {
                monkey.false_target
            } as usize;

            parsed[throw_to_monkey]
                .borrow_mut()
                .items
                .push(item_worrylevel);
        }
    }
}

fn do_round(parsed: &Vec<RefCell<Monkey>>) {
    let mut monkey_items;
    for monkey in parsed {
        // take the items from the monkey
        monkey_items = std::mem::take(&mut monkey.borrow_mut().items);
        monkey.borrow_mut().inspected += monkey_items.len() as u64;
        // for each item, stored by its worrylevel:
        // perform the monkeys operation,
        // divide by three
        // move the item to the appropriate monkey
        for mut item_worrylevel in monkey_items {
            // monkey.borrow_mut().inspected += 1;
            let monkey = monkey.borrow();

            item_worrylevel = monkey.operation.perform(item_worrylevel);

            item_worrylevel /= 3;

            let throw_to_monkey = if monkey.test.check(item_worrylevel) {
                monkey.true_target
            } else {
                monkey.false_target
            } as usize;

            parsed[throw_to_monkey]
                .borrow_mut()
                .items
                .push(item_worrylevel);
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Test(u64);

impl Test {
    const fn check(self, and: u64) -> bool {
        and % self.0 == 0
    }
}

#[derive(Debug)]
struct Monkey {
    inspected: u64,
    idx: u8,
    items: Vec<u64>,
    operation: Op,
    test: Test,
    true_target: u8,
    false_target: u8,
}

fn parse(data: &str) -> Vec<RefCell<Monkey>> {
    data.split("\r\n\r\n")
        .map(parse_monkey)
        .map(RefCell::new)
        .collect()
}

fn parse_monkey(data: &str) -> Monkey {
    let mut lines = data.lines();

    let idx = {
        let line = lines.next().unwrap();
        line["Monkey ".len()..line.len() - 1].parse().unwrap()
    };

    let items = {
        let line = lines.next().unwrap();
        line["  Starting items: ".len()..]
            .split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect()
    };

    let operation = {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(old|\d{1,2}) ([+*]) (old|\d{1,2})").unwrap());

        let line = lines.next().unwrap();

        let (_, [lhs, op, rhs]) = RE
            .captures_iter(line)
            .map(|it| it.extract())
            .next()
            .unwrap();

        Op {
            op: op.parse().unwrap(),
            args: [lhs.parse().unwrap(), rhs.parse().unwrap()],
        }
    };

    let test = {
        let line = lines.next().unwrap();
        Test(
            line["  Test: divisible by ".len()..]
                .parse::<u64>()
                .unwrap(),
        )
    };

    let true_target = {
        let line = lines.next().unwrap();
        line["    If true: throw to monkey ".len()..]
            .parse()
            .unwrap()
    };

    let false_target = {
        let line = lines.next().unwrap();
        line["    If false: throw to monkey ".len()..]
            .parse()
            .unwrap()
    };

    Monkey {
        idx,
        items,
        operation,
        test,
        true_target,
        false_target,
        inspected: 0,
    }
}

mod ops {
    use std::fmt::Formatter;
    use std::num::NonZeroU64;
    use std::str::FromStr;

    macro_rules! const_assert {
        ($lhs:expr, $rhs:expr) => {
            const _: () = assert!($lhs == $rhs);
        };
        ($expr:expr) => {
            const _: () = assert!($expr);
        };
    }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub enum Operation {
        Add,
        Mul,
    }

    impl FromStr for Operation {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "+" => Ok(Self::Add),
                "*" => Ok(Self::Mul),
                _ => Err(()),
            }
        }
    }

    impl Operation {
        const fn perform(self, [lhs, rhs]: [u64; 2]) -> u64 {
            match self {
                Self::Add => lhs + rhs,
                Self::Mul => lhs * rhs,
            }
        }
    }

    impl std::fmt::Display for Operation {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Add => write!(f, "+"),
                Self::Mul => write!(f, "*"),
            }
        }
    }

    const_assert!(core::mem::size_of::<Operand>() == core::mem::size_of::<u64>());
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub enum Operand {
        Num(NonZeroU64),
        Arg,
    }

    impl FromStr for Operand {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "old" => Ok(Self::Arg),
                _ => s.parse().map(Self::Num).map_err(|_| ()),
            }
        }
    }

    impl Operand {
        fn insert(self, arg: u64) -> u64 {
            match self {
                Self::Num(n) => u64::from(n),
                Self::Arg => arg,
            }
        }
    }

    impl std::fmt::Display for Operand {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Num(n) => write!(f, "{n}"),
                Self::Arg => write!(f, "<arg>"),
            }
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Op {
        pub(crate) op: Operation,
        pub(crate) args: [Operand; 2],
    }

    impl Op {
        pub fn perform(&self, arg: u64) -> u64 {
            let args = self.args.map(|it| it.insert(arg));
            self.op.perform(args)
        }
    }

    impl std::fmt::Display for Op {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {} {}", self.args[0], self.op, self.args[1])
        }
    }
}
