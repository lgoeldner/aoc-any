use std::{
    cmp::max,
    io::{BufRead, BufReader},
};

use aoc_any::{BenchTimes, Info, ProblemResult, Run, Solution};

pub const SOLUTION: Solution = Solution {
    info: Info {
        name: "Calorie Counting",
        day: 1,
        year: 2022,
        bench: BenchTimes::Many(10),
    },
    part1: |_| 66_186.into(),
    part2: Some(|_| 196_804.into()),
    other: &[
        (
            "part1 heavy",
            |_| ProblemResult::Other(Box::new(biginp())),
            Run::No,
        ),
        (
            "part2 heavy",
            |_| ProblemResult::Other(Box::new(biginp2())),
            Run::No,
        ),
    ],
};

fn biginp() -> u32 {
    let mut reader =
        BufReader::new(std::fs::File::open("inputs/aoc_2022_day01_large_input.txt").unwrap());

    let mut line_buf = String::new();
    //let mut buf = String::new();
    let mut elf_sum = 0;
    let mut result = 0;
    while let Ok(1..) = {
        line_buf.clear();
        reader.read_line(&mut line_buf)
    } {
        match &line_buf as &str {
            "\n" => {
                result = max(result, elf_sum);
                elf_sum = 0;
            }
            _ => {
                if let Ok(num) = line_buf.trim().parse::<u32>() {
                    elf_sum += num;
                } else {
                    dbg!(&&line_buf, result);
                }
            }
        }
    }
    result
}

fn biginp2() -> u32 {
    let mut reader =
        BufReader::new(std::fs::File::open("inputs/aoc_2022_day01_large_input.txt").unwrap());
    let mut line_buf = String::new();
    //let mut buf = String::new();
    let mut elf_sum = 0;
    let mut result = [0; 3];

    while let Ok(1..) = {
        line_buf.clear();
        reader.read_line(&mut line_buf)
    } {
        match &line_buf as &str {
            "\n" => {
                if elf_sum > result[0] {
                    result[0] = elf_sum;
                } else if elf_sum > result[1] {
                    result[1] = elf_sum;
                } else if elf_sum > result[2] {
                    result[2] = elf_sum;
                }

                // for x in result.iter_mut() {
                //     if elf_sum > *x {
                //         *x = elf_sum;
                //         break;
                //     }
                // }

                elf_sum = 0;
            }
            _ => {
                if let Ok(num) = line_buf.trim().parse::<u32>() {
                    elf_sum += num;
                } else {
                    dbg!(&&line_buf, result);
                }
            }
        }
    }
    result.iter().sum()
}
