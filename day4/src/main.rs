use std::{
    error::Error,
    fs::OpenOptions,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open("input.txt")?;
    let reader = BufReader::new(file);
    let mut total_pairs = 0;

    for line in reader.lines() {
        let line = line?;
        let values: Vec<&str> = line.split(',').collect();
        let first: Vec<i32> = values[0]
            .split('-')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let second: Vec<i32> = values[1]
            .split('-')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        if duplicate_schedule(first, second) {
            total_pairs += 1
        }
    }

    println!("Total Pairs: {}", total_pairs);

    Ok(())
}

fn duplicate_schedule(first: Vec<i32>, second: Vec<i32>) -> bool {
    let first = first[0]..=first[1];
    let second = second[0]..=second[1];
    let mut is_duplicate = false;
    for num in first {
        if second.contains(&num) {
            is_duplicate = true;
            break;
        }
    }
    is_duplicate
}

#[cfg(test)]
mod tests {
    use crate::duplicate_schedule;

    #[test]
    fn test_duplicates() {
        let first = vec![5, 7];
        let second = vec![7, 9];
        assert!(duplicate_schedule(first, second))
    }
}
