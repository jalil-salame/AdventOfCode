use std::{
    collections::{HashMap, HashSet},
    iter::{once, repeat},
};

use color_eyre::{Report, Result};
use itertools::Itertools;
use parse::{Direction, Move};

#[allow(dead_code)]
fn empty_option_err() -> Report {
    use std::io::{Error, ErrorKind::Other};
    Error::new(Other, "Option was empty").into()
}

static INPUT: &str = include_str!("../input");
static TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

fn main() -> Result<()> {
    color_eyre::install()?;

    let (_, test_moves) = parse::input(TEST_INPUT)?;
    assert_eq!(
        visited_positions(&test_moves)
            .iter()
            .flat_map(|(_, s)| s.iter())
            .count(),
        13
    );
    assert_eq!(
        visited_positions_snake(&test_moves)
            .iter()
            .flat_map(|(_, s)| s.iter())
            .count(),
        1
    );

    let (_, moves) = parse::input(INPUT)?;
    let p1_solution = visited_positions(&moves)
        .iter()
        .flat_map(|(_, s)| s.iter())
        .count();
    let p2_solution = visited_positions_snake(&moves)
        .iter()
        .flat_map(|(_, s)| s.iter())
        .count();

    println!("Problem 1: {p1_solution}\nProblem 2: {p2_solution}");

    Ok(())
}

fn visited_positions(moves: &[Move]) -> HashMap<i32, HashSet<i32>> {
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);

    let mut visited = HashMap::new();
    visited.insert(0, once(0).collect());

    for direction in moves
        .iter()
        .flat_map(|move_| repeat(move_.direction).take(move_.amount as usize))
    {
        head_pos = match (head_pos, direction) {
            ((x, y), Direction::Up) => (x, y + 1),
            ((x, y), Direction::Down) => (x, y - 1),
            ((x, y), Direction::Left) => (x - 1, y),
            ((x, y), Direction::Right) => (x + 1, y),
        };
        tail_pos = move_towards(tail_pos, head_pos);
        let (x, y) = tail_pos;
        visited
            .entry(x)
            .and_modify(|ys: &mut HashSet<i32>| {
                ys.insert(y);
            })
            .or_insert_with(|| once(y).collect());
    }

    visited
}

fn visited_positions_snake(moves: &[Move]) -> HashMap<i32, HashSet<i32>> {
    let mut snake: [(i32, i32); 10] = [(0, 0); 10];

    let mut visited = HashMap::new();
    visited.insert(0, once(0).collect());

    for direction in moves
        .iter()
        .flat_map(|move_| repeat(move_.direction).take(move_.amount as usize))
    {
        snake[0] = match (snake[0], direction) {
            ((x, y), Direction::Up) => (x, y + 1),
            ((x, y), Direction::Down) => (x, y - 1),
            ((x, y), Direction::Left) => (x - 1, y),
            ((x, y), Direction::Right) => (x + 1, y),
        };
        for (ix, (prev, pos)) in snake
            .iter()
            .copied()
            .tuple_windows()
            .collect::<Vec<_>>()
            .into_iter()
            .enumerate()
        {
            snake[ix + 1] = move_towards(pos, prev);
        }
        let (x, y) = snake[9];
        visited
            .entry(x)
            .and_modify(|ys: &mut HashSet<i32>| {
                ys.insert(y);
            })
            .or_insert_with(|| once(y).collect());
    }

    visited
}

fn adjacent(this: (i32, i32), that: (i32, i32)) -> bool {
    let ((x0, y0), (x1, y1)) = (this, that);
    (x0 - x1).abs() <= 1 && (y0 - y1).abs() <= 1
}

fn move_towards(this: (i32, i32), that: (i32, i32)) -> (i32, i32) {
    if adjacent(this, that) {
        this
    } else {
        let ((x0, y0), (x1, y1)) = (this, that);
        let dx = (x1 - x0).signum();
        let dy = (y1 - y0).signum();

        (x0 + dx, y0 + dy)
    }
}

mod parse {
    use nom::{
        character::complete::{digit1, line_ending, one_of, space1},
        combinator::{map, map_res},
        multi::many1,
        sequence::{separated_pair, terminated},
        IResult,
    };

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    #[derive(Debug)]
    pub struct Move {
        pub amount: i32,
        pub direction: Direction,
    }

    pub fn input(input: &str) -> IResult<&str, Vec<Move>> {
        many1(terminated(
            map(
                separated_pair(
                    map(one_of("UDLR"), |c| match c {
                        'U' => Direction::Up,
                        'D' => Direction::Down,
                        'L' => Direction::Left,
                        'R' => Direction::Right,
                        _ => unreachable!("{c} is not a valid side"),
                    }),
                    space1,
                    map_res(digit1, str::parse),
                ),
                |(direction, amount): (_, i32)| Move { direction, amount },
            ),
            line_ending,
        ))(input)
    }
}
