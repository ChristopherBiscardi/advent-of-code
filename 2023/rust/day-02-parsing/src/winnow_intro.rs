// winnow, ported from nom according to
// a the winnow migration guide
// port to 0.3, upgrade to 0.4, then 0.5
use winnow::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, digit1, line_ending, space1,
    },
    combinator::opt,
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

use crate::game::*;

fn parse_color(input: &str) -> IResult<&str, Color> {
    alt((
        tag("red").map(|_| Color::Red),
        tag("green").map(|_| Color::Green),
        tag("blue").map(|_| Color::Blue),
    ))(input)
}
fn cube(input: &str) -> IResult<&str, (u32, Color)> {
    separated_pair(complete::u32, space1, parse_color)(
        input,
    )
}
fn round(input: &str) -> IResult<&str, Round> {
    fold_many1(
        terminated(cube, opt(tag(", "))),
        Round::default,
        |mut round, (count, color)| {
            match color {
                Color::Red => {
                    round.red = count;
                }
                Color::Green => {
                    round.green = count;
                }
                Color::Blue => {
                    round.blue = count;
                }
            }
            round
        },
    )(input)
}
pub fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) =
        delimited(tag("Game "), digit1, tag(": "))(input)?;
    let (input, rounds) =
        separated_list1(tag("; "), round)(input)?;
    Ok((input, Game { id, rounds }))
}

pub fn parse(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(line_ending, game)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::game_output;

    #[test]
    fn test_parse() {
        let (input, game) =
            parse(&game_output::INPUT).unwrap();
        assert_eq!(input, "");
        assert_eq!(game_output::output(), &game);
    }
}
