use glam::UVec2;
use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{
        preceded, separated_pair, terminated, tuple,
    },
    IResult, Parser,
};
use pathfinding::prelude::dijkstra;

const A_COST: u32 = 3;
const B_COST: u32 = 1;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, machines) = parse(input)
        .map_err(|e| miette!("parse failed {}", e))?;

    let sum: u32 = machines
        .iter()
        .map(|machine| {
            let start_node = UVec2::ZERO;
            let result = dijkstra(
                &start_node,
                |position| {
                    if position.x > machine.prize.x
                        || position.y > machine.prize.y
                    {
                        vec![]
                    } else {
                        vec![
                            (position + machine.a, A_COST),
                            (position + machine.b, B_COST),
                        ]
                    }
                },
                |&p| p == machine.prize,
            );
            result.map(|(_, cost)| cost)
        })
        .flatten()
        .sum();

    Ok(sum.to_string())
}

#[derive(Debug)]
struct Machine {
    a: UVec2,
    b: UVec2,
    prize: UVec2,
}

fn a_button(input: &str) -> IResult<&str, UVec2> {
    preceded(
        tag("Button A: X+"),
        separated_pair(
            complete::u32,
            tag(", Y+"),
            complete::u32,
        )
        .map(|(x, y)| UVec2::new(x, y)),
    )(input)
}
fn b_button(input: &str) -> IResult<&str, UVec2> {
    preceded(
        tag("Button B: X+"),
        separated_pair(
            complete::u32,
            tag(", Y+"),
            complete::u32,
        )
        .map(|(x, y)| UVec2::new(x, y)),
    )(input)
}
fn prize(input: &str) -> IResult<&str, UVec2> {
    preceded(
        tag("Prize: X="),
        separated_pair(
            complete::u32,
            tag(", Y="),
            complete::u32,
        )
        .map(|(x, y)| UVec2::new(x, y)),
    )(input)
}

fn machine(input: &str) -> IResult<&str, Machine> {
    let (input, (a, b, p)) = tuple((
        terminated(a_button, line_ending),
        terminated(b_button, line_ending),
        prize,
    ))(input)?;

    Ok((input, Machine { a, b, prize: p }))
}
fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(
        tuple((line_ending, line_ending)),
        machine,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", process(input)?);
        Ok(())
    }
}
