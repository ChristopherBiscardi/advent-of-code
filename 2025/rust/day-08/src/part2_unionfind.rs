use glam::IVec3;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
};
use petgraph::unionfind::UnionFind;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, positions) = parse(input).unwrap();
    let output = groups(positions).unwrap();
    Ok(output.to_string())
}

fn groups(positions: Vec<IVec3>) -> miette::Result<usize> {
    // let mut connections: Vec<Vec<IVec3>> = vec![];
    let mut connections = UnionFind::new(positions.len());

    let mut result: usize = 0;
    for (a, b, _) in positions
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|(a, b)| {
            (
                a,
                b,
                a.1.as_vec3().distance(b.1.as_vec3()),
            )
        })
        .sorted_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
    {
        if connections.union(a.0, b.0) {
            // new union happened; check to see if we made a loop
            if (0..positions.len())
                .into_iter()
                .tuple_windows()
                .all(|(a, b)| connections.equiv(a, b))
            {
                result = a.1.x as usize * b.1.x as usize;
                break;
            }
        }
    }

    Ok(result)
}

fn parse(input: &str) -> IResult<&str, Vec<IVec3>> {
    separated_list1(
        line_ending,
        separated_list1(tag(","), complete::i32)
            .map(|v| IVec3::from_slice(&v)),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!("25272", process(input)?);
        Ok(())
    }
}
