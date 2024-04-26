use aoc_any::{BenchTimes, Info, Solution};
use parse::{Packet, Value};

pub const SOLUTION: Solution = Solution {
    info: Info {
        name: "Distress Signal",
        day: 13,
        year: 2022,
        bench: BenchTimes::Many(10),
    },
    part1: |data| part1(data).into(),
    part2: Some(|data| part2(data).into()),
    other: &[],
};

const _EXAMPLE: &str = include_str!("../inputs/day13-test.txt");

fn part1(data: &str) -> usize {
    parse::part1(data)
        .iter()
        .map(cmp_packet::true_orders)
        .enumerate()
        .filter_map(|it| it.1.then_some(it.0 + 1))
        .sum::<usize>()
}

fn part2(data: &str) -> usize {
    thread_local! {
        static MARKER_PACKETS: [Packet; 2] = [
            Packet(Value::List(vec![Value::List(vec![Value::Num(2)].into())].into())),
            Packet(Value::List(vec![Value::List(vec![Value::Num(6)].into())].into())),
        ];
    }

    MARKER_PACKETS.with(|div_packets| {
        let mut parsed = parse::part2(data);

        parsed.extend_from_slice(div_packets);
        parsed.sort();

        let fst_packet = parsed.binary_search(&div_packets[0]).unwrap() + 1;
        let snd_packet = parsed.binary_search(&div_packets[1]).unwrap() + 1;

        fst_packet * snd_packet
    })
}

mod cmp_packet {

    use itertools::{EitherOrBoth, Itertools};

    use super::parse::{Packet, Value};

    use std::cmp;

    fn cmp_inner([lhs, rhs]: [&Value; 2]) -> cmp::Ordering {
        match (lhs, rhs) {
            (Value::Num(l), Value::Num(r)) => l.cmp(r),

            (Value::List(l), Value::List(r)) => cmp_lists_inner(l, r),

            (Value::Num(_), Value::List(r)) => cmp_lists_inner(&[lhs.clone()], r),

            (Value::List(l), Value::Num(_)) => cmp_lists_inner(l, &[rhs.clone()]),
        }
    }

    fn cmp_lists_inner(l: &[Value], r: &[Value]) -> cmp::Ordering {
        // short circuits when one list runs out or one number is not equal
        for either in l.iter().zip_longest(r.iter()) {
            match either {
                EitherOrBoth::Both(l, r) => match cmp_inner([l, r]) {
                    cmp::Ordering::Equal => continue,
                    r#else => return r#else,
                },
                EitherOrBoth::Left(_) => return cmp::Ordering::Greater,
                EitherOrBoth::Right(_) => return cmp::Ordering::Less,
            }
        }

        // else
        cmp::Ordering::Equal
    }

    // returns true if the packets are in the right order,
    /// so, if the left one is smaller than the right one.
    ///
    /// internally uses a `cmp::Ordering`.
    /// If the Ordering is Greater, the packet is in the wrong order
    /// If the Ordering is Less, the packet is in the right order
    pub fn true_orders([Packet(lhs), Packet(rhs)]: &[Packet; 2]) -> bool {
        match cmp_inner([&lhs, &rhs]) {
            cmp::Ordering::Greater => false,
            cmp::Ordering::Equal | cmp::Ordering::Less => true,
        }
    }

    impl std::cmp::PartialOrd for Packet {
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl std::cmp::Ord for Packet {
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            cmp_inner([&self.0, &other.0])
        }
    }

    #[test]
    fn cmp_packet_test() {
        const INP: &str = indoc::indoc! {"\
	[7,7,7,7]
	[7,7,7]"
        };

        let packet = super::parse::part1(INP);
        assert!(!true_orders(&packet[0]));
    }
}

mod parse {
    use std::rc::Rc;
    use std::{hint::unreachable_unchecked, iter::Peekable};

    use once_cell::sync::Lazy;
    use regex::Regex;

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
        type Error = std::num::ParseIntError;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            Ok(match value {
                "[" => Self::OpenList,
                "]" => Self::CloseList,
                n => Self::Num(n.parse()?),
            })
        }
    }

    #[derive(Clone, PartialEq, Eq)]
    pub struct Packet(pub Value);

    impl Packet {
        const fn get_list(&self) -> Result<&Rc<[Value]>, ()> {
            match self {
                Self(Value::List(l)) => Ok(l),
                _ => Err(()),
            }
        }
    }

    #[derive(Clone, PartialEq, Eq)]
    pub enum Value {
        Num(u32),
        List(Rc<[Value]>),
    }

    const _: () = assert!(std::mem::size_of::<Rc<Value>>() == 8);

    pub fn part2(data: &str) -> Vec<Packet> {
        data.split("\n\n")
            .flat_map(|it| {
                let (l, r) = it.split_once('\n').unwrap();

                [parse_line(l), parse_line(r)]
            })
            .collect()
    }

    pub fn part1(data: &str) -> Vec<[Packet; 2]> {
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
                    // if f.alternate() {
                    write!(f, "[")?;
                    // } else {
                    //     write!(f, "List[")?;
                    // }

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
                Value::List(Rc::clone(self.get_list().unwrap()))
            )
        }
    }
}
