use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Match {
    Lose,
    Draw,
    Win,
}

impl Match {
    fn serialized(match_result: char) -> Self {
        match match_result {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            a => unreachable!("Invalid hand: {a}"),
        }
    }
    fn points(self) -> u32 {
        match self {
            Match::Lose => 0,
            Match::Draw => 3,
            Match::Win => 6,
        }
    }
}

impl Hand {
    fn serialized(hand: char) -> Self {
        match hand {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            a => unreachable!("Invalid hand: {a}"),
        }
    }

    fn match_result(self, opponent: Self) -> Match {
        match (self, opponent) {
            (Hand::Rock, Hand::Rock) => Match::Draw,
            (Hand::Rock, Hand::Paper) => Match::Lose,
            (Hand::Rock, Hand::Scissors) => Match::Win,
            (Hand::Paper, Hand::Rock) => Match::Win,
            (Hand::Paper, Hand::Paper) => Match::Draw,
            (Hand::Paper, Hand::Scissors) => Match::Lose,
            (Hand::Scissors, Hand::Rock) => Match::Lose,
            (Hand::Scissors, Hand::Paper) => Match::Win,
            (Hand::Scissors, Hand::Scissors) => Match::Draw,
        }
    }

    fn hand_from_result(self, result: Match) -> Self {
        match (self, result) {
            (Hand::Rock, Match::Lose) => Hand::Scissors,
            (Hand::Rock, Match::Draw) => Hand::Rock,
            (Hand::Rock, Match::Win) => Hand::Paper,
            (Hand::Paper, Match::Lose) => Hand::Rock,
            (Hand::Paper, Match::Draw) => Hand::Paper,
            (Hand::Paper, Match::Win) => Hand::Scissors,
            (Hand::Scissors, Match::Lose) => Hand::Paper,
            (Hand::Scissors, Match::Draw) => Hand::Scissors,
            (Hand::Scissors, Match::Win) => Hand::Rock,
        }
    }

    fn points(self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut matches = vec![];
    let mut strategy = vec![];
    for line in BufReader::new(File::open("input")?).lines() {
        let line = line?;
        let opponent = Hand::serialized(line.chars().next().unwrap());
        let strat = Match::serialized(line.chars().last().unwrap());
        let mine = Hand::serialized(line.chars().last().unwrap());
        matches.push((mine, mine.match_result(opponent)));
        strategy.push((opponent.hand_from_result(strat), strat));
    }

    let p1_solution: u32 = matches.iter().map(|(h, m)| h.points() + m.points()).sum();
    println!("Problem 1: {}", p1_solution);

    let p2_solution: u32 = strategy.iter().map(|(h, m)| h.points() + m.points()).sum();
    println!("Problem 2: {}", p2_solution);

    Ok(())
}
