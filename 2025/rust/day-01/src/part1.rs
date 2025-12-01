use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
};

const STARTING_POSITION: i32 = 50;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, directions) = directions.parse(input).unwrap();

    let (_final_position, counter) =
        directions.iter().fold(
            (STARTING_POSITION, 0),
            |(dial, counter), direction| {
                let num = match direction {
                    Direction::Left(num) => -num,
                    Direction::Right(num) => *num,
                };
                let next_dial =
                    (dial - num).rem_euclid(100);
                let additional_counters =
                    if next_dial == 0 { 1 } else { 0 };
                (next_dial, counter + additional_counters)
            },
        );

    Ok(counter.to_string())
}

#[derive(Debug)]
enum Direction {
    Left(i32),
    Right(i32),
}

fn directions(
    input: &str,
) -> IResult<&str, Vec<Direction>> {
    separated_list1(line_ending, direction).parse(input)
}

fn direction(input: &str) -> IResult<&str, Direction> {
    let (input, dir) =
        alt((tag("L"), tag("R"))).parse(input)?;
    let (input, num) = complete::i32(input)?;

    let d = match dir {
        "L" => Direction::Left(num),
        "R" => Direction::Right(num),
        x => panic!("unknown {x}"),
    };

    Ok((input, d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
