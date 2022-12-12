use std::collections::{HashSet, VecDeque};

use color_eyre::{Report, Result};
use flagset::{flags, FlagSet};
use parse::HeightMapPoint;

#[allow(dead_code)]
fn empty_option_err() -> Report {
    use std::io::{Error, ErrorKind::Other};
    Error::new(Other, "Option was empty").into()
}

static INPUT: &str = include_str!("input");

fn main() -> Result<()> {
    color_eyre::install()?;

    let (width, height_map) = parse::input(INPUT)?.1;
    let height = height_map.len() / width;

    let p1_solution = bfs((width, height), &height_map);
    let p2_solution = multi_bfs((width, height), &height_map);

    println!("Problem 1: {p1_solution}\nProblem 2: {p2_solution}");

    Ok(())
}

fn multi_bfs((width, height): (usize, usize), height_map: &[HeightMapPoint]) -> usize {
    println!(
        "Building Flow Map ({width}x{height}: {} points)",
        height_map.len()
    );
    let flow_map = heigh_map_to_flow_map((width, height), height_map);
    let mut bfs_map = vec![usize::MAX; width * height];

    let mut reachable = VecDeque::new();
    for start in
        height_map
            .iter()
            .enumerate()
            .filter_map(|(ix, point)| if point.height() == 0 { Some(ix) } else { None })
    {
        bfs_map[start] = 0;
        reachable.push_back((start, (start % width, start / width)));
    }

    let (end, _) = height_map
        .iter()
        .enumerate()
        .find(|(_, point)| matches!(point, HeightMapPoint::End))
        .expect("End point");

    println!("Running BFS...");
    let mut processed = HashSet::new();
    while let Some((ix, (x, y))) = reachable.pop_front() {
        if !processed.insert(ix) {
            continue;
        }

        if reachable.len() > width * height {
            panic!(
                "Using more than {width}*{height}={} elements, that's a bug",
                width * height
            );
        }

        let distance = bfs_map[ix];
        for reached in flow_map[ix]
            .into_iter()
            .filter_map(|dir| move_point((width, height), (x, y), dir))
        {
            if bfs_map[reached] <= distance {
                continue;
            }

            if reached == end {
                return distance + 1;
            }

            bfs_map[reached] = distance + 1;
            reachable.push_back((reached, (reached % width, reached / width)));
        }
    }

    bfs_map[end]
}

fn bfs((width, height): (usize, usize), height_map: &[HeightMapPoint]) -> usize {
    println!(
        "Building Flow Map ({width}x{height}: {} points)",
        height_map.len()
    );
    let flow_map = heigh_map_to_flow_map((width, height), height_map);
    let mut bfs_map = vec![usize::MAX; width * height];

    let (start, _) = height_map
        .iter()
        .enumerate()
        .find(|(_, point)| matches!(point, HeightMapPoint::Start))
        .expect("Start point");
    bfs_map[start] = 0;

    let (end, _) = height_map
        .iter()
        .enumerate()
        .find(|(_, point)| matches!(point, HeightMapPoint::End))
        .expect("End point");

    println!("Running BFS...");
    let mut reachable = VecDeque::new();
    reachable.push_back((start, (start % width, start / width)));
    let mut processed = HashSet::new();
    while let Some((ix, (x, y))) = reachable.pop_front() {
        if !processed.insert(ix) {
            continue;
        }

        if reachable.len() > width * height {
            panic!(
                "Using more than {width}*{height}={} elements, that's a bug",
                width * height
            );
        }

        let distance = bfs_map[ix];
        for reached in flow_map[ix]
            .into_iter()
            .filter_map(|dir| move_point((width, height), (x, y), dir))
        {
            if bfs_map[reached] <= distance {
                continue;
            }

            if reached == end {
                return distance + 1;
            }

            bfs_map[reached] = distance + 1;
            reachable.push_back((reached, (reached % width, reached / width)));
        }
    }

    bfs_map[end]
}

fn move_point(
    (width, height): (usize, usize),
    (x, y): (usize, usize),
    dir: Direction,
) -> Option<usize> {
    let cond = match dir {
        Direction::Up => y > 0,
        Direction::Down => y < height - 1,
        Direction::Left => x > 0,
        Direction::Right => x < width - 1,
    };

    if !cond {
        return None;
    }

    Some(match dir {
        Direction::Up => x + (y - 1) * width,
        Direction::Down => x + (y + 1) * width,
        Direction::Left => x - 1 + y * width,
        Direction::Right => x + 1 + y * width,
    })
}

