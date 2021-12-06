use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::opt,
    multi::{fill, separated_list1},
    IResult,
};

pub fn puzzle_input(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, fish) =
        separated_list1(tag(","), digit1)(input)?;
    Ok((
        input,
        fish.iter().map(|s| s.parse().unwrap()).collect(),
    ))
}
