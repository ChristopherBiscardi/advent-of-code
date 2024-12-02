use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    IResult,
};
use tracing::{info, instrument};

enum Direction {
    Increasing,
    Decreasing,
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, reports) = parse(input)
        .map_err(|e| miette!("parse failed {}", e))?;
    let result = reports
        .iter()
        .filter(|report| {
            if check_safety(report).is_err() {
                for index in 0..report.len() {
                    let mut new_report = (*report).clone();
                    new_report.remove(index);
                    if check_safety(&new_report).is_ok() {
                        return true;
                    } else {
                        continue;
                    }
                }
                return false;
            } else {
                true
            }
        })
        .count();
    Ok(result.to_string())
}

#[instrument(ret)]
fn check_safety(report: &Report) -> Result<(), String> {
    let mut direction: Option<Direction> = None;
    for (a, b) in report.iter().tuple_windows() {
        let diff = a - b;
        match diff.signum() {
            -1 => match direction {
                Some(Direction::Increasing) => {
                    return Err(format!(
                        "{}, {} switched to increasing",
                        a, b
                    ));
                }
                Some(Direction::Decreasing) => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(format!(
                            "{}, {} diff value is {}",
                            a,
                            b,
                            diff.abs()
                        ));
                    } else {
                        continue;
                    }
                }
                None => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(format!(
                            "{}, {} diff value is {}",
                            a,
                            b,
                            diff.abs()
                        ));
                    } else {
                        direction =
                            Some(Direction::Decreasing);
                        continue;
                    }
                }
            },
            1 => match direction {
                Some(Direction::Increasing) => {
                    if !(1..=3).contains(&diff) {
                        return Err(format!(
                            "{}, {} diff value is {}",
                            a,
                            b,
                            diff.abs()
                        ));
                    } else {
                        continue;
                    }
                }
                Some(Direction::Decreasing) => {
                    return Err(format!(
                        "{}, {} switched to decreasing",
                        a, b
                    ));
                }
                None => {
                    if !(1..=3).contains(&diff) {
                        return Err(format!(
                            "{}, {} diff value is {}",
                            a,
                            b,
                            diff.abs()
                        ));
                    } else {
                        direction =
                            Some(Direction::Increasing);
                        continue;
                    }
                }
            },
            0 => {
                return Err(format!(
                    "{}, {} diff was 0",
                    a, b
                ));
            }
            _ => {
                panic!(
                    "should never have a non -1,1,0 number"
                );
            }
        }
    }
    Ok(())
}

type Report = Vec<i32>;

fn parse(input: &str) -> IResult<&str, Vec<Report>> {
    separated_list1(
        newline,
        separated_list1(space1, complete::i32),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