fn heigh_map_to_flow_map(
    (width, height): (usize, usize),
    height_map: &[HeightMapPoint],
) -> Vec<FlagSet<Direction>> {
    height_map
        .iter()
        .enumerate()
        .map(|(ix, point)| {
            let (x, y) = (ix % width, ix / width);

            let mut set = FlagSet::<Direction>::default();

            if move_point((width, height), (x, y), Direction::Up)
                .and_then(|ix| height_map.get(ix))
                .copied()
                .map(|p| point.can_move_to(p))
                .unwrap_or(false)
            {
                set |= Direction::Up;
            };
            if move_point((width, height), (x, y), Direction::Down)
                .and_then(|ix| height_map.get(ix))
                .copied()
                .map(|p| point.can_move_to(p))
                .unwrap_or(false)
            {
                set |= Direction::Down;
            }
            if move_point((width, height), (x, y), Direction::Left)
                .and_then(|ix| height_map.get(ix))
                .copied()
                .map(|p| point.can_move_to(p))
                .unwrap_or(false)
            {
                set |= Direction::Left;
            }
            if move_point((width, height), (x, y), Direction::Right)
                .and_then(|ix| height_map.get(ix))
                .copied()
                .map(|p| point.can_move_to(p))
                .unwrap_or(false)
            {
                set |= Direction::Right;
            }

            set
        })
        .collect()
}

flags! {
    enum Direction : u8 {
        Up,
        Down,
        Left,
        Right,
    }
}

mod parse {
    use nom::{
        character::complete::{line_ending, one_of},
        combinator::{eof, map_res},
        multi::many1,
        sequence::terminated,
        IResult, Parser,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum HeightMapPoint {
        Start,
        Point(u8),
        End,
    }

    impl HeightMapPoint {
        pub fn height(self) -> u8 {
            match self {
                HeightMapPoint::Start => 0,
                HeightMapPoint::Point(h) => h,
                HeightMapPoint::End => b'z' - b'a',
            }
        }

        pub fn can_move_to(self, to: HeightMapPoint) -> bool {
            to.height() <= self.height() + 1
        }
    }

    impl TryFrom<char> for HeightMapPoint {
        type Error = ();

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'a'..='z' => Ok(HeightMapPoint::Point(value as u8 - b'a')),
                'S' => Ok(HeightMapPoint::Start),
                'E' => Ok(HeightMapPoint::End),
                _ => Err(()),
            }
        }
    }

    pub fn input(input: &str) -> IResult<&str, (usize, Vec<HeightMapPoint>)> {
        terminated(
            many1(terminated(
                many1(map_res(
                    one_of("abcdefghijklmnopqrstuvwxyzSE"),
                    HeightMapPoint::try_from,
                )),
                line_ending,
            ))
            .map(|points| {
                let width = points[0].len();
                (
                    width,
                    points
                        .into_iter()
                        .flat_map(|line| line.into_iter())
                        .collect(),
                )
            }),
            eof,
        )(input)
    }
}

#[cfg(test)]
mod test {
    use crate::{bfs, heigh_map_to_flow_map, multi_bfs, parse};
    use color_eyre::Result;
    #[allow(unused)]
    use pretty_assertions::{assert_eq, assert_ne};

    static INPUT: &str = include_str!("test_input");

    #[test]
    fn parse_test_input() -> Result<()> {
        let (rest, (width, points)) = parse::input(INPUT)?;

        assert_eq!(rest, "");
        assert_eq!(width, 8);

        let height = points.len() / width;
        let _flow_map = heigh_map_to_flow_map((width, height), &points);

        Ok(())
    }

    #[test]
    fn parse_input() -> Result<()> {
        let (rest, (width, points)) = parse::input(crate::INPUT)?;

        assert_eq!(rest, "");
        assert_eq!(width, 136);

        let height = points.len() / width;
        let _flow_map = heigh_map_to_flow_map((width, height), &points);

        Ok(())
    }

    #[test]
    fn problem_1() -> Result<()> {
        let (width, points) = parse::input(INPUT)?.1;

        let height = points.len() / width;
        assert_eq!(bfs((width, height), &points), 31);

        Ok(())
    }

    #[test]
    fn problem_2() -> Result<()> {
        let (width, points) = parse::input(INPUT)?.1;

        let height = points.len() / width;
        assert_eq!(multi_bfs((width, height), &points), 29);

        Ok(())
    }
}
