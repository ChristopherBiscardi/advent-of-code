// nom_intro, rewritten using nom_supreme
use nom::{
    branch::alt,
    character::complete::{
        self, digit1, line_ending, space1,
    },
    combinator::opt,
    multi::{fold_many1, separated_list1},
    IResult, Parser,
};
use nom_supreme::{
    error::ErrorTree, final_parser::final_parser,
    tag::complete::tag, ParserExt,
};

use crate::game::*;

fn parse_color(
    input: &str,
) -> IResult<&str, Color, ErrorTree<&str>> {
    alt((
        tag("red").map(|_| Color::Red),
        tag("green").map(|_| Color::Green),
        tag("blue").map(|_| Color::Blue),
    ))(input)
}
fn cube(
    input: &str,
) -> IResult<&str, (u32, Color), ErrorTree<&str>> {
    complete::u32
        .terminated(space1)
        .and(parse_color)
        .parse(input)
}
fn round(
    input: &str,
) -> IResult<&str, Round, ErrorTree<&str>> {
    fold_many1(
        cube.terminated(opt(tag(", "))),
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
    .parse(input)
}
pub fn game(
    input: &str,
) -> IResult<&str, Game, ErrorTree<&str>> {
    digit1
        .preceded_by(tag("Game "))
        .terminated(tag(": "))
        .and(separated_list1(tag("; "), round))
        .parse(input)
        .map(|(input, (id, rounds))| {
            (input, Game { id, rounds })
        })
}

pub fn parse(
    input: &str,
) -> Result<Vec<Game>, ErrorTree<&str>> {
    final_parser(separated_list1(line_ending, game))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::game_output;

    #[test]
    fn test_parse() {
        let game = parse(&game_output::INPUT).unwrap();
        assert_eq!(game_output::output(), &game);
    }
}
