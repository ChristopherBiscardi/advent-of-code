use bevy::utils::HashSet;
use glam::IVec2;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a [u8]>;

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
) -> IResult<Span, (IVec2, HashSet<IVec2>)> {
    let (input, items) =
        separated_list1(line_ending, many1(token))(input)?;

    let y_max = items
        .iter()
        .flatten()
        .map(|(pos, _)| pos.y)
        .max()
        .unwrap();

    let player = items
        .iter()
        .flatten()
        .find(|(_, value)| value == &'^')
        .cloned()
        .map(|(v, _)| (v - IVec2::new(0, y_max)).abs())
        .expect("should have a player");
    let walls = items
        .into_iter()
        .flatten()
        .filter(|(_, value)| value == &'#')
        .map(|(v, _)| (v - IVec2::new(0, y_max)).abs())
        .collect::<HashSet<IVec2>>();
    Ok((input, (player, walls)))
}
