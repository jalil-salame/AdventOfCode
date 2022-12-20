use color_eyre::{Report, Result};
use std::time::Instant;

#[allow(dead_code)]
fn empty_option_err() -> Report {
    use std::io::{Error, ErrorKind::Other};
    Error::new(Other, "Option was empty").into()
}

static INPUT: &str = include_str!("input");

fn main() -> Result<()> {
    color_eyre::install()?;

    let now = Instant::now();

    let coordinates = {
        let now = Instant::now();

        let data = parse::input(INPUT)?.1;

        println!("Parsing took:    {:>16?}", now.elapsed());
        data
    };

    let now_ = Instant::now();

    // Add common data structures to Problem 1 and Problem 2 here
    let mixed = mix(&coordinates);

    println!("Processing took: {:>16?}", now_.elapsed());

    let problem_1_solution = {
        let now = Instant::now();

        let solution = problem_1_result(&mixed);

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

fn problem_1_result(mixed: &[i32]) -> i32 {
    let zero_pos = mixed.iter().position(|&n| n == 0).unwrap();
    retrieve_wrapping(mixed, 1000 + zero_pos)
        + retrieve_wrapping(mixed, 2000 + zero_pos)
        + retrieve_wrapping(mixed, 3000 + zero_pos)
}

fn retrieve_wrapping(slice: &[i32], ix: usize) -> i32 {
    slice[ix % slice.len()]
}

fn mix(coordinates: &[i32]) -> Vec<i32> {
    let mut decrypted = coordinates.to_vec();
    let len = decrypted.len();

    for n in coordinates {
        let old_ix = decrypted.iter().position(|x| x == n).unwrap();
        let value = decrypted[old_ix];

        let offset = (value as isize) % len as isize;

        let new_ix = (len + old_ix).checked_add_signed(offset).unwrap();
        let new_ix = new_ix % len;

        match new_ix.cmp(&old_ix) {
            std::cmp::Ordering::Less => {
                if offset.is_negative() {
                    decrypted.copy_within(new_ix..old_ix, new_ix + 1);
                } else {
                    decrypted.copy_within(old_ix + 1..len, old_ix);
                    decrypted[len - 1] = decrypted[0];
                    decrypted.copy_within(1..new_ix, 0);
                }
            }
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => {
                if offset.is_negative() {
                    decrypted.copy_within(0..old_ix, 1);
                    decrypted[0] = decrypted[len - 1];
                    decrypted.copy_within(new_ix..len - 1, new_ix + 1);
                } else {
                    decrypted.copy_within(old_ix + 1..=new_ix, old_ix);
                }
            }
        }

        decrypted[new_ix] = value;

        #[cfg(test)]
        println!("{decrypted:?}");
    }

    decrypted
}

mod parse {
    use nom::{
        character::complete::line_ending, combinator::eof, multi::many1, sequence::terminated,
        IResult,
    };

    pub fn input(input: &str) -> IResult<&str, Vec<i32>> {
        terminated(
            many1(terminated(nom::character::complete::i32, line_ending)),
            eof,
        )(input)
    }
}

#[cfg(test)]
mod test {
    use crate::{mix, parse, problem_1_result};
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
        let coordinates = parse::input(INPUT)?.1;

        println!(
            "Expected:
[1, 2, -3, 3, -2, 0, 4]
[2, 1, -3, 3, -2, 0, 4]
[1, -3, 2, 3, -2, 0, 4]
[1, 2, 3, -2, -3, 0, 4]
[1, 2, -2, -3, 0, 3, 4]
[1, 2, -3, 0, 3, 4, -2]
[1, 2, -3, 0, 3, 4, -2]
[1, 2, -3, 4, 0, 3, -2]

Got:"
        );

        println!("{coordinates:?}");

        let mixed = mix(&coordinates);

        assert_eq!(problem_1_result(&mixed), 3);
        // assert_eq!(&mixed, &[1, 2, -3, 4, 0, 3, -2]);

        Ok(())
    }

    #[test]
    #[ignore]
    fn problem_2() -> Result<()> {
        let _ = parse::input(INPUT)?.1;

        Ok(())
    }
}
