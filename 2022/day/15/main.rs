use color_eyre::{Report, Result};
use itertools::Itertools;
use parse::{Coord, SensorBeaconPair};
use std::{iter::successors, ops::RangeInclusive, time::Instant};

#[allow(dead_code)]
fn empty_option_err() -> Report {
    use std::io::{Error, ErrorKind::Other};
    Error::new(Other, "Option was empty").into()
}

static INPUT: &str = include_str!("input");

fn main() -> Result<()> {
    color_eyre::install()?;

    let now = Instant::now();

    let data = {
        let now = Instant::now();

        let data = parse::input(INPUT)?.1;

        println!("Parsing took:    {:>16?}", now.elapsed());
        data
    };

    let now_ = Instant::now();

    // Add common data structures to Problem 1 and Problem 2 here

    println!("Processing took: {:>16?}", now_.elapsed());

    let problem_1_solution = {
        let now = Instant::now();

        let solution = (LazyGrid { sensors: &data }).empty_at_row(2_000_000);

        println!("Problem 1 took:  {:>16?}", now.elapsed());
        solution
    };

    let problem_2_solution = {
        let now = Instant::now();

        let solution = (LazyGrid { sensors: &data }).avalilable_within_area(4_000_000);

        println!("Problem 2 took:  {:>16?}", now.elapsed());
        solution
    };

    println!("Total runtime:   {:>16?}", now.elapsed());
    println!("----------------O----------------");
    println!("Problem 1:       {problem_1_solution:>16}");
    println!("Problem 2:       {problem_2_solution:>16}");

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct LazyGrid<'a> {
    sensors: &'a [SensorBeaconPair],
}

impl LazyGrid<'_> {
    fn sensor_cover_ranges_at_row(self, row: i32) -> Vec<RangeInclusive<i32>> {
        self
            .sensors
            .iter()
            .filter_map(|&SensorBeaconPair { sensor, beacon }| {
                let distance = manhattan_distace(sensor, beacon);

                if (sensor.1 - distance..=sensor.1 + distance).contains(&row) {
                    let dist = (sensor.1 - row).abs();

                    let x_min = sensor.0 - (distance - dist);
                    let x_max = sensor.0 + (distance - dist);

                    #[cfg(test)]
                    println!("{sensor:>2?} {beacon:>2?}: {distance:>2} {dist:>2} -> {x_min:>2}..={x_max:2>}");

                    Some((x_min, x_max))
                } else {
                    None
                }
            })
            .sorted_unstable_by(|a, b| a.0.cmp(&b.0))
            .fold(vec![], |mut ranges, (x_min, x_max)| {
                if let Some((a, b)) = ranges.pop() {
                    if x_min <= b {
                        ranges.push((a, b.max(x_max)));
                    } else {
                        ranges.push((a, b));
                        ranges.push((x_min, x_max));
                    }
                } else {
                    ranges.push((x_min, x_max));
                }
                ranges
            })
            .into_iter()
            .map(|(min, max)| min..=max)
            .collect_vec()
    }

    fn avalilable_within_area(self, width: i32) -> i64 {
        for row in 0..=width {
            if row % 100 == 0 {
                print!(
                    "[{row:>w$}/{width}]\r",
                    w = successors(Some(width), |&n| (n >= 10).then_some(n / 10)).count()
                );
            }
            if let Some(x) = self.available_at_row(row, width) {
                return x as i64 * 4_000_000 + row as i64;
            }
        }

        unreachable!()
    }

    fn available_at_row(self, row: i32, width: i32) -> Option<i32> {
        let mut range = self
            .sensor_cover_ranges_at_row(row)
            .into_iter()
            .find(|range| range.contains(&0) || range.contains(&width))?;

        if range.contains(&0) && range.contains(&width) {
            None
        } else if range.contains(&0) {
            range.last().map(|n| n + 1)
        } else {
            range.next().map(|n| n - 1)
        }
    }

    fn empty_at_row(self, row: i32) -> usize {
        let ranges = self.sensor_cover_ranges_at_row(row);

        let beacons_on_row = self
            .sensors
            .iter()
            .map(|&SensorBeaconPair { sensor: _, beacon }| beacon)
            .filter(|(x, y)| y == &row && ranges.iter().any(|range| range.contains(x)))
            .unique()
            .count();

        #[cfg(test)]
        println!("{ranges:?}");

        ranges.into_iter().map(|range| range.count()).sum::<usize>() - beacons_on_row
    }
}

fn manhattan_distace((a, b): Coord, (c, d): Coord) -> i32 {
    (a - c).abs() + (b - d).abs()
}

mod parse {
    use nom::{
        bytes::complete::tag,
        character::{self, complete::line_ending},
        combinator::{eof, map},
        multi::many1,
        sequence::{preceded, separated_pair, terminated},
        IResult,
    };

    pub type Coord = (i32, i32);

    #[derive(Debug, Clone, Copy)]
    pub struct SensorBeaconPair {
        pub sensor: Coord,
        pub beacon: Coord,
    }

    impl SensorBeaconPair {
        fn parse(input: &str) -> IResult<&str, Self> {
            map(
                separated_pair(
                    preceded(
                        tag("Sensor at "),
                        separated_pair(
                            preceded(tag("x="), character::complete::i32),
                            tag(", "),
                            preceded(tag("y="), character::complete::i32),
                        ),
                    ),
                    tag(": "),
                    preceded(
                        tag("closest beacon is at "),
                        separated_pair(
                            preceded(tag("x="), character::complete::i32),
                            tag(", "),
                            preceded(tag("y="), character::complete::i32),
                        ),
                    ),
                ),
                SensorBeaconPair::from,
            )(input)
        }
    }

    impl From<(Coord, Coord)> for SensorBeaconPair {
        fn from((sensor, beacon): (Coord, Coord)) -> Self {
            Self { sensor, beacon }
        }
    }

    pub fn input(input: &str) -> IResult<&str, Vec<SensorBeaconPair>> {
        terminated(many1(terminated(SensorBeaconPair::parse, line_ending)), eof)(input)
    }
}

#[cfg(test)]
mod test {
    use crate::{parse, LazyGrid};
    use color_eyre::Result;
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
        let data = parse::input(INPUT)?.1;

        let grid = LazyGrid { sensors: &data };
        assert_eq!(grid.empty_at_row(10), 26);

        Ok(())
    }

    #[test]
    fn problem_2() -> Result<()> {
        let data = parse::input(INPUT)?.1;

        let grid = LazyGrid { sensors: &data };
        assert_eq!(grid.avalilable_within_area(20), 56000011);

        Ok(())
    }
}
