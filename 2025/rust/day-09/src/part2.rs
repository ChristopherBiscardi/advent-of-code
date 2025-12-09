use glam::I64Vec2;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
};
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, red_tiles) = parse(input).unwrap();
    let lines = red_tiles
        .iter()
        .circular_tuple_windows()
        .collect::<Vec<(&I64Vec2, &I64Vec2)>>();
    let max_box = red_tiles
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let area = (a.x.abs_diff(b.x) + 1)
                * (a.y.abs_diff(b.y) + 1);
            (a, b, area)
        })
        .sorted_by_key(|v| v.2)
        .rev()
        .find(|(a, b, area)| {
            lines.iter().all(|(line_start, line_end)| {
                // if line is to left
                let left_of_rect = a.x.max(b.x)
                    <= line_start.x.min(line_end.x);
                let right_of_rect = a.x.min(b.x)
                    >= line_start.x.max(line_end.x);
                let above = a.y.max(b.y)
                    <= line_start.y.min(line_end.y);
                let below = a.y.min(b.y)
                    >= line_start.y.max(line_end.y);
                left_of_rect
                    || right_of_rect
                    || above
                    || below
            })
        });

    Ok(max_box.unwrap().2.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<I64Vec2>> {
    separated_list1(
        line_ending,
        separated_pair(
            complete::i64,
            tag(","),
            complete::i64,
        )
        .map(|(x, y)| I64Vec2::new(x, y)),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!("50", process(input)?);
        Ok(())
    }
}
// let t_top = (one.x - three.x)
//     * (three.y - four.y)
//     - (one.y - three.y)
//         * (three.x - four.x);
// let t_bottom = (one.x - two.x)
//     * (three.y - four.y)
//     - (one.y - two.y)
//         * (three.x - four.x);

// let u_top = (one.x - two.x)
//     * (one.y - three.y)
//     - (one.y - two.y)
//         * (one.x - three.x);
// let u_bottom = (one.x - two.x)
//     * (three.y - four.y)
//     - (one.y - two.y)
//         * (three.x - four.x);

// if t_bottom == 0 || u_bottom == 0 {
//     // coincident line, so no intersection necessary
//     return false;
// }

// let t = t_top / t_bottom;
// let u = u_top / u_bottom;

// if 0 <= t && t <= 1 && 0 <= u && u <= 1
// {
//     true
// } else {
//     false
// }
