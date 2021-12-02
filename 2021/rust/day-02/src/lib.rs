use nom::{
    branch::alt, bytes::complete::tag,
    character::complete::i32, IResult,
};
pub enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}
pub fn parse_direction(
    input: &str,
) -> IResult<&str, Direction> {
    let (input, dir) =
        alt((tag("forward"), tag("up"), tag("down")))(
            input,
        )?;
    let (input, _) = tag(" ")(input)?;
    let (input, magnitude) = i32(input)?;

    let result = match dir {
        "forward" => Direction::Forward(magnitude),
        "up" => Direction::Up(magnitude),
        "down" => Direction::Down(magnitude),
        _ => {
            panic!("invalid")
        }
    };

    Ok((input, result))
}
