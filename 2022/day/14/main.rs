use color_eyre::{Report, Result};
use itertools::Itertools;
use parse::Coord;
use std::{
    fmt::{Display, Write},
    time::Instant,
};

#[allow(dead_code)]
fn empty_option_err() -> Report {
    use std::io::{Error, ErrorKind::Other};
    Error::new(Other, "Option was empty").into()
}

static INPUT: &str = include_str!("input");

fn main() -> Result<()> {
    color_eyre::install()?;

    let now = Instant::now();

    let paths = {
        let now = Instant::now();

        let data = parse::input(INPUT)?.1;

        println!("Parsing took:    {:>16?}", now.elapsed());
        data
    };

    let now_ = Instant::now();

    println!("Processing took: {:>16?}", now_.elapsed());

    let problem_1_solution = {
        let now = Instant::now();

        let mut grid: Grid = paths.as_slice().into();

        let solution = grid.deposit_all_the_sand((500, 0));

        println!("Problem 1 took:  {:>16?}", now.elapsed());
        solution
    };

    let problem_2_solution = {
        let now = Instant::now();

        let mut grid = Grid::from_paths_with_floor(&paths);

        let solution = grid.deposit_all_the_sand((500, 0));

        println!("Problem 2 took:  {:>16?}", now.elapsed());
        solution
    };

    println!("Total runtime:   {:>16?}", now.elapsed());
    println!("----------------O----------------");
    println!("Problem 1:       {problem_1_solution:>16}");
    println!("Problem 2:       {problem_2_solution:>16}");

    Ok(())
}

