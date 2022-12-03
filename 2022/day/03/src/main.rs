use color_eyre::Result;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Item(char);

impl Item {
    fn new(c: char) -> Option<Self> {
        if c.is_alphabetic() {
            Some(Self(c))
        } else {
            None
        }
    }

    fn inherent_points(self) -> u32 {
        match self.0 {
            'a'..='z' => (self.0 as u8 - b'a' + 1).into(),
            'A'..='Z' => (self.0 as u8 - b'A' + 27).into(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Rucksack {
    storage: (Vec<Item>, Vec<Item>),
}

impl Rucksack {
    fn duplicated_item(&self) -> Option<Item> {
        self.storage
            .0
            .iter()
            .find(|x| self.storage.1.contains(x))
            .copied()
    }

    fn items(&self) -> Vec<Item> {
        let mut res = self.storage.0.clone();
        res.extend(self.storage.1.iter());
        res
    }

    fn badge(first: &Self, second: &Self, third: &Self) -> Option<Item> {
        let hash_1: HashSet<_> = first.items().into_iter().collect();
        let hash_2: HashSet<_> = second.items().into_iter().collect();
        let hash_3: HashSet<_> = third.items().into_iter().collect();

        let collect: HashSet<_> = hash_1.intersection(&hash_2).copied().collect();
        collect.intersection(&hash_3).next().copied()
    }
}

#[derive(Debug, Error)]
enum ParseRucksackError {
    #[error("odd length")]
    OddLength,
    #[error("multiline string")]
    MultilineStr,
}

impl FromStr for Rucksack {
    type Err = ParseRucksackError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let (Some(line), None) = (lines.next(), lines.next()) else { return Err(ParseRucksackError::MultilineStr) };
        let mut line: Vec<_> = line.chars().filter_map(Item::new).collect();
        let len = line.len();
        if len % 2 == 1 {
            Err(ParseRucksackError::OddLength)
        } else {
            let last = line.split_off(len / 2);
            Ok(Self {
                storage: (line, last),
            })
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut points = 0;
    let mut group_points = 0;
    let mut group = vec![];
    for line in BufReader::new(File::open("input")?).lines() {
        let rucksack: Rucksack = line?.parse()?;
        points += rucksack.duplicated_item().unwrap().inherent_points();
        if group.len() == 2 {
            let (first, second) = (group.pop().unwrap(), group.pop().unwrap());
            group_points += Rucksack::badge(&first, &second, &rucksack)
                .unwrap()
                .inherent_points();
        } else {
            group.push(rucksack);
        }
    }

    let p1_solution = points;
    println!("Problem 1: {}", p1_solution);

    let p2_solution = group_points;
    println!("Problem 2: {}", p2_solution);

    Ok(())
}
