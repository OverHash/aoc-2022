use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_character_priority(c: char) -> u32 {
    let score = if c <= 'Z' {
        // uppercase chars have normalized priority 27-52
        c as u32 - 'A' as u32 + 27
    } else {
        // lowercase chars have normalized priority 1-26
        c as u32 - 'a' as u32 + 1
    };

    score
}

fn main() {
    let file = File::open("input.txt").expect("Failed to open input file");

    let lines: Vec<_> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .collect();

    let score: u32 = lines
        .iter()
        .map(|l| {
            // split by compartment
            let (left, right) = l.split_at(l.len() / 2);

            let shared_items = [left, right]
                .into_iter()
                .map(|compartment| compartment.chars().collect::<HashSet<_>>())
                // perform set intersection
                .reduce(|a, b| &a & &b)
                .expect("left and right was empty");

            let priority: u32 = shared_items
                .into_iter()
                .map(calculate_character_priority)
                .sum();

            priority
        })
        .sum();

    println!("score was {score}");

    // solve part 2
    let badge_sum: u32 = lines
        // split lines into groups of 3
        .chunks(3)
        .map(|elf_group| {
            elf_group
                .iter()
                // convert line into a HashSet of chars
                .map(|rucksack| rucksack.chars().collect::<HashSet<_>>())
                // find intersection of elf group
                .reduce(|a, b| &a & &b)
                .expect("elf group was empty")
                .into_iter()
                // map the badge to character priority
                .map(calculate_character_priority)
                .next()
                .expect("elf group was empty")
        })
        .sum();
    println!("badge sum was {badge_sum}");
}
