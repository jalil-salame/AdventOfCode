fn main() -> Result<(), Box<dyn Error>> {
    for line in BufReader::new(File::open("input")?).lines() {
        println!("{line:?}");
    }

    let p1_solution = "Nothing Yet";
    println!("Problem 1: {}", p1_solution);

    let p2_solution = "Nothing Yet";
    println!("Problem 2: {}", p2_solution);

    Ok(())
}
