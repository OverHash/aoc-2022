use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").expect("Failed to open input file");

    let score: u32 = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .map(|l| {
            let mut chars = l.chars();
            let opponent =
                chars.next().expect("Failed to read opponent choice") as u32 - 'A' as u32;
            chars.next(); // skip " "
            let you = chars.next().expect("Failed to read your choice") as u32 - 'X' as u32;

            let winner = (you == ((opponent + 1) % 3)).then(|| 6).unwrap_or_else(|| {
                // check for draw
                if you == opponent {
                    return 3;
                }

                // 0 points for a loss
                0
            });

            println!(
                "you chose {you} + winner score was {winner} + 1 = {}",
                you + winner + 1,
            );

            return you + winner + 1;
        })
        .sum();

    println!("score is {score}");
}
