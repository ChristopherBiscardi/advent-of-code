use glam::IVec2;
use miette::miette;
use nom::{
    character::complete::{line_ending, satisfy},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::{position, LocatedSpan};
use std::collections::{HashMap, HashSet};

const DIRECTIONS: [IVec2; 4] =
    [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, map) = parse(Span::new(input))
        .map_err(|e| miette!("parse failed {}", e))?;

    let counts: usize = map
        .iter()
        .filter(|(_, height)| height == &&0)
        .map(|(position, _)| search_trail(&map, position))
        .sum();

    Ok(counts.to_string())
}

fn search_trail(
    map: &HashMap<IVec2, u32>,
    position: &IVec2,
) -> usize {
    let mut visited: HashSet<IVec2> = HashSet::from([]);
    let mut new_locations: HashSet<IVec2> =
        HashSet::from([*position]);
    while !new_locations.is_empty() {
        let newer_locations = new_locations
            .iter()
            .flat_map(|starting_location| {
                DIRECTIONS.iter().map(move |dir| {
                    (
                        dir + starting_location,
                        starting_location,
                    )
                })
            })
            .filter(|(new_location, starting_location)| {
                !new_locations.contains(new_location)
                    && map.get(new_location).is_some_and(
                        |h| {
                            let current_height = map
                                .get(starting_location)
                                .unwrap();

                            *h == current_height + 1
                        },
                    )
            })
            .map(|(new, _)| new)
            .collect::<HashSet<IVec2>>();

        for loc in newer_locations.iter() {
            visited.insert(*loc);
        }

        new_locations = newer_locations;
    }
    visited
        .iter()
        .filter(|pos| map.get(pos).unwrap() == &9)
        .count()
}

pub type Span<'a> = LocatedSpan<&'a str>;
fn alphanum_pos(
    input: Span,
) -> IResult<Span, (IVec2, u32)> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, c) = satisfy(|c| c.is_numeric())(input)?;
    Ok((
        input,
        (
            IVec2::new(x, y),
            c.to_digit(10).unwrap(),
        ),
    ))
}
pub fn parse(
    input: Span,
) -> IResult<Span, HashMap<IVec2, u32>> {
    let (input, lines) = separated_list1(
        line_ending,
        many1(alphanum_pos),
    )(input)?;

    let hashmap = lines
        .iter()
        .flatten()
        .copied()
        .collect::<HashMap<IVec2, u32>>();

    Ok((input, hashmap))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("36", process(input)?);
        Ok(())
    }
}