fn maximum_dimentions(paths: &[parse::Path]) -> (Coord, Coord) {
    let x = paths.iter().flatten().map(|(x, _)| x).minmax();
    let y = paths.iter().flatten().map(|(_, y)| y).minmax();

    let x = match x {
        itertools::MinMaxResult::NoElements => unreachable!("Invalid input"),
        itertools::MinMaxResult::OneElement(&x) => (x, x),
        itertools::MinMaxResult::MinMax(&x_min, &x_max) => (x_min, x_max),
    };

    let y = match y {
        itertools::MinMaxResult::NoElements => unreachable!("Invalid input"),
        itertools::MinMaxResult::OneElement(&y) => (y, y),
        itertools::MinMaxResult::MinMax(&y_min, &y_max) => (y_min, y_max),
    };

    ((x.0.min(500), 0), (x.1.max(500), y.1))
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Air,
    Rock,
    Sand,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Grid {
    dimentions: (Coord, Coord),
    width: usize,
    cells: Vec<Cell>,
}

impl Cell {
    fn as_char(&self) -> char {
        match self {
            Cell::Air => '.',
            Cell::Rock => '#',
            Cell::Sand => 'o',
        }
    }
}

impl Grid {
    fn from_paths_with_floor(paths: &[parse::Path]) -> Self {
        let dimentions = maximum_dimentions(paths);
        assert_eq!(dimentions.0 .1, 0);

        let height = dimentions.1 .1 + 2;

        let dimentions = (
            (dimentions.0 .0.min(500 - height), 0),
            (dimentions.1 .0.max(500 + height), height),
        );

        let offset_x = dimentions.0 .0;
        let offset_y = dimentions.0 .1;

        let width = (dimentions.1 .0) - offset_x + 1;
        let height = (dimentions.1 .1) - offset_y + 1;

        let mut cells = vec![Cell::Air; width * height];
        cells[width * (height - 1)..].fill(Cell::Rock);

        for path in paths.iter() {
            for (from, to) in path.iter().tuple_windows() {
                // print!("{from:?} -> {to:?}: ");
                if from.1 == to.1 {
                    let f = from.0 - offset_x + from.1 * width;
                    let t = to.0 - offset_x + from.1 * width;

                    let start = f.min(t);
                    let end = f.max(t);

                    // println!("[{start}..={end}]");
                    cells[start..=end].fill(Cell::Rock);
                } else {
                    let start_x = from.0 - offset_x;

                    let f = from.1;
                    let t = to.1;

                    // println!("{start_x}, {}..={}", from.1, to.1);
                    for y in f.min(t)..=f.max(t) {
                        cells[start_x + y * width] = Cell::Rock;
                    }
                }
            }
        }

        Self {
            dimentions,
            width,
            cells,
        }
    }

    fn deposit_all_the_sand(&mut self, source: Coord) -> usize {
        let mut count = 0;

        #[cfg(test)]
        println!("{self}");
        while let Some(()) = self.deposit_sand(source) {
            #[cfg(test)]
            println!("{self}");
            count += 1;
        }

        count
    }

    fn deposit_sand(&mut self, (mut x, mut y): Coord) -> Option<()> {
        if !matches!(self.cells[self.as_index((x, y))], Cell::Air) {
            #[cfg(test)]
            println!("Covered Source");
            return None;
        }

        loop {
            let cell = self.cells.get(self.as_index((x, y + 1)))?;

            if matches!(cell, Cell::Air) {
                y += 1;
                continue;
            }

            // Fall off to the left
            if x <= (self.dimentions.0 .0) {
                #[cfg(test)]
                println!("fell to the left");
                return None;
            }

            let cell = self.cells.get(self.as_index((x - 1, y + 1)))?;

            if matches!(cell, Cell::Air) {
                x -= 1;
                y += 1;
                continue;
            }

            // Fall off to the right
            if x >= (self.dimentions.1 .0) {
                #[cfg(test)]
                println!("fell to the right");
                return None;
            }

            let cell = self.cells.get(self.as_index((x + 1, y + 1)))?;

            if matches!(cell, Cell::Air) {
                x += 1;
                y += 1;
                continue;
            }

            let ix = self.as_index((x, y));
            self.cells[ix] = Cell::Sand;
            return Some(());
        }
    }

    fn as_index(&self, (x, y): Coord) -> usize {
        x - (self.dimentions.0 .0) + y * self.width
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.cells.iter().chunks(self.width) {
            f.write_str(&line.map(Cell::as_char).collect::<String>())?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl From<&[parse::Path]> for Grid {
    fn from(paths: &[parse::Path]) -> Self {
        let dimentions = maximum_dimentions(paths);
        assert_eq!(dimentions.0 .1, 0);

        let offset_x = dimentions.0 .0;
        let offset_y = dimentions.0 .1;

        let width = (dimentions.1 .0) - offset_x + 1;
        let height = (dimentions.1 .1) - offset_y + 1;

        let mut cells = vec![Cell::Air; width * height];

        for path in paths.iter() {
            for (from, to) in path.iter().tuple_windows() {
                // print!("{from:?} -> {to:?}: ");
                if from.1 == to.1 {
                    let f = from.0 - offset_x + from.1 * width;
                    let t = to.0 - offset_x + from.1 * width;

                    let start = f.min(t);
                    let end = f.max(t);

                    // println!("[{start}..={end}]");
                    cells[start..=end].fill(Cell::Rock);
                } else {
                    let start_x = from.0 - offset_x;

                    let f = from.1;
                    let t = to.1;

                    // println!("{start_x}, {}..={}", from.1, to.1);
                    for y in f.min(t)..=f.max(t) {
                        cells[start_x + y * width] = Cell::Rock;
                    }
                }
            }
        }

        Self {
            dimentions,
            width,
            cells,
        }
    }
}

mod parse {
    use nom::{
        character::{complete::line_ending, streaming::char},
        combinator::eof,
        multi::{many1, separated_list1},
        sequence::{separated_pair, terminated},
        IResult, Parser,
    };

    pub type Coord = (usize, usize);
    pub type Path = Vec<Coord>;

    pub fn input(input: &str) -> IResult<&str, Vec<Path>> {
        terminated(
            many1(terminated(
                separated_list1(
                    nom::bytes::complete::tag(" -> "),
                    separated_pair(
                        nom::character::complete::u32,
                        char(','),
                        nom::character::complete::u32,
                    )
                    .map(|(x, y)| (x as usize, y as usize)),
                ),
                line_ending,
            )),
            eof,
        )(input)
    }
}

#[cfg(test)]
mod test {
    use crate::{maximum_dimentions, parse, Grid};
    use color_eyre::Result;
    #[allow(unused)]
    use pretty_assertions::{assert_eq, assert_ne};

    static INPUT: &str = include_str!("test_input");

    #[test]
    fn parse_test_input() -> Result<()> {
        let (rest, data) = parse::input(INPUT)?;

        assert_eq!(rest, "");
        assert_eq!(data.len(), 2);

        Ok(())
    }

    #[test]
    fn parse_input() -> Result<()> {
        let (rest, data) = parse::input(crate::INPUT)?;

        assert_eq!(rest, "");
        assert_eq!(data.len(), 129);

        Ok(())
    }

    #[test]
    fn problem_1() -> Result<()> {
        let data = parse::input(INPUT)?.1;

        let dimentions = maximum_dimentions(&data);
        assert_eq!(dimentions, ((494, 0), (503, 9)));

        let mut grid: Grid = data.as_slice().into();
        let count = grid.deposit_all_the_sand((500, 0));
        assert_eq!(count, 24);

        Ok(())
    }

    #[test]
    fn problem_2() -> Result<()> {
        let paths = parse::input(INPUT)?.1;

        let mut grid: Grid = Grid::from_paths_with_floor(&paths);
        let count = grid.deposit_all_the_sand((500, 0));
        println!("{:?}", grid.dimentions);
        assert_eq!(
            format!("{grid}"),
            "...........o...........
..........ooo..........
.........ooooo.........
........ooooooo........
.......oo#ooo##o.......
......ooo#ooo#ooo......
.....oo###ooo#oooo.....
....oooo.oooo#ooooo....
...oooooooooo#oooooo...
..ooo#########ooooooo..
.ooooo.......ooooooooo.
#######################
"
        );
        assert_eq!(count, 93);

        Ok(())
    }
}
