use color_eyre::Result;
use flagset::{flags, FlagSet};
use parse::Vec3;
use std::{collections::HashMap, time::Instant};

static INPUT: &str = include_str!("input");

fn main() -> Result<()> {
    color_eyre::install()?;

    let now = Instant::now();

    let points = {
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

        let solution = exposed_faces(&points);

        println!("Problem 1 took:  {:>16?}", now.elapsed());
        solution
    };

    let problem_2_solution = {
        let now = Instant::now();

        let solution = exposed_faces_strict(&points);

        println!("Problem 2 took:  {:>16?}", now.elapsed());
        solution
    };

    println!("Total runtime:   {:>16?}", now.elapsed());
    println!("----------------O----------------");
    println!("Problem 1:       {problem_1_solution:>16}");
    println!("Problem 2:       {problem_2_solution:>16}");

    Ok(())
}

fn exposed_faces_strict(points: &[Vec3]) -> isize {
    let mut cubes: HashMap<_, _> = cube_faces(points);

    let mut exposed = 0;

    for point in points {
        let faces = cubes.get(point).unwrap();
        exposed += 6;

        for face in faces.into_iter() {
            let adjacet = *point + face.to_vec3();

            if let Some(faces) = cubes.get_mut(&adjacet) {
                exposed -= 2;
                *faces -= face.opposing_face();
                *cubes.get_mut(point).unwrap() -= face;
                continue;
            }

            'outer: {
                for face in FlagSet::<Face>::full() - face.opposing_face() {
                    let lava = adjacet + face.to_vec3();
                    if cubes.get(&lava).is_none() {
                        break 'outer;
                    }
                }

                exposed -= 6;
                for face in FlagSet::<Face>::full() {
                    let lava = adjacet + face.to_vec3();
                    *cubes.get_mut(&lava).unwrap() -= face.opposing_face();
                }
            }
        }
    }

    exposed
}

fn exposed_faces(points: &[Vec3]) -> isize {
    let mut cubes: HashMap<_, _> = cube_faces(points);

    let mut exposed = 0;

    for point in points {
        let faces = cubes.get(point).unwrap();
        exposed += 6;

        for face in faces.into_iter() {
            if let Some(faces) = cubes.get_mut(&(*point + face.to_vec3())) {
                exposed -= 2;
                *faces -= face.opposing_face();
                *cubes.get_mut(point).unwrap() -= face;
            }
        }
    }

    exposed
}

fn cube_faces(points: &[Vec3]) -> HashMap<Vec3, FlagSet<Face>> {
    points
        .iter()
        .copied()
        .map(|p| (p, FlagSet::<Face>::full()))
        .collect()
}

flags! {
    enum Face : u8 {
        PosX,
        PosY,
        PosZ,
        NegX,
        NegY,
        NegZ,
    }
}

impl Face {
    fn opposing_face(self) -> Self {
        match self {
            Face::PosX => Face::NegX,
            Face::PosY => Face::NegY,
            Face::PosZ => Face::NegZ,
            Face::NegX => Face::PosX,
            Face::NegY => Face::PosY,
            Face::NegZ => Face::PosZ,
        }
    }
    fn to_vec3(self) -> Vec3 {
        match self {
            Face::PosX => (1, 0, 0).into(),
            Face::PosY => (0, 1, 0).into(),
            Face::PosZ => (0, 0, 1).into(),
            Face::NegX => (-1, 0, 0).into(),
            Face::NegY => (0, -1, 0).into(),
            Face::NegZ => (0, 0, -1).into(),
        }
    }
}

mod parse {
    use std::ops::Add;

    use nom::{
        character::{
            self,
            complete::{char, line_ending, multispace0},
        },
        combinator::map,
        multi::separated_list0,
        sequence::{delimited, terminated, tuple},
        IResult,
    };

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    pub struct Vec3 {
        pub x: i32,
        pub y: i32,
        pub z: i32,
    }

    impl Vec3 {
        fn parse(input: &str) -> IResult<&str, Self> {
            map(
                tuple((
                    character::complete::i32,
                    delimited(char(','), character::complete::i32, char(',')),
                    character::complete::i32,
                )),
                Vec3::from,
            )(input)
        }
    }

    impl Add for Vec3 {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }

    impl From<(i32, i32, i32)> for Vec3 {
        fn from((x, y, z): (i32, i32, i32)) -> Self {
            Self { x, y, z }
        }
    }

    pub fn input(input: &str) -> IResult<&str, Vec<Vec3>> {
        terminated(separated_list0(line_ending, Vec3::parse), multispace0)(input)
    }
}

#[cfg(test)]
mod test {
    use crate::{exposed_faces, exposed_faces_strict, parse};
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
        let points = parse::input(INPUT)?.1;

        assert_eq!(exposed_faces(&points), 64);

        Ok(())
    }

    #[test]
    fn problem_2() -> Result<()> {
        let points = parse::input(INPUT)?.1;

        assert_eq!(exposed_faces_strict(&points), 58);

        Ok(())
    }
}
