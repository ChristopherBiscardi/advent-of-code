// winnow, ported from nom according to
// a the winnow migration guide
// port to 0.3, upgrade to 0.4, then 0.5
use winnow::{
    ascii::{dec_uint, digit1, line_ending, space1},
    branch::alt,
    bytes::tag,
    combinator::{fold_repeat, opt},
    multi::separated1,
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

use crate::game::*;

fn parse_color(input: &str) -> IResult<&str, Color> {
    alt((
        tag("red").map(|_| Color::Red),
        tag("green").map(|_| Color::Green),
        tag("blue").map(|_| Color::Blue),
    ))
    .parse_next(input)
}
fn cube(input: &str) -> IResult<&str, (u32, Color)> {
    separated_pair(dec_uint, space1, parse_color)
        .parse_next(input)
}
fn round(input: &str) -> IResult<&str, Round> {
    fold_repeat(
        0..,
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
    )
    .parse_next(input)
}
pub fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) =
        delimited(tag("Game "), digit1, tag(": "))
            .parse_next(input)?;
    let (input, rounds) =
        separated1(round, tag("; ")).parse_next(input)?;
    Ok((input, Game { id, rounds }))
}

pub fn parse(input: &str) -> IResult<&str, Vec<Game>> {
    separated1(game, line_ending).parse_next(input)
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
