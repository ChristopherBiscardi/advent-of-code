use std::ops::Add;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::preceded,
};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, directions) =
        all_consuming(directions).parse(input).unwrap();

    let resulting_dial = directions.iter().fold(
        Dial {
            location: 50,
            revolutions: 0,
        },
        |mut dial, direction| {
            dial.spin(*direction);
            dial
        },
    );

    Ok(resulting_dial.revolutions.to_string())
}

#[derive(Debug, PartialEq)]
struct Dial {
    /// the current location of the dial
    location: i32,
    /// number of times we've passed or landed
    /// on 0
    revolutions: i32,
}

impl Dial {
    fn new(location: i32) -> Self {
        Self {
            location,
            revolutions: 0,
        }
    }
    fn from_tuple(
        (starting_position, counter): (i32, i32),
    ) -> Self {
        Self {
            location: starting_position,
            revolutions: counter,
        }
    }
    fn spin(&mut self, rot: Direction) {
        let dial_long = self.location + rot;
        let mut revolutions =
            (dial_long / DIAL_TOTAL).abs();

        if self.location != 0 && dial_long <= 0 {
            revolutions += 1;
        }

        self.location = dial_long.rem_euclid(DIAL_TOTAL);
        self.revolutions += revolutions;
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left(i32),
    Right(i32),
}
impl Add<Direction> for i32 {
    type Output = i32;

    fn add(self, rhs: Direction) -> Self::Output {
        self + match rhs {
            Direction::Left(num) => -num,
            Direction::Right(num) => num,
        }
    }
}

fn directions(
    input: &str,
) -> IResult<&str, Vec<Direction>> {
    separated_list1(line_ending, direction).parse(input)
}

fn direction(input: &str) -> IResult<&str, Direction> {
    alt((
        preceded(tag("L"), complete::i32)
            .map(Direction::Left),
        preceded(
            tag("R"),
            complete::i32.map(Direction::Right),
        ),
    ))
    .parse(input)
}

const DIAL_TOTAL: i32 = 100;

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
    #[case((20, 0), 50, Direction::Left(30))]
    #[case((90, 1), 50, Direction::Left(60))]
    #[case((90, 3), 50, Direction::Left(260))]
    #[case((80, 0), 50, Direction::Right(30))]
    #[case((10, 1), 50, Direction::Right(60))]
    #[case((10, 4), 50, Direction::Right(360))]
    #[case((90, 0), 0, Direction::Left(10))]
    #[case((0, 1), 0, Direction::Left(100))]
    #[case((10, 0), 0, Direction::Right(10))]
    #[case((0, 1), 0, Direction::Right(100))]
    #[case((82, 1), 50, Direction::Left(68))]
    #[case((52, 0), 82, Direction::Left(30))]
    #[case((0, 1), 52, Direction::Right(48))]
    #[case((95, 0), 0, Direction::Left(5))]
    #[case((55, 1), 95, Direction::Right(60))]
    #[case((0, 1), 55, Direction::Left(55))]
    #[case((99, 0), 0, Direction::Left(1))]
    #[case((0, 1), 99, Direction::Left(99))]
    #[case((14, 0), 0, Direction::Right(14))]
    #[case((32, 1), 14, Direction::Left(82))]
    fn spin_test(
        #[case] expected: (i32, i32),
        #[case] starting_position: i32,
        #[case] rotation: Direction,
    ) {
        let mut dial = Dial::new(starting_position);
        dial.spin(rotation);
        assert_eq!(Dial::from_tuple(expected), dial);
    }
}
