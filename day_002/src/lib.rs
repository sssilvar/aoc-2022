//! ## \--- Day 2: Rock Paper Scissors ---
//!
//! The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant [Rock Paper Scissors](https://en.wikipedia.org/wiki/Rock_paper_scissors) tournament is already in progress.
//!
//! Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.
//!
//! Appreciative of your help yesterday, one Elf gives you an _encrypted strategy guide_ (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: `A` for Rock, `B` for Paper, and `C` for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.
//!
//! The second column, you reason, must be what you should play in response: `X` for Rock, `Y` for Paper, and `Z` for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.
//!
//! The winner of the whole tournament is the player with the highest score. Your _total score_ is the sum of your scores for each round. The score for a single round is the score for the _shape you selected_ (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the _outcome of the round_ (0 if you lost, 3 if the round was a draw, and 6 if you won).
//!
//! Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.
//!
//! For example, suppose you were given the following strategy guide:
//!
//! ```md
//! A Y
//! B X
//! C Z
//! ```
//!
//! This strategy guide predicts and recommends the following:
//!
//! -   In the first round, your opponent will choose Rock (`A`), and you should choose Paper (`Y`). This ends in a win for you with a score of _8_ (2 because you chose Paper + 6 because you won).
//! -   In the second round, your opponent will choose Paper (`B`), and you should choose Rock (`X`). This ends in a loss for you with a score of _1_ (1 + 0).
//! -   The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = _6_.
//!
//! In this example, if you were to follow the strategy guide, you would get a total score of `_15_` (8 + 1 + 6).
//!
//! _What would your total score be if everything goes exactly according to your strategy guide?_

const INPUT: &str = include_str!("../data/input.txt");

/// Part 1 Challenge:
///
/// You are given a list of rounds in a Rock-Paper-Scissors game. Each round has two columns:
///
/// First column: Your opponent’s move (A for Rock, B for Paper, C for Scissors).
/// Second column: Your move (X for Rock, Y for Paper, Z for Scissors).
/// The scoring system is:
///
/// 1 point for choosing Rock, 2 for Paper, 3 for Scissors.
/// 0 points if you lose, 3 points if it's a draw, and 6 points if you win.
/// Task: Calculate your total score for all rounds.
fn part_1() -> u32 {
    let compute_score = |line: &str| -> u32 {
        let opponent = line.chars().nth(0).unwrap();
        let you = line.chars().nth(2).unwrap();

        let opponent_score = match opponent {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => 0,
        };

        let you_score = match you {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => 0,
        };

        let score = match (opponent_score, you_score) {
            (1, 2) | (2, 3) | (3, 1) => 6,
            (1, 1) | (2, 2) | (3, 3) => 3,
            _ => 0,
        };

        you_score + score
    };

    let res = INPUT
        .lines()
        .map(|line| compute_score(line))
        .collect::<Vec<u32>>();

    // Compute the total scores for each round
    let total_score = res.iter().sum::<u32>();
    total_score
}

/// Part 2
/// --- Part Two ---
/// The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"
/// The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:
///
/// In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
/// In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
/// In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
/// Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.
///
/// What would your total score be if everything goes exactly according to your strategy guide?
fn part_2() -> u32 {
    let new_compute_score = |line: &str| -> u32 {
        let opponent = line.chars().nth(0).unwrap(); // 'A' for Rock, 'B' for Paper, 'C' for Scissors
        let outcome = line.chars().nth(2).unwrap(); // 'X' for lose, 'Y' for draw, 'Z' for win

        // Map opponent's choice to 1 (Rock), 2 (Paper), 3 (Scissors)
        let opponent_score = match opponent {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => 0,
        };

        // Based on outcome ('X' for lose, 'Y' for draw, 'Z' for win), determine your choice
        let you_score = match (opponent_score, outcome) {
            // If you need to lose ('X'), choose the losing move
            (1, 'X') => 3, // Opponent: Rock (1), You: Scissors (3)
            (2, 'X') => 1, // Opponent: Paper (2), You: Rock (1)
            (3, 'X') => 2, // Opponent: Scissors (3), You: Paper (2)

            // If you need to draw ('Y'), choose the same move
            (1, 'Y') => 1, // Opponent: Rock (1), You: Rock (1)
            (2, 'Y') => 2, // Opponent: Paper (2), You: Paper (2)
            (3, 'Y') => 3, // Opponent: Scissors (3), You: Scissors (3)

            // If you need to win ('Z'), choose the winning move
            (1, 'Z') => 2, // Opponent: Rock (1), You: Paper (2)
            (2, 'Z') => 3, // Opponent: Paper (2), You: Scissors (3)
            (3, 'Z') => 1, // Opponent: Scissors (3), You: Rock (1)

            _ => 0,
        };

        // Outcome score: 0 if you lose, 3 if draw, 6 if win
        let outcome_score = match outcome {
            'X' => 0, // Lose
            'Y' => 3, // Draw
            'Z' => 6, // Win
            _ => 0,
        };

        you_score + outcome_score
    };

    let res = INPUT
        .lines()
        .map(|line| new_compute_score(line))
        .collect::<Vec<u32>>();

    // Total score if everything goes exactly according to your strategy guide
    res.iter().sum::<u32>()
}

pub fn challenge_day_002() -> String {
    // Part 1: Calculate the total score for all rounds
    print!(
        "Total score if everything goes exactly according to your strategy guide: {}\n",
        part_1()
    );

    // Part 2: Calculate the total score if everything goes exactly according to your strategy guide
    print!(
        "Total score if everything goes exactly according to your strategy guide: {}\n",
        part_2()
    );

    // Return a message
    String::from("Done!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(part_1(), 10718);
    }

    #[test]
    fn part_2_it_works() {
        assert_eq!(part_2(), 14652);
    }
}
