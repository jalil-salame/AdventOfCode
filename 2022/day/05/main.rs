use color_eyre::{Report, Result};

use crate::parse::{parse_input, Crate};
#[allow(dead_code)]
fn empty_option_err() -> Report {
    use std::io::{Error, ErrorKind::Other};
    Error::new(Other, "Option was empty").into()
}

#[allow(dead_code)]
fn err(msg: &str) -> Report {
    use std::io::{Error, ErrorKind::Other};
    Error::new(Other, msg).into()
}

static INPUT: &str = include_str!("input");
// static INPUT: &str = "    [D]    \n [N] [C]    \n [Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n";

mod parse {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, digit1, line_ending, multispace1, one_of, space1},
        combinator::{map, map_res},
        multi::separated_list1,
        sequence::{delimited, preceded, tuple},
        IResult,
    };
    use std::str::FromStr;

    pub type CrateStack = Vec<Crate>;

    #[derive(Debug, Clone, Copy)]
    pub struct Crate(u8);

    impl Crate {
        pub fn as_char(self) -> char {
            self.0 as char
        }
    }

    fn stacks(input: &str) -> nom::IResult<&str, Vec<CrateStack>> {
        separated_list1(
            line_ending,
            separated_list1(
                tag(" "),
                alt((
                    delimited(char('['), one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), char(']')),
                    delimited(char(' '), char(' '), char(' ')),
                )),
            ),
        )(input)
        .map(|(s, v)| {
            let mut v: Vec<Vec<_>> = v
                .into_iter()
                .map(|s| {
                    s.into_iter()
                        .map(|c| match c {
                            ' ' => None,
                            c @ 'A'..='Z' => Some(Crate(c as u8)),
                            _ => unreachable!(),
                        })
                        .rev()
                        .collect()
                })
                .rev()
                .collect();
            let mut res: Vec<Vec<Crate>> = vec![vec![]; v[0].len()];
            for ele in res.iter_mut() {
                ele.extend(v.iter_mut().filter_map(|v| v.pop().flatten()));
            }
            (s, res)
        })
    }

    fn print_stack(stack: &CrateStack) -> String {
        stack.iter().copied().map(Crate::as_char).collect()
    }

    #[allow(dead_code)]
    pub fn print_stacks(stacks: &[CrateStack]) -> String {
        stacks
            .iter()
            .enumerate()
            .map(|(i, s)| format!("{}: {}\n", i + 1, print_stack(s)))
            .collect()
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Move {
        count: u32,
        from: u32,
        to: u32,
    }

    fn borrow_at_mut<T>(s: &mut [T], one: usize, two: usize) -> (&mut T, &mut T) {
        assert_ne!(one, two);
        if one < two {
            let (l, r) = s.split_at_mut(two);
            (&mut l[one], &mut r[0])
        } else {
            let (l, r) = s.split_at_mut(one);
            (&mut r[0], &mut l[two])
        }
    }

    impl Move {
        pub fn apply_one_by_one(self, stacks: &mut [CrateStack]) {
            let (f, t) = borrow_at_mut(stacks, self.from as usize - 1, self.to as usize - 1);
            t.extend(f.drain(f.len().saturating_sub(self.count as usize)..).rev());
        }

        pub fn apply_batch(self, stacks: &mut [CrateStack]) {
            let (f, t) = borrow_at_mut(stacks, self.from as usize - 1, self.to as usize - 1);
            t.extend(f.drain(f.len().saturating_sub(self.count as usize)..));
        }
    }

    fn moves(input: &str) -> IResult<&str, Vec<Move>> {
        separated_list1(
            line_ending,
            map(
                tuple((
                    preceded(tag("move "), map_res(digit1, u32::from_str)),
                    preceded(tag(" from "), map_res(digit1, u32::from_str)),
                    preceded(tag(" to "), map_res(digit1, u32::from_str)),
                )),
                |(count, from, to)| Move { count, from, to },
            ),
        )(input)
    }

    pub fn parse_input(input: &str) -> IResult<&str, (Vec<Vec<Crate>>, Vec<Move>)> {
        let (input, stacks) = stacks(input)?;
        let (input, _) =
            delimited(multispace1, separated_list1(space1, digit1), multispace1)(input)?;
        let (input, moves) = moves(input)?;
        Ok((input, (stacks, moves)))
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let (_, (stacks, moves)) = parse_input(INPUT)?;
    let mut stacks_p1 = stacks.clone();
    for ele in moves.iter() {
        ele.apply_one_by_one(&mut stacks_p1);
    }
    let p1_solution: String = stacks_p1
        .iter()
        .filter_map(|v| v.last().copied().map(Crate::as_char))
        .collect();

    let mut stacks_p2 = stacks;
    for ele in moves.iter() {
        ele.apply_batch(&mut stacks_p2);
    }
    let p2_solution: String = stacks_p2
        .iter()
        .filter_map(|v| v.last().copied().map(Crate::as_char))
        .collect();

    println!("Problem 1: {p1_solution}\nProblem 2: {p2_solution}");

    Ok(())
}
