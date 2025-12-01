use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, directions) = directions.parse(input).unwrap();

    let mut dial = 50;
    let mut counter = 0;

    for direction in directions {
        let num = match direction {
            Direction::Left(num) => -num,
            Direction::Right(num) => num,
        };
        let (new_dial, additional_counters) =
            spin(dial, num);
        dial = new_dial;
        counter += additional_counters;
    }

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

const DIAL_TOTAL: i32 = 100;

fn spin(dial: i32, rot: i32) -> (i32, i32) {
    let dial_long = dial + rot;
    let mut revolutions = (dial_long / DIAL_TOTAL).abs();

    if dial != 0 && dial_long <= 0 {
        revolutions += 1;
    }

    (
        dial_long.rem_euclid(DIAL_TOTAL),
        revolutions,
    )
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
        assert_eq!("6", process(input)?);
        Ok(())
    }

    use rstest::rstest;

    #[rstest]
    #[case((20, 0), 50, -30)]
    #[case((90, 1), 50, -60)]
    #[case((90, 3), 50, -260)]
    #[case((80, 0), 50, 30)]
    #[case((10, 1), 50, 60)]
    #[case((10, 4), 50, 360)]
    #[case((90, 0), 0, -10)]
    #[case((0, 1), 0, -100)]
    #[case((10, 0), 0, 10)]
    #[case((0, 1), 0, 100)]
    #[case((82, 1), 50, -68)]
    #[case((52, 0), 82, -30)]
    #[case((0, 1), 52, 48)]
    #[case((95, 0), 0, -5)]
    #[case((55, 1), 95, 60)]
    #[case((0, 1), 55, -55)]
    #[case((99, 0), 0, -1)]
    #[case((0, 1), 99, -99)]
    #[case((14, 0), 0, 14)]
    #[case((32, 1), 14, -82)]
    fn spin_test(
        #[case] expected: (i32, i32),
        #[case] starting_position: i32,
        #[case] rotation: i32,
    ) {
        assert_eq!(
            expected,
            spin(starting_position, rotation)
        );
    }
}
