use glam::IVec2;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

const MAP_SIZE: IVec2 = if cfg!(test) {
    IVec2::new(11, 7)
} else {
    IVec2::new(101, 103)
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, mut robots) = parse(input).map_err(|e| {
        miette::miette!("parse failed {}", e)
    })?;

    // debug_grid(&robots);
    for _i in 0..100 {
        for robot in robots.iter_mut() {
            robot.position = (robot.position
                + robot.velocity)
                .rem_euclid(MAP_SIZE);
        }
    }
    // debug_grid(&robots);

    let halves = MAP_SIZE / 2;
    let quadrants = [
        (0..halves.x, 0..halves.y),
        ((halves.x + 1)..MAP_SIZE.x, 0..halves.y),
        (0..halves.x, (halves.y + 1)..MAP_SIZE.y),
        (
            (halves.x + 1)..MAP_SIZE.x,
            (halves.y + 1)..MAP_SIZE.y,
        ),
    ];
    let result: usize = quadrants
        .iter()
        .map(|(xs, ys)| {
            robots
                .iter()
                .filter(|Robot { position, .. }| {
                    xs.contains(&position.x)
                        && ys.contains(&position.y)
                })
                .count()
        })
        .product();

    Ok(result.to_string())
}

#[allow(dead_code)]
fn debug_grid(robots: &[Robot]) {
    println!("");
    for y in 0..MAP_SIZE.y {
        for x in 0..MAP_SIZE.x {
            match robots
                .iter()
                .filter(|Robot { position, .. }| {
                    position.x == x && position.y == y
                })
                .count()
            {
                0 => print!("."),
                n => print!("{}", n),
            }
        }
        println!("");
    }
}

#[derive(Debug)]
struct Robot {
    position: IVec2,
    velocity: IVec2,
}

fn parse_ivec2(input: &str) -> IResult<&str, IVec2> {
    let (input, (x, y)) = separated_pair(
        complete::i32,
        tag(","),
        complete::i32,
    )(input)?;
    Ok((input, IVec2::new(x, y)))
}

fn parse(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(
        line_ending,
        separated_pair(
            preceded(tag("p="), parse_ivec2),
            space1,
            preceded(tag("v="), parse_ivec2),
        )
        .map(|(position, velocity)| Robot {
            position,
            velocity,
        }),
    )(input)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("12", process(input)?);
        Ok(())
    }
}
