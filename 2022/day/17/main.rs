use color_eyre::Result;
use parse::Direction;
use std::{borrow::Cow, fmt::Debug, time::Instant};

static ROCK_PATTERN: &[RockPattern] = &[
    RockPattern {
        width: 4,
        cells: Cow::Borrowed(&[Cell::Rock, Cell::Rock, Cell::Rock, Cell::Rock]),
    },
    RockPattern {
        width: 3,
        cells: Cow::Borrowed(&[
            Cell::Air,
            Cell::Rock,
            Cell::Air,
            Cell::Rock,
            Cell::Rock,
            Cell::Rock,
            Cell::Air,
            Cell::Rock,
            Cell::Air,
        ]),
    },
    RockPattern {
        width: 3,
        cells: Cow::Borrowed(&[
            Cell::Air,
            Cell::Air,
            Cell::Rock,
            Cell::Air,
            Cell::Air,
            Cell::Rock,
            Cell::Rock,
            Cell::Rock,
            Cell::Rock,
        ]),
    },
    RockPattern {
        width: 1,
        cells: Cow::Borrowed(&[Cell::Rock, Cell::Rock, Cell::Rock, Cell::Rock]),
    },
    RockPattern {
        width: 2,
        cells: Cow::Borrowed(&[Cell::Rock, Cell::Rock, Cell::Rock, Cell::Rock]),
    },
];

static INPUT: &str = include_str!("input");

#[derive(Clone, Copy)]
enum Cell {
    Air,
    Rock,
}

impl Cell {
    const fn as_char(self) -> char {
        match self {
            Cell::Air => '.',
            Cell::Rock => '#',
        }
    }

    const fn is_empty(self) -> bool {
        matches!(self, Cell::Air)
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

struct RockPattern<'a> {
    width: usize,
    cells: Cow<'a, [Cell]>,
}

impl Debug for RockPattern<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ele in self.rows().rev() {
            write!(f, "|")?;
            for cell in ele {
                write!(f, "{cell:?}")?;
            }
            writeln!(f, "|")?;
        }
        write!(
            f,
            "+{}+",
            vec!['-'; self.width].into_iter().collect::<String>()
        )
    }
}

impl RockPattern<'_> {
    fn let_pattern_fall(
        &mut self,
        pattern: &Self,
        directions: &[Direction],
        mut dir_ix: usize,
    ) -> usize {
        assert_eq!(self.width, 7);
        assert!(dir_ix < directions.len());

        let space_wants = pattern.height() + 3;
        let free_space = self.height() - self.rocks_height();

        let mut pos = match free_space.cmp(&space_wants) {
            std::cmp::Ordering::Less => {
                let padding = (space_wants - free_space) * self.width;
                self.cells.to_mut().extend(vec![Cell::Air; padding]);
                (2, self.height() - pattern.height())
            }
            std::cmp::Ordering::Equal => (2, self.height() - pattern.height()),
            std::cmp::Ordering::Greater => (
                2,
                self.height() - pattern.height() - (free_space - space_wants),
            ),
        };

        loop {
            let (x, y) = pos;

            let direction = directions[dir_ix];
            dir_ix = (dir_ix + 1) % directions.len();

            #[cfg(test)]
            println!("Pushed {direction:?} {pos:?}");

            let new_x = match (direction, x) {
                (Direction::Left, 0) => x,
                (Direction::Left, x) => x - 1,
                (Direction::Right, x) => {
                    if x > self.width - pattern.width - 1 {
                        x
                    } else {
                        x + 1
                    }
                }
            };

            let x = if self.rock_intersect_at(pattern, (new_x, y)) {
                #[cfg(test)]
                println!("Intersects");
                x
            } else {
                new_x
            };

            if y == 0 || self.rock_intersect_at(pattern, (x, y - 1)) {
                #[cfg(test)]
                println!("Settled at ({x},{y})");
                self.settle(pattern, (x, y));
                return dir_ix;
            };

            pos = (x, y - 1);
        }
    }

    fn settle(&mut self, other: &Self, (x, y): (usize, usize)) {
        for (s_row, o_row) in self
            .mut_rows()
            .skip(y)
            .take(other.height())
            .rev()
            .zip(other.rows())
        {
            s_row[x..x + o_row.len()].copy_from_slice(o_row);
        }
    }

    fn rock_intersect_at(&self, other: &Self, (x, y): (usize, usize)) -> bool {
        for (s_row, o_row) in self
            .rows()
            .skip(y)
            .take(other.height())
            .rev()
            .zip(other.rows())
        {
            for cell_pair in s_row.iter().skip(x).take(o_row.len()).zip(o_row) {
                match cell_pair {
                    (_, Cell::Air) => continue,
                    (Cell::Air, Cell::Rock) => continue,
                    (Cell::Rock, Cell::Rock) => return true,
                }
            }
        }

        false
    }

    fn mut_rows(&mut self) -> std::slice::ChunksMut<Cell> {
        self.cells.to_mut().chunks_mut(self.width)
    }

    fn rows(&self) -> std::slice::Chunks<Cell> {
        self.cells.as_ref().chunks(self.width)
    }

    fn height(&self) -> usize {
        self.cells.len() / self.width
    }

    fn rocks_height(&self) -> usize {
        let rows = self.cells.as_ref().chunks(self.width);

        for (ix, row) in rows.enumerate().rev() {
            if !row.iter().copied().all(Cell::is_empty) {
                return ix + 1;
            }
        }

        0
    }
}

