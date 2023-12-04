use std::{collections::HashSet, str::FromStr};

fn main() {
    let input_path = std::env::args()
        .nth(1)
        .unwrap_or(String::from("/dev/stdin"));

    let input = std::fs::read_to_string(input_path).expect("Could not read input");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(Card::from_str)
        .map(Result::unwrap)
        .map(|card| card.points())
        .sum()
}

fn part2(_input: &str) -> u64 {
    todo!()
}

#[derive(Debug)]
struct Card {
    #[allow(unused)]
    id: u64,
    winning_nums: HashSet<u64>,
    draw: HashSet<u64>,
}

impl Card {
    fn points(&self) -> u64 {
        let winning_num_count = self.winning_nums.intersection(&self.draw).count();

        if winning_num_count < 2 {
            u64::try_from(winning_num_count).unwrap()
        } else {
            2u64.pow(u32::try_from(winning_num_count - 1).unwrap())
        }
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (remaining, card) = parser::parse_card(s).map_err(|err| err.to_string())?;

        if !remaining.is_empty() {
            return Err(format!("Unexpected remaining input: {remaining}"));
        }

        Ok(card)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn part1_with_sample() {
        assert_eq!(13, part1(SAMPLE));
    }

    #[test]
    #[ignore = "until part 2 is implemented"]
    fn part2_with_sample() {
        assert_eq!(0, part2(SAMPLE));
    }
}

mod parser {
    use std::collections::HashSet;

    use nom::{
        bytes::complete::tag,
        character::complete::{digit1, multispace0, multispace1},
        multi::separated_list1,
        sequence::{delimited, separated_pair, terminated},
        IResult,
    };

    use super::Card;

    pub(crate) fn parse_card(input: &str) -> IResult<&str, Card> {
        let (remaining, (id, (winning_nums, draw))) = separated_pair(
            parse_card_id,
            multispace0,
            separated_pair(
                parse_u64_set,
                delimited(multispace0, tag("|"), multispace0),
                parse_u64_set,
            ),
        )(input)?;

        Ok((
            remaining,
            Card {
                id,
                winning_nums,
                draw,
            },
        ))
    }

    fn parse_card_id(input: &str) -> IResult<&str, u64> {
        delimited(terminated(tag("Card"), multispace1), parse_u64, tag(":"))(input)
    }

    fn parse_u64_set(input: &str) -> IResult<&str, HashSet<u64>> {
        separated_list1(multispace1, parse_u64)(input)
            .map(|(remaining, list)| (remaining, list.into_iter().collect()))
    }

    fn parse_u64(input: &str) -> IResult<&str, u64> {
        let (remaining, digits) = digit1(input)?;

        let num = u64::from_str_radix(digits, 10).unwrap();

        Ok((remaining, num))
    }

    #[cfg(test)]
    mod tests {
        use super::parse_card;

        #[test]
        fn test_parse_card() {
            let (remaining, card) =
                parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap();

            assert!(remaining.is_empty(), "the input is entirely consumed");

            assert_eq!(1, card.id, "the card ID is ok");

            assert_eq!(
                5,
                card.winning_nums.len(),
                "we have the right amount of winning nums"
            );

            assert_eq!(
                8,
                card.draw.len(),
                "we have the right amount of winning nums"
            );
        }
    }
}
