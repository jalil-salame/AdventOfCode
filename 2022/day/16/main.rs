use color_eyre::{Report, Result};
use parse::{Valve, ValveLabel};
use petgraph::{
    dot::Dot,
    matrix_graph::{MatrixGraph, NodeIndex},
    Undirected,
};
use std::{collections::HashMap, fmt::Debug, time::Instant};

#[allow(dead_code)]
fn empty_option_err() -> Report {
    use std::io::{Error, ErrorKind::Other};
    Error::new(Other, "Option was empty").into()
}

static INPUT: &str = include_str!("input");

fn main() -> Result<()> {
    color_eyre::install()?;

    let now = Instant::now();

    let valves = {
        let now = Instant::now();

        let data = parse::input(INPUT)?.1;

        println!("Parsing took:    {:>16?}", now.elapsed());
        data
    };

    let now_ = Instant::now();

    // Add common data structures to Problem 1 and Problem 2 here
    let (start, graph) = as_graph(valves);

    println!("Processing took: {:>16?}", now_.elapsed());

    let problem_1_solution = {
        let now = Instant::now();

        println!("{:?}", Dot::new(&graph));

        let solution = "Nothing yet";

        println!("Problem 1 took:  {:>16?}", now.elapsed());
        solution
    };

    let problem_2_solution = {
        let now = Instant::now();

        let solution = "Nothing yet";

        println!("Problem 2 took:  {:>16?}", now.elapsed());
        solution
    };

    println!("Total runtime:   {:>16?}", now.elapsed());
    println!("----------------O----------------");
    println!("Problem 1:       {problem_1_solution:>16}");
    println!("Problem 2:       {problem_2_solution:>16}");

    Ok(())
}

struct Node {
    label: u16,
    flow_rate: u32,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node {}{} ({})",
            (self.label / 256) as u8 as char,
            (self.label % 256) as u8 as char,
            self.flow_rate
        )
    }
}

fn as_graph(
    valves: impl IntoIterator<Item = Valve>,
) -> (NodeIndex<u16>, MatrixGraph<Node, f32, Undirected>) {
    let mut graph = MatrixGraph::<Node, f32, Undirected>::with_capacity(57);
    let mut map: HashMap<ValveLabel, _> = HashMap::new();

    for Valve {
        label,
        flow_rate,
        connections,
    } in valves
    {
        let ix = if let Some(ix) = map.get(&label) {
            *ix
        } else {
            let ix = graph.add_node(Node { label, flow_rate });
            map.insert(label, ix);
            ix
        };

        for label in connections {
            let Some(iy) = map.get(&label) else {continue;};

            graph.update_edge(ix, *iy, 1.0);
        }
    }

    (*map.get(&('A' as u16 * 256 + 'A' as u16)).unwrap(), graph)
}

mod parse {
    use nom::{
        bytes::complete::tag,
        character::{
            self,
            complete::{line_ending, one_of},
        },
        combinator::{eof, map},
        multi::{many1, separated_list1},
        sequence::{delimited, pair, preceded, terminated, tuple},
        IResult,
    };

    pub type ValveLabel = u16;

    #[derive(Debug)]
    pub struct Valve {
        pub label: ValveLabel,
        pub flow_rate: u32,
        pub connections: Vec<ValveLabel>,
    }

    impl Valve {
        fn parse(input: &str) -> IResult<&str, Self> {
            map(
                tuple((
                    preceded(
                        tag("Valve "),
                        map(
                            pair(
                                one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
                                one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
                            ),
                            |(a, b)| (a as u16 * 256 + b as u16),
                        ),
                    ),
                    delimited(tag(" has flow rate="), character::complete::u32, tag("; ")),
                    preceded(
                        nom::branch::alt((
                            tag("tunnel leads to valve "),
                            tag("tunnels lead to valves "),
                        )),
                        separated_list1(
                            tag(", "),
                            map(
                                pair(
                                    one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
                                    one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
                                ),
                                |(a, b)| (a as u16 * 256 + b as u16),
                            ),
                        ),
                    ),
                )),
                |(label, flow_rate, connections)| Self {
                    label,
                    flow_rate,
                    connections,
                },
            )(input)
        }
    }

    pub fn input(input: &str) -> IResult<&str, Vec<Valve>> {
        terminated(many1(terminated(Valve::parse, line_ending)), eof)(input)
    }
}

#[cfg(test)]
mod test {
    use crate::{as_graph, parse};
    use color_eyre::Result;
    use petgraph::{algo::bellman_ford, dot::Dot};
    #[allow(unused)]
    use pretty_assertions::{assert_eq, assert_ne};

    static INPUT: &str = include_str!("test_input");

    #[test]
    fn parse_test_input() -> Result<()> {
        let (rest, _) = parse::input(INPUT)?;

        assert_eq!(rest, "");

        Ok(())
    }

    #[test]
    fn parse_input() -> Result<()> {
        let (rest, _) = parse::input(crate::INPUT)?;

        assert_eq!(rest, "");

        Ok(())
    }

    #[test]
    fn problem_1() -> Result<()> {
        let valves = parse::input(INPUT)?.1;

        let (start, graph) = as_graph(valves);
        println!("{:?}", Dot::new(&graph));
        println!("{:#?}", bellman_ford(&graph, start).unwrap());
        assert_eq!(0, 1651);

        Ok(())
    }

    #[test]
    #[ignore]
    fn problem_2() -> Result<()> {
        let _ = parse::input(INPUT)?.1;

        Ok(())
    }
}
