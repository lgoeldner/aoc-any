use std::fmt::Formatter;
use std::{convert::Into, str::FromStr};

use ndarray::Array2;

use aoc_any::{BenchTimes, Info, ProblemResult, Run, Solution};

pub const SOLUTION: Solution = Solution {
    info: Info {
        name: "Cathode-Ray Tube",
        day: 10,
        year: 2022,
        bench: BenchTimes::Once,
    },
    part1: |data| do_part1(data).unwrap().into(),
    part2: Some(|data| {
        ProblemResult::Other({
            let _ = do_part2(data);
            Box::new(())
        })
    }),
    other: &[
        (
            "part1 example",
            |_| do_part1(TEST_DATA).unwrap().into(),
            Run::No,
        ),
        (
            "part2 printed",
            |data| {
                ProblemResult::Other({
                    eprintln!("{}", do_part2(data).unwrap());
                    Box::new(())
                })
            },
            Run::No,
        ),
    ],
};

// const DATA: &str = include_str!("../inputs/day10-inp.txt");

const TEST_DATA: &str = include_str!("../inputs/day10-test.txt");

fn do_part1(data: &str) -> anyhow::Result<i32> {
    let instructions = parse(data)?;
    let mut instructions = instructions.iter();

    let mut register = 1;
    let mut carryover_addinstr = Some(0);
    let mut checksignal_at_next_idx = 20;
    let mut signal_strength_sum = 0;

    let mut check = |i, register| {
        if i == checksignal_at_next_idx {
            checksignal_at_next_idx += 40;
            signal_strength_sum += i * register;
        }
    };

    let mut i = 0;

    loop {
        match instructions.next().unwrap() {
            Instruction::Noop => i += 1,
            Instruction::AddX(n) => {
                check(i + 1, register);
                i += 2;
                carryover_addinstr = Some(*n);
            }
        }
        check(i, register);

        if i >= 220 {
            break Ok(signal_strength_sum);
        }

        if let Some(y) = carryover_addinstr {
            carryover_addinstr = None;
            register += y;
        }
    }
}

fn do_part2(data: &str) -> anyhow::Result<String> {
    let instructions = parse(data)?;
    let mut instructions = instructions.iter();

    let mut register = 1;

    let mut screen: Array2<Pixel> = Array2::default((6, 40));

    let mut next_pixel = screen.iter_mut().enumerate();

    let mut cycle_idx = 0;
    let mut check = |cycle, register| {
        let pixel = next_pixel.next().unwrap();
        if (register - 1..=register + 1).contains(&((cycle - 1) % 40)) {
            *pixel.1 = Pixel::Filled;
        }
    };

    loop {
        // check the instruction. if its a noop, increment the cycle index and move on.
        // the check fn is called after the match block.
        // the add instruction
        let carryover_addinstr = match instructions.next().unwrap() {
            Instruction::Noop => {
                cycle_idx += 1;
                None
            }
            Instruction::AddX(n) => {
                check(cycle_idx + 1, register);
                cycle_idx += 2;
                Some(*n)
            }
        };

        //----  during the cycle ----//

        check(cycle_idx, register);

        if cycle_idx >= 240 {
            break;
        }

        //----  after the cycle ----//
        // finish add instruction if there is one
        if let Some(y) = carryover_addinstr {
            register += y;
        }
    }

    // eprintln!("\n{:?}", Dbgarr(screen));
    Ok(format!("{:?}", Dbgarr(screen)))
}

#[derive(Default, Copy, Clone)]
enum Pixel {
    Filled,
    #[default]
    Dark,
}

impl std::fmt::Debug for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Filled => write!(f, "#"),
            Self::Dark => write!(f, "."),
        }
    }
}

struct Dbgarr<T>(Array2<T>);
impl std::fmt::Debug for Dbgarr<Pixel> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for cell in self.0.rows() {
            for pixel in cell {
                write!(f, "{pixel:?}")?;
            }
            writeln!(f)?;
        }

        Ok(())
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
