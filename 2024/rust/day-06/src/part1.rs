use std::collections::{HashMap, HashSet};

use glam::IVec2;
use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::LocatedSpan;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, ((mut player_position, _), walls)) =
        parse(Span::new(input))
            .map_err(|e| miette!("parse failed {}", e))?;

    let x_minmax = walls
        .iter()
        .map(|(position, _)| position.x)
        .minmax()
        .into_option()
        .unwrap();

    let y_minmax = walls
        .iter()
        .map(|(position, _)| position.y)
        .minmax()
        .into_option()
        .unwrap();

    let mut direction = Direction::North;

    let mut visited_positions: HashSet<IVec2> =
        HashSet::from([player_position]);

    while (x_minmax.0..=x_minmax.1)
        .contains(&player_position.x)
        && (y_minmax.0..=y_minmax.1)
            .contains(&player_position.y)
    {
        let next_position =
            player_position + direction.to_ivec2();
        if walls.get(&next_position).is_some() {
            direction = direction.turn_right();
        } else {
            player_position = next_position;
            visited_positions.insert(player_position);
        }
    }
    // dbg!(&visited_positions);

    Ok((visited_positions.len() - 1).to_string())
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
    fn to_ivec2(&self) -> IVec2 {
        match self {
            Direction::North => IVec2::NEG_Y,
            Direction::South => IVec2::Y,
            Direction::East => IVec2::X,
            Direction::West => IVec2::NEG_X,
        }
    }
}

pub type Span<'a> = LocatedSpan<&'a str>;

fn token(input: Span) -> IResult<Span, (IVec2, char)> {
    let y = input.location_line();
    let x = input.get_column();
    let (input, token) = one_of(".#^")(input)?;

    Ok((
        input,
        (
            IVec2::new(x as i32 - 1, y as i32 - 1),
            token,
        ),
    ))
}
pub fn parse(
    input: Span,
) -> IResult<Span, ((IVec2, char), HashMap<IVec2, char>)> {
    let (input, items) =
        separated_list1(line_ending, many1(token))(input)?;

    let player = items
        .iter()
        .flatten()
        .find(|(_, value)| value == &'^')
        .cloned()
        .expect("should have a player");
    let walls = items
        .into_iter()
        .flatten()
        .filter(|(_, value)| value == &'#')
        .collect::<HashMap<IVec2, char>>();
    Ok((input, (player, walls)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
