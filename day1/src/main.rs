use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").expect("Failed to open input file");

    let mut elfs: Vec<_> = Vec::new();

    for line in BufReader::new(file)
        .lines()
        .map(|l| l.expect("Failed to read line"))
    {
        if line.is_empty() {
            // we finished reading this elf
            elfs.push(0);
            continue;
        }

        let line_val = line.parse::<u32>().expect("Failed to parse line");

        if let Some(last) = elfs.last_mut() {
            *last += line_val
        } else {
            // handle the very first elf value
            elfs.push(line_val);
        }
    }

    elfs.sort_unstable();

    println!("max is {}", elfs[elfs.len() - 1]);

    // part two:
    let last_three_elfs_summed = elfs
        .into_iter()
        .rev()
        .take(3)
        .reduce(|a, b| a + b)
        .expect("No elfs input data was provided");
    println!("last three elfs summed is {last_three_elfs_summed}");
}
