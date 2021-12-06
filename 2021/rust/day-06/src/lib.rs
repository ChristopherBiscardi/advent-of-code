use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::opt,
    multi::{fill, separated_list1},
    IResult,
};

pub fn puzzle_input(
    input: &str,
) -> IResult<&str, VecDeque<u32>> {
    let (input, fish) =
        separated_list1(tag(","), digit1)(input)?;
    let mut arr = [0; 9];
    for num in fish.iter() {
        let num: usize = num.parse().unwrap();
        arr[num] += 1
    }
    let que: VecDeque<u32> = VecDeque::from(arr);

    Ok((input, que))
}
