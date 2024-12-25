use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::{iterator, opt, peek, recognize},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, mut all) = parse(input).map_err(|e| {
        miette::miette!("parse failed {}", e)
    })?;
    let locks = all
        .extract_if(.., |device| {
            device.r#type == DeviceType::Lock
        })
        .collect::<Vec<Device>>();
    let keys = all;

    let count = locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(lock, key)| {
            std::iter::zip(lock.pins, key.pins)
                .all(|(a, b)| a + b <= 5)
        })
        .count();
    Ok(count.to_string())
}

#[derive(Debug, PartialEq, Eq)]
enum DeviceType {
    Lock,
    Key,
}

#[derive(Debug)]
struct Device {
    r#type: DeviceType,
    pins: [i32; 5],
}

fn accumulate_pins(
    mut pins: [i32; 5],
) -> impl FnMut(&str) -> IResult<&str, [i32; 5]> {
    move |input| {
        let mut it = iterator(
            input,
            terminated(
                alt((tag("#"), tag("."))),
                opt(line_ending),
            ),
        );

        for (i, value) in it.enumerate() {
            pins[i % 5] += match value {
                "#" => 1,
                _ => 0,
            };
        }
        let res: IResult<_, _> = it.finish();

        res.map(|(input, _)| (input, pins))
    }
}

fn lock(input: &str) -> IResult<&str, Device> {
    let (input, _) = tag("#####")(input)?;
    let (input, pins) = preceded(
        tuple((
            line_ending,
            peek(alt((tag("."), tag("#")))),
        )),
        accumulate_pins([0i32; 5]),
    )(input)?;
    Ok((
        input,
        Device {
            pins,
            r#type: DeviceType::Lock,
        },
    ))
}

fn key(input: &str) -> IResult<&str, Device> {
    let (input, _) = tag(".....")(input)?;

    let (input, pins) = preceded(
        tuple((
            line_ending,
            peek(alt((tag("."), tag("#")))),
        )),
        accumulate_pins([-1i32; 5]),
    )(input)?;
    Ok((
        input,
        Device {
            pins,
            r#type: DeviceType::Key,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Device>> {
    separated_list1(line_ending, alt((key, lock)))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
