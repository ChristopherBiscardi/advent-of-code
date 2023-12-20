// winnow, ported from nom according to
// a the winnow migration guide
// port to 0.3, upgrade to 0.4, then 0.5
use winnow::{
    ascii::{dec_uint, digit1, line_ending, space1},
    combinator::{
        alt, delimited, fold_repeat, opt, separated,
        separated_pair, terminated,
    },
    token::tag,
    PResult, Parser,
};

use crate::game::*;

fn parse_color(input: &mut &[u8]) -> PResult<Color> {
    alt((
        tag("red").map(|_| Color::Red),
        tag("green").map(|_| Color::Green),
        tag("blue").map(|_| Color::Blue),
    ))
    .parse_next(input)
}
fn cube(input: &mut &[u8]) -> PResult<(u32, Color)> {
    separated_pair(dec_uint, space1, parse_color)
        .parse_next(input)
}
fn round(input: &mut &[u8]) -> PResult<Round> {
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
pub fn game<'i>(input: &mut &'i [u8]) -> PResult<Game<'i>> {
    let id = delimited(tag("Game "), digit1, tag(": "))
        .parse_next(input)?;
    let rounds = separated(0.., round, tag("; "))
        .parse_next(input)?;
    Ok(Game {
        id: std::str::from_utf8(id).unwrap(),
        rounds,
    })
}

pub fn parse<'i>(
    input: &mut &'i [u8],
) -> PResult<Vec<Game<'i>>> {
    separated(0.., game, line_ending).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::game_output;

    #[test]
    fn test_parse() {
        let game =
            parse(&mut game_output::INPUT.as_bytes())
                .unwrap();
        assert_eq!(game_output::output(), &game);
    }
}
