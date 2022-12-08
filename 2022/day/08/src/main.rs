use std::str::Lines;

use color_eyre::{Report, Result};

#[allow(dead_code)]
fn empty_option_err() -> Report {
    use std::io::{Error, ErrorKind::Other};
    Error::new(Other, "Option was empty").into()
}

static INPUT: &str = include_str!("../input");
static TEST_INPUT: &str = "30373
25512
65332
33549
35390
";

fn main() -> Result<()> {
    color_eyre::install()?;

    assert_eq!(visible_trees(TEST_INPUT), 21);
    let p1_solution = visible_trees(INPUT);
    assert_eq!(best_scenic_score(TEST_INPUT), 8);
    let p2_solution = best_scenic_score(INPUT);

    println!("Problem 1: {p1_solution}\nProblem 2: {p2_solution}");

    Ok(())
}

fn best_scenic_score(forest: &str) -> usize {
    let forest: Vec<_> = Forest::from(forest).collect();
    forest
        .iter()
        .enumerate()
        .filter_map(|(ix, line)| {
            line.iter()
                .enumerate()
                .map(|(iy, _)| scenic_score(&forest, (ix, iy)))
                .max()
        })
        .max()
        .unwrap()
}

fn scenic_score(forest: &[Vec<u8>], tree_pos: (usize, usize)) -> usize {
    let (x, y) = tree_pos;
    let height = forest[x][y];
    let up = {
        let forest_slice = &forest[..x];
        let count = forest_slice
            .iter()
            .rev()
            .map(|line| line[y])
            .take_while(|tree| tree < &height)
            .count();
        if count == forest_slice.len() {
            count
        } else {
            count + 1
        }
    };
    let down = {
        let forest_slice = &forest[x + 1..];
        let count = forest_slice
            .iter()
            .map(|line| line[y])
            .take_while(|tree| tree < &height)
            .count();
        if count == forest_slice.len() {
            count
        } else {
            count + 1
        }
    };
    let left = {
        let line = &forest[x][..y];
        let count = line
            .iter()
            .rev()
            .copied()
            .take_while(|tree| tree < &height)
            .count();
        if count == line.len() {
            count
        } else {
            count + 1
        }
    };
    let right = {
        let line = &forest[x][y + 1..];
        let count = line
            .iter()
            .copied()
            .take_while(|tree| tree < &height)
            .count();
        if count == line.len() {
            count
        } else {
            count + 1
        }
    };
    up * down * left * right
}

fn visible_trees(forest: &str) -> usize {
    let forest: Vec<_> = Forest::from(forest).collect();
    forest
        .iter()
        .enumerate()
        .map(|(ix, line)| {
            if ix == 0 || ix == forest.len() - 1 {
                line.len()
            } else {
                line.iter()
                    .enumerate()
                    .map(|(iy, height)| {
                        usize::from(
                            iy == 0
                                || iy == line.len() - 1
                                || line[..iy].iter().all(|tree| tree < height) // left visibility,
                                || line[iy+1..].iter().all(|tree| tree < height) // right visibility,
                                || forest[..ix].iter().map(|line| line[iy]).all(|tree| tree < *height) // top visibility
                                || forest[ix+1..].iter().map(|line| line[iy]).all(|tree| tree < *height), // bottom visibility
                        )
                    })
                    .sum()
            }
        })
        .sum()
}

#[repr(transparent)]
struct Forest<'a>(Lines<'a>);

impl<'a> From<&'a str> for Forest<'a> {
    fn from(s: &'a str) -> Self {
        Forest(s.lines())
    }
}

impl Iterator for Forest<'_> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.0.next()?;
        Some(line.bytes().map(|b| b - b'0').collect())
    }
}
