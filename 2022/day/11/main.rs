use color_eyre::{Report, Result};
use itertools::Itertools;
use parse::Monkey;

use crate::parse::input;

#[allow(dead_code)]
fn empty_option_err() -> Report {
    use std::io::{Error, ErrorKind::Other};
    Error::new(Other, "Option was empty").into()
}

static INPUT: &str = include_str!("input");

fn main() -> Result<()> {
    color_eyre::install()?;

    let monkeys = input(INPUT)?.1;

    let (_, inspect_count) = play_rounds::<20, 3>(&monkeys);
    let p1_solution = monkey_bussiness(&inspect_count);

    let (_, inspect_count) = play_rounds::<10000, 1>(&monkeys);
    let p2_solution = monkey_bussiness(&inspect_count);

    println!("Problem 1: {p1_solution}\nProblem 2: {p2_solution}");

    Ok(())
}

fn monkey_bussiness(inspect_count: &[usize]) -> i64 {
    let mut biggest = inspect_count.iter().map(|&n| -(n as i64)).k_smallest(2);
    let (Some(c1),Some(c2), None) = (biggest.next(), biggest.next(), biggest.next()) else {
            panic!("Didn't have only 2 elements")
        };
    c1 * c2
}

fn play_rounds<const ROUNDS: usize, const WORRY_DIV: u32>(
    monkeys: &[Monkey],
) -> (Vec<Monkey>, Vec<usize>) {
    let mut monkeys: Vec<_> = monkeys.to_vec();
    let gcd = monkeys.iter().map(Monkey::test_divisor).product();
    let mut inspect_count = vec![0; monkeys.len()];

    for _ in 0..ROUNDS {
        round::<WORRY_DIV>(&mut monkeys, &mut inspect_count, gcd)
    }

    (monkeys, inspect_count)
}

fn round<const WORRY_DIV: u32>(monkeys: &mut [Monkey], inspect_count: &mut [usize], gcd: u64) {
    let mut thrown = vec![vec![]; monkeys.len()];

    for (ix, (monkey, inspected)) in monkeys.iter_mut().zip(inspect_count.iter_mut()).enumerate() {
        monkey.recieve(thrown[ix].drain(0..));

        *inspected += monkey.inspects();

        let ((monkey_0, thrown_0), (monkey_1, thrown_1)) = monkey.inspect_all::<WORRY_DIV>(gcd);

        thrown[monkey_0 as usize].extend(thrown_0.into_iter());
        thrown[monkey_1 as usize].extend(thrown_1.into_iter());
    }

    for (monkey, thrown) in monkeys.iter_mut().zip(thrown.into_iter()) {
        monkey.recieve(thrown.into_iter());
    }
}

mod parse {
    use nom::{
        branch::{self, alt},
        bytes::complete::tag,
        character::{
            complete::{self, line_ending, multispace0, space1},
            streaming::char,
        },
        combinator::{eof, map, value},
        multi::{many1, separated_list1},
        sequence::{delimited, pair, preceded, terminated, tuple},
        IResult, Parser,
    };

    pub type ItemWorry = u64;

    #[derive(Debug, Clone)]
    pub struct Monkey {
        items: Vec<ItemWorry>,
        operation: Operation,
        test: Test,
    }

    impl Monkey {
        pub fn inspect_all<const WORRY_DIV: u32>(
            &mut self,
            gcd: u64,
        ) -> ((u32, Vec<ItemWorry>), (u32, Vec<ItemWorry>)) {
            let mut passed_test = vec![];
            let mut failed_test = vec![];

            for worry in self.items.drain(0..) {
                let new_worry = self.operation.apply(worry, gcd) / WORRY_DIV as u64; // Reduced by not being broken
                if self.test.apply(new_worry) {
                    &mut passed_test
                } else {
                    &mut failed_test
                }
                .push(new_worry);
            }

            (
                (self.test.throw_passed_test, passed_test),
                (self.test.throw_failed_test, failed_test),
            )
        }

        pub fn test_divisor(&self) -> u64 {
            self.test.divisible_by
        }

