use std::{num::ParseIntError, rc::Rc};

use aoc_any::{BenchTimes, Info, Solution};
use itertools::Itertools;

use crate::yr2022_day13::parse::Value;

use self::parse::Packet;

pub const SOLUTION: Solution = Solution {
    info: Info {
        name: "Distress Signal",
        day: 13,
        year: 2022,
        bench: BenchTimes::None,
    },
    part1: |_data| part1(EXAMPLE).into(),
    part2: None,
    other: &[],
};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Token {
    OpenList,
    CloseList,
    Num(u32),
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpenList => write!(f, "["),
            Self::CloseList => write!(f, "]"),
            Self::Num(n) => write!(f, "{n}"),
        }
    }
}

impl TryFrom<&str> for Token {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "[" => Self::OpenList,
            "]" => Self::CloseList,
            n => Self::Num(n.parse()?),
        })
    }
}

const EXAMPLE: &str = include_str!("../inputs/day13-test.txt");

macro_rules! boxed {
    ($ex:expr) => {
        Box::new($ex)
    };
}

fn part1(data: &str) -> usize {
    let parsed = parse::parse(data);

    dbg!(parsed
        .iter()
        .map(cmp_packet)
        .enumerate()
        .filter_map(|it| it.1.then_some(it.0 + 1))
        .collect_vec());
    todo!()
    //.sum::<usize>()
}

fn cmp_packet([lhs, rhs]: &[Packet; 2]) -> bool {
    fn inner_cmp((lhs, rhs): (&Value, &Value)) -> bool {
        match (lhs, rhs) {
            (Value::Num(l), Value::Num(r)) => l <= r,

            (lhs @ Value::List(_), num @ Value::Num(_)) => {
                // let rhs_len = 1;
                // if rhs_len < lhs.len() {
                //     false
                // } else if lhs.len() == 1 {
                //     inner_cmp((&lhs[0], &num))
                // } else {
                //     true
                // }
                inner_cmp((lhs, &Value::List(Rc::new([num.clone()]))))
            }

            (num @ Value::Num(_), rhs @ Value::List(_)) => {
                // let lhs_len = 1;
                // if lhs_len < rhs.len() {
                //     false
                // }
                // // else if rhs.len() == 1 {
                // //     inner_cmp((&num, &rhs[0]))
                // // }
                // else {
                //     true
                // }

                inner_cmp((&Value::List(Rc::new([num.clone()])), &rhs))
            }

            (Value::List(lhs), Value::List(rhs)) => match lhs.len().cmp(&rhs.len()) {
                std::cmp::Ordering::Equal => lhs.iter().zip(rhs.iter()).all(inner_cmp),
                std::cmp::Ordering::Greater => false,
                std::cmp::Ordering::Less => true,
            },
        }
    }

    inner_cmp((&lhs.0, &rhs.0))

    //	lhs.0.iter().zip(rhs.0.iter()).all(inner_cmp)
}

#[test]
fn cmp_packet_test() {
    const INP: &str = indoc::indoc! {"\
	[7,7,7,7]
	[7,7,7]"
    };

    let packet = parse::parse(INP);
    assert!(!cmp_packet(&packet[0]));
}

mod parse {
    use std::rc::Rc;
    use std::{hint::unreachable_unchecked, iter::Peekable};

    use once_cell::sync::Lazy;
    use regex::Regex;

    use super::Token;

    #[derive(Clone)]
    pub struct Packet(pub Value);

    impl Packet {
        const fn get_list(&self) -> Result<&Rc<[Value]>, ()> {
            match self {
                Self(Value::List(l)) => Ok(l),
                _ => Err(()),
            }
        }
    }

    #[derive(Clone)]
    pub enum Value {
        Num(u32),
        List(Rc<[Value]>),
    }

    pub fn parse(data: &str) -> Vec<[Packet; 2]> {
        data.split("\n\n")
            .map(|it| {
                let (l, r) = it.split_once('\n').unwrap();

                [parse_line(l), parse_line(r)]
            })
            .collect()
    }

    fn parse_line(line: &str) -> Packet {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[|\]|\d+").unwrap());

        let tokens = RE
            .find_iter(line)
            .map(|t| Token::try_from(t.as_str()).unwrap());

        let l = parse_list(&mut tokens.peekable());

        Packet(l)
    }

    fn parse_list(inp: &mut Peekable<impl Iterator<Item = Token>>) -> Value {
        // if the next or the one after are none, return an empty list
        if inp.next().is_none() || inp.peek().is_none() {
            return Value::List(vec![].into_boxed_slice().into());
        }

        let mut s: Vec<Value> = vec![];

        loop {
            let elem = inp.peek().copied();

            match elem {
                Some(Token::OpenList) => s.push(parse_list(inp)),

                Some(Token::Num(n)) => {
                    inp.next();
                    s.push(Value::Num(n));
                }

                Some(Token::CloseList) => {
                    inp.next();
                    return Value::List(s.into_boxed_slice().into());
                }

                None => unsafe { unreachable_unchecked() },
            }
        }
    }

    impl std::fmt::Debug for Value {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::List(l) => {
                    if f.alternate() {
                        write!(f, "[")?;
                    } else {
                        write!(f, "List[")?;
                    }

                    if let Some(first) = l.first() {
                        write!(f, "{first:?}")?;
                    }

                    l.iter().skip(1).try_for_each(|it| write!(f, ", {it:?}"))?;

                    write!(f, "]")
                }
                Self::Num(n) => write!(f, "{n}"),
            }
        }
    }

    impl std::fmt::Debug for Packet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Packet({:#?})",
                Value::List(Rc::clone(&self.get_list().unwrap()))
            )
        }
    }
}
