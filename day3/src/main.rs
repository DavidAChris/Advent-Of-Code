use std::{
    error::Error,
    fs::OpenOptions,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open("input.txt")?;
    let reader = BufReader::new(file);
    let mut total = 0;

    for line in reader.lines() {
        let line = line?;
        let len = line.len() / 2;
        let (first, second) = (&line[..len], &line[len..]);
        total += priority(first, second);
    }

    println!("Total: {}", total);
    Ok(())
}

fn priority(first: &str, second: &str) -> i32 {
    let lower_offset = 96;
    let upper_offset = 38;
    let mut priority = 0;
    for character in first.chars() {
        if second.contains(character) {
            let is_lower = character.is_lowercase();
            if is_lower {
                priority = character as i32 - lower_offset;
            } else {
                priority = character as i32 - upper_offset;
            }
        }
    }
    priority
}