        pub fn recieve(&mut self, items: impl Iterator<Item = ItemWorry>) {
            self.items.extend(items);
        }

        pub fn inspects(&self) -> usize {
            self.items.len()
        }
    }

    impl From<(Vec<ItemWorry>, Operation, Test)> for Monkey {
        fn from((items, operation, test): (Vec<ItemWorry>, Operation, Test)) -> Self {
            Self {
                items,
                operation,
                test,
            }
        }
    }

    #[derive(Debug, Clone)]
    struct Operation {
        kind: Op,
        to: Value,
    }

    impl Operation {
        fn apply(&self, worry: ItemWorry, gcd: u64) -> ItemWorry {
            let value = match self.to {
                Value::Immediate(n) => n,
                Value::Old => worry,
            };

            match self.kind {
                Op::Add => (worry + value) % gcd,
                Op::Mul => (worry * value) % gcd,
            }
        }

        fn parse(input: &str) -> IResult<&str, Self> {
            delimited(
                pair(space1, tag("Operation: new = old ")),
                pair(
                    branch::alt((value(Op::Add, char('+')), value(Op::Mul, char('*')))),
                    alt((
                        preceded(space1, complete::u64).map(Value::Immediate),
                        tag(" old").map(|_| Value::Old),
                    )),
                )
                .map(|(kind, to)| Operation { kind, to }),
                line_ending,
            )(input)
        }
    }

    #[derive(Debug, Clone, Copy)]
    enum Op {
        Add,
        Mul,
    }

    #[derive(Debug, Clone, Copy)]
    enum Value {
        Immediate(ItemWorry),
        Old,
    }

    #[derive(Debug, Clone)]
    struct Test {
        divisible_by: u64,
        throw_passed_test: u32,
        throw_failed_test: u32,
    }

    impl Test {
        fn apply(&self, worry: ItemWorry) -> bool {
            worry % self.divisible_by == 0
        }

        fn parse(input: &str) -> IResult<&str, Self> {
            map(
                tuple((
                    delimited(
                        pair(space1, tag("Test: divisible by ")),
                        complete::u64,
                        line_ending,
                    ),
                    delimited(
                        pair(space1, tag("If true: throw to monkey ")),
                        complete::u32,
                        line_ending,
                    ),
                    delimited(
                        pair(space1, tag("If false: throw to monkey ")),
                        complete::u32,
                        line_ending,
                    ),
                )),
                |(divisible_by, throw_passed_test, throw_failed_test)| Test {
                    divisible_by,
                    throw_passed_test,
                    throw_failed_test,
                },
            )(input)
        }
    }

    pub fn input(input: &str) -> IResult<&str, Vec<Monkey>> {
        terminated(
            many1(terminated(
                preceded(
                    delimited(tag("Monkey "), complete::u32, pair(tag(":"), line_ending)),
                    tuple((
                        delimited(
                            pair(space1, tag("Starting items: ")),
                            separated_list1(tag(", "), complete::u64),
                            line_ending,
                        ),
                        Operation::parse,
                        Test::parse,
                    ))
                    .map(Monkey::from),
                ),
                multispace0,
            )),
            eof,
        )(input)
    }
}

#[cfg(test)]
mod test {
    use color_eyre::Result;
    use pretty_assertions::assert_eq;

    use crate::{monkey_bussiness, parse::input, play_rounds};

    static INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn parse_input() -> Result<()> {
        let (rest, monkeys) = input(INPUT)?;

        assert_eq!(rest, "");
        assert_eq!(monkeys.len(), 4);

        Ok(())
    }

    #[test]
    fn problem_1() -> Result<()> {
        let monkeys = input(INPUT)?.1;
        let (_, inspect) = play_rounds::<20, 3>(&monkeys);
        assert_eq!(monkey_bussiness(&inspect), 10605);

        Ok(())
    }

    #[test]
    fn problem_2() -> Result<()> {
        let monkeys = input(INPUT)?.1;
        let (_, inspect) = play_rounds::<10000, 1>(&monkeys);
        assert_eq!(monkey_bussiness(&inspect), 2713310158);

        Ok(())
    }
}
