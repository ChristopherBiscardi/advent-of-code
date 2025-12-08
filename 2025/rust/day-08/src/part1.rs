use glam::IVec3;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
};
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, positions) = parse(input).unwrap();
    let output = groups(positions, 3, 1000);
    Ok(output.to_string())
}

fn groups(
    positions: Vec<IVec3>,
    num_largest: usize,
    num_pairs: usize,
) -> usize {
    let mut connections: Vec<Vec<IVec3>> = vec![];
    for (a, b, _) in positions
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            (a, b, a.as_vec3().distance(b.as_vec3()))
        })
        .sorted_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .take(num_pairs)
    {
        let matches = connections
            .iter()
            .positions(|cluster| {
                let contains_a = cluster.contains(a);
                let contains_b = cluster.contains(b);
                contains_a || contains_b
            })
            .collect::<Vec<usize>>();
        match matches.as_slice() {
            [] => {
                connections.push(vec![*a, *b]);
            }
            [index] => {
                let cluster =
                    connections.get_mut(*index).unwrap();
                let contains_a = cluster.contains(a);
                let contains_b = cluster.contains(b);
                // cluster contains one of the junction boxes
                match (contains_a, contains_b) {
                    (true, true) => {
                        // do nothing, both are already in the cluster
                    }
                    (true, false) => {
                        cluster.push(*b);
                    }
                    (false, true) => {
                        cluster.push(*a);
                    }
                    (false, false) => {
                        panic!(
                            "We just filtered for a truth, so this should never happen"
                        );
                    }
                }
            }
            [index_a, index_b] => {
                let a = connections
                    .remove(*index_a.max(index_b));
                let b = connections
                    .remove(*index_a.min(index_b));
                let new_cluster = a
                    .into_iter()
                    .chain(b.into_iter())
                    .unique()
                    .collect::<Vec<IVec3>>();
                connections.push(new_cluster);
            }
            _ => {
                panic!("");
            }
        }
    }
    connections.sort_by(|a, b| b.len().cmp(&a.len()));

    connections
        .iter()
        .map(|v| v.len())
        .take(num_largest)
        .product()
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
        assert_eq!("40", process(input)?);
        Ok(())
    }
}
