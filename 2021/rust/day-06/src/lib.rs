use std::collections::VecDeque;

use nom::{
    bytes::complete::tag, character::complete::u8,
    multi::separated_list1, IResult,
};

pub fn puzzle_input(
    input: &str,
) -> IResult<&str, VecDeque<u64>> {
    let (input, fish) =
        separated_list1(tag(","), u8)(input)?;
    let mut arr = [0; 9];
    for num in fish.iter() {
        // let num: usize = num.parse().unwrap();
        arr[*num as usize] += 1
    }
    let que: VecDeque<u64> = VecDeque::from(arr);

    Ok((input, que))
}
