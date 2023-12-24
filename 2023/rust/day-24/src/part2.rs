use glam::{DVec2, DVec3, I64Vec3, Vec3Swizzles};
use itertools::Itertools;
use ndarray_linalg::error::LinalgError;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

use crate::custom_error::AocError;

#[derive(Debug)]
struct Hail {
    starting_position: I64Vec3,
    direction: I64Vec3,
}

#[derive(Debug)]
struct Hailf64 {
    starting_position: DVec3,
    direction: DVec3,
}

impl Hail {
    pub fn at(&self, scalar: f64) -> glam::f64::DVec2 {
        self.starting_position.as_dvec3().xy()
            + scalar * self.direction.as_dvec3().xy()
    }
    fn solve_intersection(
        &self,
        hail_two: &Hail,
    ) -> Result<(f64, f64, DVec2), LinalgError> {
        use ndarray::prelude::*;
        use ndarray_linalg::Solve;

        let a: Array2<f64> = array![
            [
                self.direction.x as f64,
                -hail_two.direction.x as f64
            ],
            [
                self.direction.y as f64,
                -hail_two.direction.y as f64
            ],
        ];
        let b: Array1<f64> = array![
            hail_two.starting_position.x as f64
                - self.starting_position.x as f64,
            hail_two.starting_position.y as f64
                - self.starting_position.y as f64
        ];
        let x = a.solve_into(b)?;
        let xx = x[0];
        let yy = x[1];
        // assert_eq!(self.at(xx), hail_two.at(yy));
        // dbg!(xx);
        Ok((xx, yy, self.at(xx)))
    }
}

fn ivec3(input: &str) -> IResult<&str, I64Vec3> {
    let (input, x) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space1)(input)?;
    let (input, y) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space1)(input)?;
    let (input, z) = complete::i64(input)?;

    Ok((input, I64Vec3::new(x, y, z)))
}
fn parse(input: &str) -> IResult<&str, Vec<Hail>> {
    separated_list1(
        line_ending,
        separated_pair(
            ivec3,
            delimited(space1, tag("@"), space1),
            ivec3,
        )
        .map(|(starting_position, direction)| {
            Hail {
                starting_position,
                direction,
            }
        }),
    )(input)
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (_, hail) = parse(input).expect("should parse");
    // the bound for the tests
    // let bound = 7f64..=27f64;
    let bound = 200000000000000f64..=400000000000000f64;
    let results = hail
        .iter()
        .tuple_combinations()
        .filter_map(|(hail_one, hail_two)| {
            hail_one.solve_intersection(&hail_two).ok().map(
                |intersection| {
                    ((hail_one, hail_two), intersection)
                },
            )
        })
        .filter(|(_, (xx,yy, result))| {
            bound.contains(&result.x)
                && bound.contains(&result.y)
                && xx >= &0. && yy >= &0.
        })
        .collect::<Vec<((&Hail, &Hail), (f64, f64, DVec2))>>();
    //     for result in results {
    //         println!(
    //             "\n{:?}\n{:?}
    // hailstone_a_solve: {:?}
    // hailstone_b_solve: {:?}
    // {:?}",
    //             result.0 .0,
    //             result.0 .1,
    //             result.1 .0,
    //             result.1 .1,
    //             result.1 .2
    //         );
    //     }
    Ok(results.len().to_string())
}

// 19, 13, 30 @ -2,  1, -2
// 18, 19, 22 @ -1, -1, -2
fn hardcoded_solve_intersection() {
    use ndarray::prelude::*;
    use ndarray_linalg::Solve;

    // 19 - 2 * a = 18 - a
    // 13 + b = 19 - b
    //
    // 1 - a
    // -6 + 2b
    let a: Array2<f64> = array![
        [-2., 1.],
        [1., 1.],
        // [-2., 1., -2.]
    ];
    let b: Array1<f64> = array![18. - 19., 19. - 13.];
    let x = a.solve_into(b).unwrap();
    let xx = x[0];
    let yy = x[1];
    // dbg!(x[0], x[1]);
    // solved eq 1
    // dbg!(19. - 2. * xx);
    // dbg!(13. + xx);

    // solved eq 2
    // dbg!(18. - yy);
    // dbg!(19. - yy);
}

