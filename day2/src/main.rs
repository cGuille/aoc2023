use std::str::FromStr;

use aocutil::timed;

use crate::game::Game;

mod game;

fn main() {
    let input_path = std::env::args()
        .nth(1)
        .unwrap_or(String::from("/dev/stdin"));

    let input = std::fs::read_to_string(input_path).expect("Could not read input");

    let (res, timing) = timed(&input, part1);
    println!("Part 1: {res} ({timing:?})");

    let (res, timing) = timed(&input, part2);
    println!("Part 2: {res} ({timing:?})");
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(Game::from_str)
        .map(Result::unwrap)
        .filter(|game| game.is_possible_with(12, 13, 14))
        .map(|game| game.id())
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(Game::from_str)
        .map(Result::unwrap)
        .map(|game| game.minimal_set())
        .map(|min_set| min_set.power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn part1_with_sample() {
        assert_eq!(8, part1(SAMPLE));
    }

    #[test]
    fn part2_with_sample() {
        assert_eq!(2286, part2(SAMPLE));
    }
}
