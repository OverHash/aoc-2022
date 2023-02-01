use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").expect("Failed to open input file");

    let score: u32 = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .map(|l| {
            // split by compartment
            let (left, right) = l.split_at(l.len() / 2);

            let similar_items = [left, right]
                .into_iter()
                .map(|compartment| compartment.chars().collect::<HashSet<_>>())
                .reduce(|a, b| &a & &b)
                .expect("left and right was empty");

            let priority: u32 = similar_items
                .into_iter()
                .map(|c| {
                    let score = if c <= 'Z' {
                        // uppercase chars have normalized priority 27-52
                        c as u32 - 'A' as u32 + 27
                    } else {
                        // lowercase chars have normalized priority 1-26
                        c as u32 - 'a' as u32 + 1
                    };

                    println!("score={score}");

                    score
                })
                .sum();

            priority
        })
        .sum();

    println!("score was {score}");
}
