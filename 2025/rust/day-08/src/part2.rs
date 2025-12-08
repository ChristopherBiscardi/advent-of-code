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
    let output = groups(positions).unwrap();
    Ok(output.to_string())
}

fn groups(positions: Vec<IVec3>) -> miette::Result<usize> {
    let mut connections: Vec<Vec<IVec3>> = vec![];
    for (a, b, _) in positions
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            (a, b, a.as_vec3().distance(b.as_vec3()))
        })
        .sorted_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
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
                if connections.len() == 1
                    && connections[0].len()
                        == positions.len()
                {
                    // new cluster includes all positions
                    return Ok(a.x as usize * b.x as usize);
                }
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
                        if connections.len() == 1
                            && connections[0].len()
                                == positions.len()
                        {
                            // new cluster includes all positions
                            return Ok(
                                a.x as usize * b.x as usize
                            );
                        }
                    }
                    (false, true) => {
                        cluster.push(*a);
                        if connections.len() == 1
                            && connections[0].len()
                                == positions.len()
                        {
                            // new cluster includes all positions
                            return Ok(
                                a.x as usize * b.x as usize
                            );
                        }
                    }
                    (false, false) => {
                        panic!(
                            "We just filtered for a truth, so this should never happen"
                        );
                    }
                }
            }
            [index_a, index_b] => {
                let a_group = connections
                    .remove(*index_a.max(index_b));
                let b_group = connections
                    .remove(*index_a.min(index_b));
                let new_cluster = a_group
                    .into_iter()
                    .chain(b_group.into_iter())
                    .unique()
                    .collect::<Vec<IVec3>>();
                if new_cluster.len() == positions.len() {
                    // new cluster includes all positions
                    return Ok(a.x as usize * b.x as usize);
                }
                connections.push(new_cluster);
            }
            _ => {
                return Err(miette::miette!(
                    "{}",
                    "encountered too many matching indices"
                ));
            }
        }
    }

    info!(
        conn_len = connections[0].len(),
        connections=?connections[0],
        pos_len = positions.len(),
        ?positions
    );
    Err(miette::miette!("{}", "uncalculatable"))
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
