use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::game::*;

#[derive(Parser)]
#[grammar = "games.pest"]
pub struct GamesParser;

pub fn parse(
    input: &str,
) -> Result<Pairs<'_, Rule>, Error<Rule>> {
    GamesParser::parse(Rule::games, input)
}

impl<'a> TryFrom<Pair<'a, Rule>> for Game<'a> {
    type Error = String;

    fn try_from(
        value: Pair<'a, Rule>,
    ) -> Result<Self, Self::Error> {
        if value.as_rule() != Rule::game_line {
            return Err(format!(
                "Pair is not a `game`. it is a `{:?}`",
                value.as_rule()
            ));
        };

        // there can only be one game per line, but
        // pest doesn't know that
        let game_pair = value.into_inner().next().unwrap();

        // bootstrap an "empty" game to fill into

        let final_game = game_pair.into_inner().fold(
            Game {
                id: "",
                rounds: vec![],
            },
            |mut game, pair| {
                match pair.as_rule() {
                    Rule::game_id => {
                        game.id = pair.as_span().as_str();
                    }
                    Rule::round => {
                        game.rounds.push(
                            pair.into_inner().fold(
                                Round::default(),
                                |mut round, cube| {
                                    let mut cube_it = cube
                                        .into_inner()
                                        .map(|p| {
                                            p.as_span()
                                                .as_str()
                                        });
                                    let count = cube_it
                                        .next()
                                        .and_then(|c| {
                                            c.parse::<u32>()
                                                .ok()
                                        })
                                        .unwrap();
                                    match cube_it
                                        .next()
                                        .unwrap()
                                    {
                                        "red" => {
                                            round.red =
                                                count;
                                        }
                                        "green" => {
                                            round.green =
                                                count;
                                        }
                                        "blue" => {
                                            round.blue =
                                                count;
                                        }
                                        _ => {unreachable!("no other colors");}
                                    }
                                    round
                                },
                            ),
                        );
                    }
                    _ => {}
                }
                game
            },
        );

        Ok(final_game)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::game_output;

    #[test]
    fn test_from_rule() {
        let mut games = parse(game_output::INPUT).unwrap();
        let first_game = games
            .next()
            .unwrap()
            .into_inner()
            .next()
            .unwrap();
        let a_game = Game::try_from(first_game).unwrap();
        assert_eq!(game_output::output()[0], a_game);
    }

    #[test]
    fn test_parse() {
        let input = game_output::INPUT;

        let games = parse(input)
            // parse is a Result<Pairs, _>
            .unwrap()
            .into_iter()
            // the parser for all games always comes in as a vec
            // even though there's only one "games" parser
            .next()
            .unwrap()
            // iterate over each game_line, which is the
            // "inner" parser of the "games" parser
            .into_inner()
            .map(|pair| Game::try_from(pair).unwrap())
            .collect::<Vec<Game>>();
        assert_eq!(game_output::output(), &games);
        // panic!("")
    }
}
