use std::iter::once;

use color_eyre::{Report, Result};
use itertools::Itertools;
use parse::Instruction;

use crate::parse::input;

#[allow(dead_code)]
fn empty_option_err() -> Report {
    use std::io::{Error, ErrorKind::Other};
    Error::new(Other, "Option was empty").into()
}

static INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    color_eyre::install()?;

    let program = input(INPUT)?.1;
    let p1_solution = signal_strength_sum::<20, 40>(&program);
    let p2_solution = crt_drawing::<40, 6>(&program);

    println!("Problem 1: {p1_solution}\nProblem 2:\n{p2_solution}");

    Ok(())
}

pub type Program<'a> = &'a [Instruction];

fn signal_strength_sum<const START: u32, const STEP: u32>(program: Program) -> i32 {
    Cpu::new(program)
        .filter_map(|(cycle, x_reg)| {
            if (cycle + START) % STEP == 0 {
                Some(cycle as i32 * x_reg)
            } else {
                None
            }
        })
        .sum()
}

fn crt_drawing<const CRT_WIDTH: u32, const CRT_HEIGHT: u32>(program: Program) -> String {
    let mut res = String::new();
    for slice in &Cpu::new(program)
        .map(|(cycle, x_reg)| {
            let crt_clk = (cycle - 1) % CRT_WIDTH;
            // println!("{cycle}: {crt_clk}, {x_reg}");
            if (-1..=1).contains(&(x_reg - crt_clk as i32)) {
                '#'
            } else {
                '.'
            }
        })
        .chunks(CRT_WIDTH as usize)
    {
        res.extend(slice);
        res.extend(once('\n'));
    }
    res
}

#[derive(Debug)]
struct Cpu<'a> {
    cycle: u32,
    x_reg: i32,
    state: CpuState,
    program: Program<'a>,
}

#[derive(Debug, Default)]
enum CpuState {
    #[default]
    Idle,
    Processing,
}

impl<'a> Cpu<'a> {
    fn new(program: Program<'a>) -> Self {
        Self {
            program,
            ..Default::default()
        }
    }

    fn one_cycle(&mut self) -> Option<(u32, i32)> {
        let next = self.program.iter().next()?;
        self.cycle += 1;
        match (&self.state, next) {
            (CpuState::Idle, Instruction::Noop) => {
                self.program = &self.program[1..];
                Some((self.cycle, self.x_reg))
            }
            (CpuState::Idle, Instruction::AddX(_)) => {
                self.state = CpuState::Processing;
                Some((self.cycle, self.x_reg))
            }
            (CpuState::Processing, Instruction::Noop) => panic!("Invalid CPU State"),
            (CpuState::Processing, Instruction::AddX(n)) => {
                self.state = CpuState::Idle;
                self.x_reg += n;

                self.program = &self.program[1..];
                Some((self.cycle, self.x_reg - n))
            }
        }
    }
}

impl<'a> Iterator for Cpu<'a> {
    type Item = (u32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.one_cycle()
    }
}

impl Default for Cpu<'_> {
    fn default() -> Self {
        Self {
            cycle: 0,
            x_reg: 1,
            program: &[],
            state: Default::default(),
        }
    }
}

mod parse {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, line_ending},
        combinator::{map, map_res},
        multi::many1,
        sequence::{preceded, terminated},
        IResult,
    };

    #[derive(Debug, PartialEq, Eq)]
    pub enum Instruction {
        Noop,
        AddX(i32),
    }

    impl Instruction {
        pub fn cycles(&self) -> u32 {
            match self {
                Instruction::Noop => 1,
                Instruction::AddX(_) => 2,
            }
        }
    }

    pub fn input(input: &str) -> IResult<&str, Vec<Instruction>> {
        many1(terminated(
            alt((
                map(tag("noop"), |_| Instruction::Noop),
                map(
                    preceded(
                        tag("addx "),
                        alt((
                            map_res(digit1, str::parse::<i32>),
                            map_res(preceded(tag("-"), digit1), |s: &str| {
                                s.parse::<i32>().map(|n| -n)
                            }),
                        )),
                    ),
                    Instruction::AddX,
                ),
            )),
            line_ending,
        ))(input)
    }
}

#[cfg(test)]
mod test {
    use crate::{crt_drawing, parse::Instruction, signal_strength_sum};
    use color_eyre::Result;
    use pretty_assertions::assert_eq;

    static INPUT: &str = "noop\naddx 3\naddx -5\n";
    static TEST_INPUT: &str = include_str!("../test_input");
    static TEST_IMAGE: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

    #[test]
    fn problem_1_parse_input() -> Result<()> {
        assert_eq!(
            &super::parse::input(INPUT)?.1,
            &[
                Instruction::Noop,
                Instruction::AddX(3),
                Instruction::AddX(-5)
            ]
        );

        Ok(())
    }

    #[test]
    fn problem_1_signal_strength() -> Result<()> {
        let program = super::parse::input(TEST_INPUT)?.1;
        assert_eq!(signal_strength_sum::<20, 40>(&program), 13140);
        Ok(())
    }

    #[test]
    fn problem_2() -> Result<()> {
        let program = super::parse::input(TEST_INPUT)?.1;
        assert_eq!(&crt_drawing::<40, 6>(&program), TEST_IMAGE);
        Ok(())
    }
}
