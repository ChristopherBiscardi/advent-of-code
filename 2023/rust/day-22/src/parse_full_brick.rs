use std::collections::HashMap;

use crate::custom_error::AocError;
use glam::{IVec2, IVec3, Vec3Swizzles};
use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

/// parses an IVec3 from `1,2,3`.
fn brick_end(input: &str) -> IResult<&str, IVec3> {
    let (input, x) = complete::i32(input)?;
    let (input, _) = complete::char(',')(input)?;
    let (input, y) = complete::i32(input)?;
    let (input, _) = complete::char(',')(input)?;
    let (input, z) = complete::i32(input)?;
    Ok((input, IVec3::new(x, y, z)))
}

#[derive(Debug)]
pub struct Brick {
    pub start: IVec3,
    pub end: IVec3,
}
fn brick(input: &str) -> IResult<&str, Brick> {
    let (input, (start, end)) = separated_pair(
        brick_end,
        complete::char('~'),
        brick_end,
    )(input)?;

    Ok((input, Brick { start, end }))
}
pub fn parse_bricks(
    input: &str,
) -> IResult<&str, Vec<Brick>> {
    separated_list1(line_ending, brick)(input)
}