// 19, 13, 30 @ -2,  1, -2
// 18, 19, 22 @ -1, -1, -2
fn solve_intersection() {
    use ndarray::prelude::*;
    use ndarray_linalg::Solve;

    let h1 = Hailf64 {
        starting_position: DVec3::new(
            291493672529314.,
            259618209733833.,
            379287136024123.,
        ),
        direction: DVec3::new(-9., 119., -272.),
    };

    // let h1 = Hailf64 {
    //     starting_position: DVec3::new(19., 13., 30.),
    //     direction: DVec3::new(-2., 1., -2.),
    // };
    let v1 = vec![
        h1.direction.x,
        h1.direction.y,
        h1.direction.z,
    ];
    let p1 = vec![
        h1.starting_position.x,
        h1.starting_position.y,
        h1.starting_position.z,
    ];
    let h2 = Hailf64 {
        starting_position: DVec3::new(
            308409248682955.,
            156803514643857.,
            424989308414284.,
        ),
        direction: DVec3::new(-78., 236., -255.),
    };
    //    let h2 = Hailf64 {
    //     starting_position: DVec3::new(18., 19., 22.),
    //     direction: DVec3::new(-1., -1., -2.),
    // };

    let v2 = vec![
        h2.direction.x,
        h2.direction.y,
        h2.direction.z,
    ];
    let p2 = vec![
        h2.starting_position.x,
        h2.starting_position.y,
        h2.starting_position.z,
    ];
    let h3 = Hailf64 {
        starting_position: DVec3::new(
            195379943194796.,
            213851381371727.,
            355270583377422.,
        ),
        direction: DVec3::new(25., 14., -15.),
    };
    // let h3 = Hailf64 {
    //     starting_position: DVec3::new(18., 19., 22.),
    //     direction: DVec3::new(-1., -1., -2.),
    // };

    let v3 = vec![
        h3.direction.x,
        h3.direction.y,
        h3.direction.z,
    ];
    let p3 = vec![
        h3.starting_position.x,
        h3.starting_position.y,
        h3.starting_position.z,
    ];

    // 19 - 2 * a = 18 - a
    // 13 + b = 19 - b
    //
    // 1 - a
    // -6 + 2b

    let a: Array2<f64> = array![
        [
            -(v1[1] - v2[1]),
            v1[0] - v2[0],
            0.,
            p1[1] - p2[1],
            -(p1[0] - p2[0]),
            0.
        ],
        [
            -(v1[1] - v3[1]),
            v1[0] - v3[0],
            0.,
            p1[1] - p3[1],
            -(p1[0] - p3[0]),
            0.
        ],
        [
            0.,
            -(v1[2] - v2[2]),
            v1[1] - v2[1],
            0.,
            p1[2] - p2[2],
            -(p1[1] - p2[1])
        ],
        [
            0.,
            -(v1[2] - v3[2]),
            v1[1] - v3[1],
            0.,
            p1[2] - p3[2],
            -(p1[1] - p3[1])
        ],
        [
            -(v1[2] - v2[2]),
            0.,
            v1[0] - v2[0],
            p1[2] - p2[2],
            0.,
            -(p1[0] - p2[0])
        ],
        [
            -(v1[2] - v3[2]),
            0.,
            v1[0] - v3[0],
            p1[2] - p3[2],
            0.,
            -(p1[0] - p3[0])
        ]
    ];
    let b: Array1<f64> = array![
        (p1[1] * v1[0] - p2[1] * v2[0])
            - (p1[0] * v1[1] - p2[0] * v2[1]),
        (p1[1] * v1[0] - p3[1] * v3[0])
            - (p1[0] * v1[1] - p3[0] * v3[1]),
        (p1[2] * v1[1] - p2[2] * v2[1])
            - (p1[1] * v1[2] - p2[1] * v2[2]),
        (p1[2] * v1[1] - p3[2] * v3[1])
            - (p1[1] * v1[2] - p3[1] * v3[2]),
        (p1[2] * v1[0] - p2[2] * v2[0])
            - (p1[0] * v1[2] - p2[0] * v2[2]),
        (p1[2] * v1[0] - p3[2] * v3[0])
            - (p1[0] * v1[2] - p3[0] * v3[2])
    ];
    let coefficients = a.solve_into(b).unwrap();
    dbg!(coefficients);

    // assert_eq!(hail_one.at(xx), hail_two.at(yy));
    // hail_one.at(xx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn test_solve() {
        solve_intersection();
        panic!("just a run test");
    }

    #[test]
    #[ignore]
    fn test_process() -> miette::Result<()> {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!("47", process(input)?);
        Ok(())
    }
}

//
// 𝑎1𝑥1+𝑎2𝑥2+...+𝑎𝑛𝑥𝑛=𝑏
