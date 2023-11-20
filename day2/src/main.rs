use std::{
    error::Error,
    fs::OpenOptions,
    io::{BufRead, BufReader},
};

struct Choices;

impl Choices {
    fn score(choice: &str) -> i32 {
        match choice {
            "AY" => 8,
            "AX" => 4,
            "AZ" => 3,
            "BY" => 5,
            "BX" => 1,
            "BZ" => 9,
            "CY" => 2,
            "CX" => 7,
            "CZ" => 6,
            _ => 0,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open("input.txt")?;
    let reader = BufReader::new(file);
    let mut total = 0;

    for line in reader.lines() {
        let line = line?;
        let line = line.replace(' ', "");
        total += Choices::score(&line);
    }

    println!("Total: {}", total);

    Ok(())
}
