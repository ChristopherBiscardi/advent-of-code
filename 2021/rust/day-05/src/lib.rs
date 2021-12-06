use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::opt,
    multi::{fill, separated_list1},
    IResult,
};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

pub fn point(input: &str) -> IResult<&str, Point> {
    let (input, x) = digit1(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = digit1(input)?;
    Ok((
        input,
        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        },
    ))
}
pub fn points(
    input: &str,
) -> IResult<&str, (Point, Point)> {
    let (input, a) = point(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, b) = point(input)?;
    Ok((input, (a, b)))
}
pub fn puzzle_input(
    input: &str,
) -> IResult<&str, Vec<(Point, Point)>> {
    let (input, lines) =
        separated_list1(newline, points)(input)?;
    Ok((input, lines))
}
