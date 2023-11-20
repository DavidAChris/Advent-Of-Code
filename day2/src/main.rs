use std::{
    error::Error,
    fs::OpenOptions,
    io::{BufRead, BufReader},
};

struct Choices;

impl Choices {
    const LOSE: i32 = 0;
    const DRAW: i32 = 3;
    const WIN: i32 = 6;
    const ROCK: i32 = 1;
    const PAPER: i32 = 2;
    const SCISSOR: i32 = 3;

    fn score(choice: &str) -> i32 {
        match choice {
            "AY" => Choices::DRAW + Choices::ROCK,
            "AX" => Choices::LOSE + Choices::SCISSOR,
            "AZ" => Choices::WIN + Choices::PAPER,
            "BY" => Choices::DRAW + Choices::PAPER,
            "BX" => Choices::LOSE + Choices::ROCK,
            "BZ" => Choices::WIN + Choices::SCISSOR,
            "CY" => Choices::DRAW + Choices::SCISSOR,
            "CX" => Choices::LOSE + Choices::PAPER,
            "CZ" => Choices::WIN + Choices::ROCK,
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
