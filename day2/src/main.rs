use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").expect("Failed to open input file");

    let lines: Vec<_> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .map(|l| {
            let mut chars = l.chars();
            let opponent =
                chars.next().expect("Failed to read opponent choice") as u32 - 'A' as u32;
            chars.next(); // skip " "
            let you = chars.next().expect("Failed to read your choice") as u32 - 'X' as u32;

            (opponent, you)
        })
        .collect();

    let score: u32 = lines
        .iter()
        .map(|(opponent, you)| {
            // if you are 1 above the opponent (wrapped), then we get 6 points
            let outcome_score = (you == &((opponent + 1) % 3))
                .then(|| 6)
                .unwrap_or_else(|| {
                    // check for draw, 3 points
                    if you == opponent {
                        return 3;
                    }

                    // 0 points for a loss
                    0
                });

            // score = selection_score + outcome_score
            return (you + 1) + outcome_score;
        })
        .sum();

    println!("score is {score}");

    // part 2:
    let score: u32 = lines
        .iter()
        .map(|(opponent, you)| {
            let outcome_score = 3 * you;

            let selection_score = match you {
                0 => {
                    // we need to lose
                    // we add two as we don't want to do (opponent - 1) % 3 to
                    // avoid wrapping issues, so we can use congruency of
                    // modularity to instead do (opponent + 3 - 1) % 3 <=>
                    // (opponent + 2) % 3
                    (opponent + 2) % 3
                }
                1 => {
                    // we need to draw
                    *opponent
                }
                2 => {
                    // we need to win
                    (opponent + 1) % 3
                }
                _ => unreachable!("user choice was not X, Y or Z"),
            };

            outcome_score + selection_score + 1
        })
        .sum();

    println!("result for part 2 is {score}")
}
