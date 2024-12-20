use glam::{DMat2, U64Vec2};
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

const A_COST: u64 = 3;
const B_COST: u64 = 1;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, machines) = parse(input)
        .map_err(|e| miette!("parse failed {}", e))?;

    let sum: u64 = machines
        .iter()
        .filter_map(|machine| {
            machine
                .solve()
                .map(|value| {
                    (value * U64Vec2::new(A_COST, B_COST))
                        .element_sum()
                })
                .ok()
        })
        .sum();

    Ok(sum.to_string())
}

#[derive(Debug)]
struct Machine {
    a: U64Vec2,
    b: U64Vec2,
    prize: U64Vec2,
}

impl Machine {
    fn solve(&self) -> Result<U64Vec2, ()> {
        let mat = DMat2::from_cols_array(&[
            self.a.x as f64,
            self.a.y as f64,
            self.b.x as f64,
            self.b.y as f64,
        ]);
        let d = mat.determinant();
        let mat_ac = DMat2::from_cols_array(&[
            self.prize.x as f64,
            self.prize.y as f64,
            self.b.x as f64,
            self.b.y as f64,
        ]);
        let d_ac = mat_ac.determinant();

        let mat_bc = DMat2::from_cols_array(&[
            self.a.x as f64,
            self.a.y as f64,
            self.prize.x as f64,
            self.prize.y as f64,
        ]);
        let d_bc = mat_bc.determinant();
        // // dbg!(d, d_ac, d_bc);
        let x = d_ac / d;
        let y = d_bc / d;
        // dbg!(x, y);
        // if x < 0. || y < 0. {
        //     return Err(());
        // }
        if x.trunc() != x || y.trunc() != y {
            return Err(());
        }
        let max =
            if cfg!(test) { 100f64 } else { f64::INFINITY };

        if x > max || y > max {
            Err(())
        } else {
            Ok(U64Vec2::new(x as u64, y as u64))
        }
    }
}

fn a_button(input: &str) -> IResult<&str, U64Vec2> {
    preceded(
        tag("Button A: X+"),
        separated_pair(
            complete::u64,
            tag(", Y+"),
            complete::u64,
        )
        .map(|(x, y)| U64Vec2::new(x, y)),
    )(input)
}
fn b_button(input: &str) -> IResult<&str, U64Vec2> {
    preceded(
        tag("Button B: X+"),
        separated_pair(
            complete::u64,
            tag(", Y+"),
            complete::u64,
        )
        .map(|(x, y)| U64Vec2::new(x, y)),
    )(input)
}
fn prize(input: &str) -> IResult<&str, U64Vec2> {
    preceded(
        tag("Prize: X="),
        separated_pair(
            complete::u64,
            tag(", Y="),
            complete::u64,
        )
        .map(|(x, y)| {
            U64Vec2::new(
                x + if cfg!(test) {
                    0
                } else {
                    10000000000000
                },
                y + if cfg!(test) {
                    0
                } else {
                    10000000000000
                },
            )
        }),
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
    // use glam::DMat2;

    use super::*;

    // #[test]
    // fn sandbox() {
    //     // intersection of both lines
    //     let mat =
    //         DMat2::from_cols_array(&[94., 34., 22.,
    // 67.]);     let d = mat.determinant();
    //     let mat_ac = DMat2::from_cols_array(&[
    //         8400., 5400., 22., 67.,
    //     ]);
    //     let d_ac = mat_ac.determinant();

    //     let mat_bc = DMat2::from_cols_array(&[
    //         94., 34., 8400., 5400.,
    //     ]);
    //     let d_bc = mat_bc.determinant();

    //     dbg!(d, d_ac, d_bc);
    //     let x = d_ac / d;
    //     let y = d_bc / d;
    //     dbg!(x, y);

    //     assert!(false);
    // }
    // cramer's rule
    // [[94, 22],[34, 67]] * [x,y] == [8400,5400]
    //
    // 94a + 22b == 8400
    // 34a + 67b == 5400
    //
    // cost == 3a + b
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
