use std::{
    error::Error,
    fs::OpenOptions,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open("input.txt")?;
    let reader = BufReader::new(file);
    let mut total = 0;
    let mut lines = reader.lines();

    loop {
        let (first, second, third) = (lines.next(), lines.next(), lines.next());
        if first.is_none() || second.is_none() || third.is_none() {
            break;
        }
        let (first, second, third) = (first.unwrap()?, second.unwrap()?, third.unwrap()?);
        total += priority(&first, &second, &third);
    }

    println!("Total: {}", total);
    Ok(())
}

fn priority(first: &str, second: &str, third: &str) -> i32 {
    let lower_offset = 96;
    let upper_offset = 38;
    let mut priority = 0;
    for character in first.chars() {
        if second.contains(character) && third.contains(character) {
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
