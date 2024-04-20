use std::{convert::Into, str::FromStr};

use aoc_any::{BenchTimes, Info, Run, Solution};

pub const SOLUTION: Solution = Solution {
    info: Info {
        name: "Cathode-Ray Tube",
        day: 10,
        year: 2022,
        bench: BenchTimes::Once,
    },
    part1: || do_part1(DATA).unwrap().into(),
    part2: None,
    other: &[(
        "part1 example",
        || do_part1(TEST_DATA).unwrap().into(),
        Run::No,
    )],
};

const DATA: &str = include_str!("../inputs/day10-inp.txt");

const TEST_DATA: &str = include_str!("../inputs/day10-test.txt");

fn do_part1(data: &str) -> anyhow::Result<i32> {
    let instructions = parse(data)?;
    let mut instructions = instructions.iter().map(|it| match it {
        Instruction::Noop => (it, 1),
        Instruction::AddX(_) => (it, 2),
    });

    let mut register = 1;
    let mut carryover_addinstr = Some(0);
    let mut checksignal_at_next_idx = 20;
    let mut signal_strength_sum = 0;

    let mut check_signalstrength_at_cycle = |i, register| {
        if i == checksignal_at_next_idx {
            checksignal_at_next_idx += 40;
            signal_strength_sum += i * register;
        }
    };

    let mut i = 0;

    loop {
        if let Some(y) = carryover_addinstr {
            carryover_addinstr = None;
            register += y;
        }

        match instructions.next().unwrap() {
            (Instruction::Noop, _) => i += 1,
            (Instruction::AddX(n), _) => {
                check_signalstrength_at_cycle(i + 1, register);
                i += 2;
                carryover_addinstr = Some(*n);
            }
        }
        check_signalstrength_at_cycle(i, register);

        if i >= 220 {
            break Ok(signal_strength_sum);
        }
    }
}

enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            None if s == "noop" => Ok(Self::Noop),
            Some(("addx", n)) => Ok(Self::AddX(n.parse()?)),
            _ => anyhow::bail!("Invalid Instruction"),
        }
    }
}

fn parse(data: &str) -> anyhow::Result<Vec<Instruction>> {
    data.lines().map(str::parse).collect::<Result<Vec<_>, _>>()
}
