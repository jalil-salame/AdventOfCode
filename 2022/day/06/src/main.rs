use color_eyre::{Report, Result};

#[allow(dead_code)]
fn empty_option_err() -> Report {
    use std::io::{Error, ErrorKind::Other};
    Error::new(Other, "Option was empty").into()
}

static INPUT: &str = include_str!("../input");

fn is_uniq<T: PartialEq>(s: &[T]) -> bool {
    for (ix, x) in s.iter().enumerate() {
        if s[ix + 1..].contains(x) {
            return false;
        }
    }

    true
}

fn packet_start(input: &str) -> usize {
    const PREV_SIZE: usize = 3;
    let mut prev = [' '; PREV_SIZE];
    prev.copy_from_slice(&input.chars().collect::<Vec<_>>()[..PREV_SIZE]);
    for (ix, c) in input.chars().skip(PREV_SIZE).enumerate() {
        if !prev.contains(&c) && is_uniq(&prev) {
            return ix + PREV_SIZE + 1;
        } else {
            prev[ix % PREV_SIZE] = c;
        }
    }

    unreachable!("Input contains no start marker")
}

fn message_start(input: &str) -> usize {
    const PREV_SIZE: usize = 13;
    let mut prev = [' '; PREV_SIZE];
    prev.copy_from_slice(&input.chars().collect::<Vec<_>>()[..PREV_SIZE]);
    for (ix, c) in input.chars().skip(PREV_SIZE).enumerate() {
        if !prev.contains(&c) && is_uniq(&prev) {
            return ix + PREV_SIZE + 1;
        } else {
            prev[ix % PREV_SIZE] = c;
        }
    }

    unreachable!("Input contains no message marker")
}

fn main() -> Result<()> {
    color_eyre::install()?;

    assert_eq!(packet_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(packet_start("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(packet_start("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(packet_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(packet_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    let p1_solution = packet_start(INPUT);
    assert_eq!(message_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(message_start("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(message_start("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(message_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(message_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    let p2_solution = message_start(INPUT);

    println!("Problem 1: {p1_solution:?}\nProblem 2: {p2_solution}");

    Ok(())
}
