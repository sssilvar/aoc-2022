//! ## --- Day 1: Calorie Counting ---
//!
//! Santa's reindeer typically eat regular reindeer food, but they need a lot of [magical energy](https://adventofcode.com/2018/day/25) to deliver presents on Christmas. For that, their favorite snack is a special type of _star_ fruit that only grows deep in the jungle. The Elves have brought you on their annual expedition to the grove where the fruit grows.
//!
//! To supply enough magical energy, the expedition needs to retrieve a minimum of _fifty stars_ by December 25th. Although the Elves assure you that the grove has plenty of fruit, you decide to grab any fruit you see along the way, just in case.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants _one star_. Good luck!
//!
//! The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin taking inventory of their supplies. One important consideration is food - in particular, the number of _Calories_ each Elf is carrying (your puzzle input).
//!
//! The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.
//!
//! For example, suppose the Elves finish writing their items' Calories and end up with the following list:
//!
//! ```md
//! 1000
//! 2000
//! 3000
//!
//! 4000
//!
//! 5000
//! 6000
//!
//! 7000
//! 8000
//! 9000
//!
//! 10000
//! ```
//!
//! This list represents the Calories of the food carried by five Elves:
//!
//! -   The first Elf is carrying food with `1000`, `2000`, and `3000` Calories, a total of `_6000_` Calories.
//! -   The second Elf is carrying one food item with `_4000_` Calories.
//! -   The third Elf is carrying food with `5000` and `6000` Calories, a total of `_11000_` Calories.
//! -   The fourth Elf is carrying food with `7000`, `8000`, and `9000` Calories, a total of `_24000_` Calories.
//! -   The fifth Elf is carrying one food item with `_10000_` Calories.
//!
//! In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the _most_ Calories. In the example above, this is _`24000`_ (carried by the fourth Elf).
//!
//! Find the Elf carrying the most Calories. _How many total Calories is that Elf carrying?_

const INPUT: &str = include_str!("../data/input.txt");

/// Get calories per elf
/// This function works by splitting the input by double newlines, then
/// parsing each line and summing the values. The result is a vector of
/// u32 values.
fn get_calories_per_elf() -> Vec<u32> {
    INPUT
        .split("\n\n")
        .map(|stpe| {
            stpe.lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>()
}

/// Part 1: Find how many total Calories the Elf carrying the most Calories is carrying
fn part_1() -> u32 {
    get_calories_per_elf().iter().max().unwrap().to_owned()
}

/// Part 2: Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
fn part_2() -> u32 {
    let mut calories_per_elf = get_calories_per_elf();
    calories_per_elf.sort(); // Sort the vector in ascending order (called separedlt as it receive a self mut reference `&mut self`)
    calories_per_elf.iter().rev().take(3).sum()
}

pub fn challenge_day_001() -> String {
    let part_1 = format!("Part 1: {}", part_1());
    let part_2 = format!("Part 2: {}", part_2());

    part_1 + "\n" + &part_2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_it_works() {
        assert_eq!(part_1(), 67658);
    }
}
