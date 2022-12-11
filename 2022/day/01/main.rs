use color_eyre::Result;

static INPUT: &str = include_str!("input");

fn main() -> Result<()> {
    color_eyre::install()?;
    
    let mut elf_bags: Vec<Vec<u32>> = vec![vec![]];

    for line in INPUT.lines() {
        if line.is_empty() {
            elf_bags.push(vec![])
        } else {
            let last = elf_bags.last_mut().unwrap();
            last.push(line.parse()?);
        }
    }

    let elf_calories: Vec<_> = elf_bags
        .into_iter()
        .map(|bag| bag.iter().sum::<u32>())
        .collect();

    println!("Problem 1: {}", elf_calories.iter().max().unwrap());

    let mut elf_calories = elf_calories;
    elf_calories.sort();
    let top_3 = &elf_calories[elf_calories.len() - 3..];

    println!("Problem 2: {}", top_3.iter().sum::<u32>());

    Ok(())
}
