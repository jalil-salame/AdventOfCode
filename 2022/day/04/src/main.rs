use std::{
    io,
    ops::{Deref, RangeInclusive},
    str::FromStr,
};

use color_eyre::{Report, Result};

#[allow(dead_code)]
fn empty_option_err() -> Report {
    use std::io::{Error, ErrorKind::Other};
    Error::new(Other, "no hyphen found").into()
}

#[derive(Debug, Clone)]
struct SectionRange(RangeInclusive<u32>);

impl SectionRange {
    fn contains_range(&self, other: &SectionRange) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn contains_or_is_contained_by(&self, other: &SectionRange) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }

    fn overlaps(&self, other: &SectionRange) -> bool {
        self.contains(other.start()) || other.contains(self.start())
    }
}

impl Deref for SectionRange {
    type Target = RangeInclusive<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for SectionRange {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or_else(empty_option_err)?;
        let (start, end) = (start.parse::<u32>()?, end.parse::<u32>()?);
        Ok(Self(start..=end))
    }
}

static INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut p1_solution = 0;
    let mut p2_solution = 0;

    for line in INPUT.lines() {
        let (l, r) = line
            .split_once(',')
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "no comma found"))?;
        let (l, r) = (l.parse::<SectionRange>()?, r.parse::<SectionRange>()?);
        if l.contains_or_is_contained_by(&r) {
            p1_solution += 1;
        }
        if l.overlaps(&r) {
            p2_solution += 1;
        }
    }

    println!("Problem 1: {}", p1_solution);

    println!("Problem 2: {}", p2_solution);

    Ok(())
}
