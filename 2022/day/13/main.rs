use color_eyre::{Report, Result};
use parse::Packet;
use std::time::Instant;

#[allow(dead_code)]
fn empty_option_err() -> Report {
    use std::io::{Error, ErrorKind::Other};
    Error::new(Other, "Option was empty").into()
}

static INPUT: &str = include_str!("input");

fn main() -> Result<()> {
    color_eyre::install()?;

    let now = Instant::now();

    let packets = {
        let now = Instant::now();

        let data = parse::input(INPUT)?;

        println!("Parsing took:    {:>16?}", now.elapsed());
        data
    };

    let now_ = Instant::now();

    // Add common data structures to Problem 1 and Problem 2 here

    println!("Processing took: {:>16?}", now_.elapsed());

    let problem_1_solution = {
        let now = Instant::now();

        let solution = in_order_ix_sum(&packets);

        println!("Problem 1 took:  {:>16?}", now.elapsed());
        solution
    };

    let problem_2_solution = {
        let now = Instant::now();

        let solution = decoder_key(packets);

        println!("Problem 2 took:  {:>16?}", now.elapsed());
        solution
    };

    println!("Total runtime:   {:>16?}", now.elapsed());
    println!("----------------O----------------");
    println!("Problem 1:       {problem_1_solution:>16}");
    println!("Problem 2:       {problem_2_solution:>16}");

    Ok(())
}

fn decoder_key(packets: Vec<(Packet, Packet)>) -> usize {
    let mut packets: Vec<_> = packets.into_iter().flat_map(|(l, r)| [l, r]).collect();

    let dividers = Packet::dividers();
    packets.extend_from_slice(&dividers);
    packets.sort_unstable();

    let left_ix = packets.iter().position(|packet| &dividers[0] == packet).expect("couldn't find left divisor");
    let right_ix = packets.iter().position(|packet| &dividers[1] == packet).expect("couldn't find right divisor");
    (1+left_ix) * (1+right_ix)
}

fn in_order_ix_sum(packets: &[(Packet, Packet)]) -> usize {
    packets
        .iter()
        .enumerate()
        .filter(|(_, pair)| in_order(pair))
        .map(|(ix, _)| ix + 1)
        .sum()
}

fn in_order((left, right): &(Packet, Packet)) -> bool {
    left < right
}

mod parse {
    use std::cmp::Ordering;

    use miette::GraphicalReportHandler;
    use nom::{
        branch::alt,
        character::{
            self,
            complete::{line_ending, multispace0},
            streaming::char,
        },
        combinator::map,
        error::ParseError,
        multi::{separated_list0, separated_list1},
        sequence::{delimited, separated_pair, terminated},
        IResult,
    };
    use nom_locate::LocatedSpan;
    use nom_supreme::{
        error::{BaseErrorKind, ErrorTree, GenericErrorTree},
        final_parser::final_parser,
    };

    pub type Span<'a> = LocatedSpan<&'a str>;

    #[derive(thiserror::Error, Debug, miette::Diagnostic)]
    #[error("bad input")]
    pub struct BadInput {
        #[source_code]
        src: &'static str,

        #[label("{kind}")]
        bad_bit: miette::SourceSpan,

        kind: BaseErrorKind<&'static str, Box<dyn std::error::Error + Send + Sync>>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Value {
        List(List),
        Integer(u32),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct List(Vec<Value>);

    impl Packet {
        pub fn dividers() -> [Self; 2] {
            [
                Self(vec![Value::List(Self(vec![Value::Integer(2)]))]),
                Self(vec![Value::List(Self(vec![Value::Integer(6)]))]),
            ]
        }
    }

    impl PartialOrd for List {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for List {
        fn cmp(&self, other: &Self) -> Ordering {
            for (left, right) in self.0.iter().zip(other.0.iter()) {
                match left.cmp(right) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => continue,
                    Ordering::Greater => return Ordering::Greater,
                }
            }

            self.0.len().cmp(&other.0.len())
        }
    }

    impl PartialOrd for Value {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Value {
        fn cmp(&self, other: &Self) -> Ordering {
            match (self, other) {
                (Value::List(left), Value::List(right)) => left.cmp(right),
                (Value::List(left), Value::Integer(_)) => {
                    left.cmp(&List(vec![other.clone()]))
                }
                (Value::Integer(_), Value::List(right)) => {
                    List(vec![self.clone()]).cmp(right)
                }
                (Value::Integer(left), Value::Integer(right)) => left.cmp(right),
            }
    }
    }

    pub type Packet = List;

    impl Packet {
        fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
            map(
                delimited(
                    char('['),
                    separated_list0(char(','), Value::parse),
                    char(']'),
                ),
                Self,
            )(input)
        }
    }

    impl Value {
        fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
            alt((
                map(character::complete::u32, Value::Integer),
                map(List::parse, Self::List),
            ))(input)
        }
    }

    fn input_<'a, E: ParseError<Span<'a>>>(
        input: Span<'a>,
    ) -> IResult<Span<'a>, Vec<(Packet, Packet)>, E> {
        terminated(
            separated_list1(
                line_ending,
                terminated(
                    separated_pair(Packet::parse, line_ending, Packet::parse),
                    line_ending,
                ),
            ),
            multispace0,
        )(input)
    }

    pub fn input(input: &'static str) -> Result<Vec<(Packet, Packet)>, BadInput> {
        let res: Result<_, ErrorTree<Span>> =
            final_parser(input_::<ErrorTree<Span>>)(Span::new(input));

        match res {
            Ok(data) => Ok(data),
            Err(e) => match e {
                GenericErrorTree::Base { location, kind } => {
                    let offset = location.location_offset().into();
                    let err = BadInput {
                        src: input,
                        bad_bit: miette::SourceSpan::new(offset, 0.into()),
                        kind,
                    };
                    let mut s = String::new();
                    GraphicalReportHandler::new()
                        .render_report(&mut s, &err)
                        .unwrap();
                    println!("{s}");
                    Err(err)
                }
                GenericErrorTree::Stack { .. } => todo!("stack"),
                GenericErrorTree::Alt(_) => todo!("alt"),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{in_order_ix_sum, parse, decoder_key};
    use color_eyre::Result;
    #[allow(unused)]
    use pretty_assertions::{assert_eq, assert_ne};

    static INPUT: &str = include_str!("test_input");

    #[test]
    fn parse_test_input() -> Result<()> {
        let data = parse::input(INPUT)?;

        assert_eq!(data.len(), 8);

        Ok(())
    }

    #[test]
    fn parse_input() -> Result<()> {
        let data = parse::input(crate::INPUT)?;

        assert_eq!(data.len(), 150);

        Ok(())
    }

    #[test]
    fn problem_1() -> Result<()> {
        let data = parse::input(INPUT)?;

        assert_eq!(in_order_ix_sum(&data), 13);

        Ok(())
    }

    #[test]
    fn problem_2() -> Result<()> {
        let data = parse::input(INPUT)?;

        assert_eq!(decoder_key(data), 140);

        Ok(())
    }
}
