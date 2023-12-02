use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

use super::*;

pub(crate) fn parse_game(input: &str) -> IResult<&str, Game> {
    let (remaining, id) = parse_game_id(input)?;
    let (remaining, sets) = parse_sets(remaining)?;

    Ok((remaining, Game { id, sets }))
}

fn parse_game_id(input: &str) -> IResult<&str, u64> {
    delimited(tag("Game "), parse_u64, tag(": "))(input)
}

fn parse_sets(input: &str) -> IResult<&str, Vec<GameSet>> {
    separated_list1(tag("; "), parse_set)(input)
}

fn parse_set(input: &str) -> IResult<&str, GameSet> {
    separated_list0(tag(", "), parse_colour_amount)(input).map(|(remaining, colour_amounts)| {
        (
            remaining,
            colour_amounts
                .into_iter()
                .fold(GameSet::default(), |mut set, (amount, colour)| {
                    match colour {
                        Colour::Red => set.red_count += amount,
                        Colour::Green => set.green_count += amount,
                        Colour::Blue => set.blue_count += amount,
                    };

                    set
                }),
        )
    })
}

fn parse_colour_amount(input: &str) -> IResult<&str, (u64, Colour)> {
    separated_pair(parse_u64, tag(" "), parse_colour)(input)
}

fn parse_colour(input: &str) -> IResult<&str, Colour> {
    let (remaining, colour_str) = alt((tag("red"), tag("green"), tag("blue")))(input)?;

    let colour = match colour_str {
        "red" => Colour::Red,
        "green" => Colour::Green,
        "blue" => Colour::Blue,
        _ => unreachable!(),
    };

    Ok((remaining, colour))
}

enum Colour {
    Red,
    Green,
    Blue,
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    let (remaining, digits) = digit1(input)?;

    let num = u64::from_str_radix(digits, 10).unwrap();

    Ok((remaining, num))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let (remaining, game) = parse_game(input).unwrap();

        assert_eq!("", remaining);
        assert_eq!(1, game.id);
        assert_eq!(3, game.sets.len());

        assert_eq!(3, game.sets[0].blue_count);
        assert_eq!(4, game.sets[0].red_count);
        assert_eq!(0, game.sets[0].green_count);

        assert_eq!(1, game.sets[1].red_count);
        assert_eq!(2, game.sets[1].green_count);
        assert_eq!(6, game.sets[1].blue_count);

        assert_eq!(2, game.sets[2].green_count);
        assert_eq!(0, game.sets[2].red_count);
        assert_eq!(0, game.sets[2].blue_count);
    }
}
