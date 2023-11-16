use std::{
    error::Error,
    fs::OpenOptions,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open("input.txt")?;
    let reader = BufReader::new(file);
    let mut current_elf: Vec<usize> = Vec::new();
    let mut results: Vec<usize> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            let sum: usize = current_elf.iter().sum();
            results.push(sum);
            current_elf.clear(); // Once we get all values from one elf
            continue;
        }
        let number = line.parse::<usize>()?;
        current_elf.push(number);
    }

    let mut sum: usize = 0;
    for _ in 0..3 {
        let max = results.iter().max().unwrap().to_owned();
        let idx = results.iter().position(|num| num == &max).unwrap();
        sum += max;
        results.remove(idx);
    }
    println!("Total Calories: {}", sum);

    Ok(())
}