fn final_height(steps: usize, directions: &[Direction]) -> usize {
    let mut chamber = RockPattern {
        width: 7,
        cells: Cow::Owned(vec![]),
    };
    let mut direction = 0;

    for ix in 0..steps {
        direction = chamber.let_pattern_fall(
            &ROCK_PATTERN[ix % ROCK_PATTERN.len()],
            directions,
            direction,
        );
    }

    chamber.rocks_height()
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let now = Instant::now();

    let directions = {
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

        let solution = final_height(2022, &directions);

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

mod parse {
    use nom::{
        branch::alt,
        character::{complete::multispace0, streaming::char},
        combinator::{eof, value},
        multi::many1,
        sequence::{pair, terminated},
        IResult,
    };

    #[derive(Debug, Clone, Copy)]
    pub enum Direction {
        Left,
        Right,
    }

    impl Direction {
        fn parse(input: &str) -> IResult<&str, Direction> {
            alt((
                value(Direction::Left, char('<')),
                value(Direction::Right, char('>')),
            ))(input)
        }
    }

    pub fn input(input: &str) -> IResult<&str, Vec<Direction>> {
        terminated(many1(Direction::parse), pair(multispace0, eof))(input)
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use crate::{final_height, parse, RockPattern, ROCK_PATTERN};
    use color_eyre::Result;
    #[allow(unused)]
    use pretty_assertions::{assert_eq, assert_ne};

    static INPUT: &str = include_str!("test_input");

    #[test]
    fn parse_test_input() -> Result<()> {
        let (rest, directions) = parse::input(INPUT)?;

        assert_eq!(rest, "");
        assert_eq!(directions.len(), 40);

        Ok(())
    }

    #[test]
    fn parse_input() -> Result<()> {
        let (rest, directions) = parse::input(crate::INPUT)?;

        assert_eq!(rest, "");
        assert_eq!(directions.len(), 10091);

        Ok(())
    }

    #[test]
    fn problem_1_sample() -> Result<()> {
        let directions = parse::input(INPUT)?.1;

        let mut chamber = RockPattern {
            width: 7,
            cells: Cow::Owned(vec![]),
        };

        let mut direction = 0;

        for ix in 0..10 {
            direction = chamber.let_pattern_fall(
                &ROCK_PATTERN[ix % ROCK_PATTERN.len()],
                &directions,
                direction,
            );
            println!("{chamber:?}\nHeight: {}", chamber.rocks_height());
        }

        assert_eq!(chamber.rocks_height(), 17);

        Ok(())
    }

    #[test]
    fn problem_1() -> Result<()> {
        let directions = parse::input(INPUT)?.1;

        assert_eq!(final_height(2022, &directions), 3068);

        Ok(())
    }

    #[test]
    #[ignore]
    fn problem_2() -> Result<()> {
        let _ = parse::input(INPUT)?.1;

        Ok(())
    }
}
