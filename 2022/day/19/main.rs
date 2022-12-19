use color_eyre::Result;
use parse::Blueprint;
use std::time::Instant;

static INPUT: &str = include_str!("input");

fn quality_level<const TIME: u32>(blueprints: &[Blueprint]) -> u32 {
    blueprints
        .iter()
        .enumerate()
        .map(|(ix, blueprint)| blueprint.geodes_opened::<TIME>() * (ix + 1) as u32)
        .sum()
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let now = Instant::now();

    let blueprints = {
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

        let solution = quality_level::<24>(&blueprints);

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
        branch,
        bytes::complete::tag,
        character::complete::{char, line_ending, multispace0, multispace1},
        combinator::{map, value},
        multi::{many1, separated_list1},
        sequence::{delimited, pair, separated_pair, terminated, tuple},
        IResult,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Resource {
        Ore,
        Clay,
        Geode,
        Obsidian,
    }

    #[derive(Debug)]
    pub struct ResourceCost {
        pub cost: u32,
        pub resource: Resource,
    }

    #[derive(Debug)]
    pub struct Blueprint {
        pub ore_robot_cost: Vec<ResourceCost>,
        pub clay_robot_cost: Vec<ResourceCost>,
        pub geode_robot_cost: Vec<ResourceCost>,
        pub obsidian_robot_cost: Vec<ResourceCost>,
    }

    impl Blueprint {
        pub fn geodes_opened<const TIME: u32>(&self) -> u32 {
            let mut ore_robots = 1;
            let mut clay_robots = 0;
            let mut geode_robots = 0;
            let mut obsidian_robots = 0;

            let mut ore = 0;
            let mut clay = 0;
            let mut geode = 0;
            let mut obsidian = 0;

            for _ in 0..TIME {
                let new_robot = {
                    if self
                        .geode_robot_cost
                        .iter()
                        .all(|cost| match cost.resource {
                            Resource::Ore => cost.cost <= ore,
                            Resource::Clay => cost.cost <= clay,
                            Resource::Geode => cost.cost <= geode,
                            Resource::Obsidian => cost.cost <= obsidian,
                        })
                    {
                        Some(Resource::Geode)
                    } else if self
                        .obsidian_robot_cost
                        .iter()
                        .all(|cost| match cost.resource {
                            Resource::Ore => cost.cost <= ore,
                            Resource::Clay => cost.cost <= clay,
                            Resource::Geode => cost.cost <= geode,
                            Resource::Obsidian => cost.cost <= obsidian,
                        })
                    {
                        Some(Resource::Obsidian)
                    } else if self.clay_robot_cost.iter().all(|cost| match cost.resource {
                        Resource::Ore => cost.cost <= ore,
                        Resource::Clay => cost.cost <= clay,
                        Resource::Geode => cost.cost <= geode,
                        Resource::Obsidian => cost.cost <= obsidian,
                    }) {
                        Some(Resource::Clay)
                    } else if self.ore_robot_cost.iter().all(|cost| match cost.resource {
                        Resource::Ore => cost.cost <= ore,
                        Resource::Clay => cost.cost <= clay,
                        Resource::Geode => cost.cost <= geode,
                        Resource::Obsidian => cost.cost <= obsidian,
                    }) {
                        Some(Resource::Ore)
                    } else {
                        None
                    }
                };

                if let Some(new_robot) = new_robot {
                    match new_robot {
                        Resource::Ore => todo!(),
                        Resource::Clay => todo!(),
                        Resource::Geode => todo!(),
                        Resource::Obsidian => todo!(),
                    }
                }

                ore += ore_robots;
                clay += clay_robots;
                geode += geode_robots;
                obsidian += obsidian_robots;

                #[cfg(test)]
                println!("Resources: {ore}, {clay}, {obsidian}, {geode}");

                if new_robot.is_none() {
                    continue;
                }

                let cost = {
                    match new_robot.unwrap() {
                        Resource::Ore => {
                            ore_robots += 1;
                            self.ore_robot_cost.as_slice()
                        }
                        Resource::Clay => {
                            clay_robots += 1;
                            self.clay_robot_cost.as_slice()
                        }
                        Resource::Geode => {
                            geode_robots += 1;
                            self.geode_robot_cost.as_slice()
                        }
                        Resource::Obsidian => {
                            obsidian_robots += 1;
                            self.obsidian_robot_cost.as_slice()
                        }
                    }
                };

                for ele in cost {
                    match ele.resource {
                        Resource::Ore => ore -= ele.cost,
                        Resource::Clay => clay -= ele.cost,
                        Resource::Geode => geode -= ele.cost,
                        Resource::Obsidian => obsidian -= ele.cost,
                    }
                }
            }

            geode
        }

        fn parse(input: &str) -> IResult<&str, Self> {
            delimited(
                delimited(tag("Blueprint "), nom::character::complete::u32, tag(":")),
                map(
                    tuple((
                        delimited(
                            pair(multispace1, tag("Each ore robot costs ")),
                            separated_list1(tag(" and "), ResourceCost::parse),
                            char('.'),
                        ),
                        delimited(
                            pair(multispace1, tag("Each clay robot costs ")),
                            separated_list1(tag(" and "), ResourceCost::parse),
                            char('.'),
                        ),
                        delimited(
                            pair(multispace1, tag("Each obsidian robot costs ")),
                            separated_list1(tag(" and "), ResourceCost::parse),
                            char('.'),
                        ),
                        delimited(
                            pair(multispace1, tag("Each geode robot costs ")),
                            separated_list1(tag(" and "), ResourceCost::parse),
                            char('.'),
                        ),
                    )),
                    |(ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost)| {
                        Blueprint {
                            ore_robot_cost,
                            clay_robot_cost,
                            geode_robot_cost,
                            obsidian_robot_cost,
                        }
                    },
                ),
                line_ending,
            )(input)
        }
    }

    impl Resource {
        fn parse(input: &str) -> IResult<&str, Self> {
            branch::alt((
                value(Resource::Ore, tag("ore")),
                value(Resource::Clay, tag("clay")),
                value(Resource::Geode, tag("geode")),
                value(Resource::Obsidian, tag("obsidian")),
            ))(input)
        }
    }

    impl ResourceCost {
        fn parse(input: &str) -> IResult<&str, Self> {
            nom::combinator::map(
                separated_pair(nom::character::complete::u32, multispace1, Resource::parse),
                |(cost, resource)| ResourceCost { cost, resource },
            )(input)
        }
    }

    pub fn input(input: &str) -> IResult<&str, Vec<Blueprint>> {
        many1(terminated(Blueprint::parse, multispace0))(input)
    }
}

#[cfg(test)]
mod test {
    use crate::{parse, quality_level};
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
        let blueprints = parse::input(INPUT)?.1;

        assert_eq!(blueprints[0].geodes_opened::<24>(), 9);
        assert_eq!(blueprints[1].geodes_opened::<24>(), 12);
        assert_eq!(quality_level::<24>(&blueprints), 33);

        Ok(())
    }

    #[test]
    #[ignore]
    fn problem_2() -> Result<()> {
        let _ = parse::input(INPUT)?.1;

        Ok(())
    }
}
