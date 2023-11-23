use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
    vec,
};

fn open_file(filename: &str) -> Result<BufReader<File>, Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

fn main() -> Result<(), Box<dyn Error>> {
    let reader = open_file("input.txt")?;
    let mut elf_crates: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<Vec<usize>> = Vec::new();
    let mut parse_crates = true;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            parse_crates = false;
            continue;
        }
        if parse_crates {
            let parsed = parse_line(line.chars().collect());
            println!("Parsed: {:?}", parsed);
            if parsed[0] == '1' {
                continue;
            }
            for i in 0..parsed.len() {
                let crate_char = parsed[i];
                let idx = elf_crates.get(i);
                if idx.is_none() {
                    if crate_char.is_alphabetic() {
                        elf_crates.push(vec![crate_char]);
                    } else {
                        elf_crates.push(vec![]);
                    }
                    continue;
                }
                if crate_char.is_alphabetic() {
                    elf_crates[i].insert(0, crate_char);
                }
            }
            continue;
        }
        let instruction: Vec<&str> = line.split(' ').collect();
        instructions.push(vec![
            instruction[1].parse::<usize>()?,
            instruction[3].parse::<usize>()?,
            instruction[5].parse::<usize>()?,
        ]);
    }

    println!("Crates: {:?}", elf_crates);
    println!("Instructions: {:?}", instructions);

    for instruction in instructions {
        let move_num = instruction[0];
        let from = instruction[1] - 1;
        let to = instruction[2] - 1;
        let start = elf_crates[from].len() - move_num;
        println!("{}, {}, {}", move_num, start, elf_crates[from].len());
        let elems: Vec<char> = elf_crates[from].drain(start..).collect();
        for elem in elems {
            elf_crates[to].push(elem);
        }
    }
    println!("Elf Crates: {:?}", &elf_crates);
    for crates in elf_crates.iter_mut() {
        print!("{}", crates.pop().unwrap());
    }
    println!();

    Ok(())
}

fn parse_line(line: Vec<char>) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    let mut offset = 1;
    let len = line.len();
    loop {
        if len <= offset {
            break;
        }
        result.push(line[offset]);
        offset += 4;
    }
    result
}
