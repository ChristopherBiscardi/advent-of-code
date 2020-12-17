use itertools::Itertools;
use std::convert::TryInto;

use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
enum NodeState {
    Active,
    Inactive,
}
type Space = HashMap<Vec<isize>, NodeState>;
fn parse(input: &str, dimensions: usize) -> Space {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| {
                    (
                        x.try_into().unwrap(),
                        y.try_into().unwrap(),
                        NodeState::Active,
                    )
                })
        })
        .fold(HashMap::new(), |mut map, cur| {
            for item in cur {
                let mut v = vec![item.0, item.1];
                for _ in 0..(dimensions - 2) {
                    v.push(0);
                }
                map.insert(v, item.2);
            }
            map
        })
}

fn make_points(length: usize) -> Vec<Vec<isize>> {
    let points = [-1, 0, 1];

    let mut prod = points
        .iter()
        .cloned()
        .cartesian_product(points.iter().cloned())
        .map(|(a, b)| vec![a, b])
        .collect::<Vec<Vec<isize>>>();

    for _ in 0..(length - 2) {
        prod = prod
            .iter()
            .cloned()
            .cartesian_product(points.iter().cloned())
            .map(|(mut a, b)| {
                a.push(b);
                a
            })
            .collect::<Vec<Vec<isize>>>();
    }

    prod
}

fn get_cube_points(position: &Vec<isize>, relative_points: &Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    relative_points
        .iter()
        .cloned()
        // filter 0,0,0 (aka self) out
        .filter(|vs| !vs.iter().all(|scalar| *scalar == 0))
        .map(|scalars| {
            scalars
                .into_iter()
                .enumerate()
                .map(|(i, scalar)| scalar + position[i])
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>()
}
fn get_active_neighbors(
    cube_points: &Vec<Vec<isize>>,
    space: &HashMap<Vec<isize>, NodeState>,
) -> usize {
    cube_points
        .iter()
        .filter_map(|xyz| {
            // let xyz =
            if let Some(NodeState::Active) = space.get(xyz) {
                Some(true)
            } else {
                None
            }
        })
        .count()
}
fn run(space: Space, relative_points: &Vec<Vec<isize>>) -> Space {
    let mut new_space = space.clone();
    let mut inactives: Vec<Vec<isize>> = vec![];
    for (position, node_state) in space.iter() {
        // get_neighbors
        let cube_points = get_cube_points(&position, &relative_points);

        let active_neighbors_count = get_active_neighbors(&cube_points, &space);
        // dbg!(active_neighbors_count);
        match node_state {
            NodeState::Active => {
                if active_neighbors_count == 2 || active_neighbors_count == 3 {
                    // stay active
                } else {
                    // go inactive
                    // println!("[actives] removing {:?}", position);
                    new_space.remove(position);
                }
            }
            NodeState::Inactive => {
                if active_neighbors_count == 3 {
                    // println!("[actives] inserting {:?}", position);
                    new_space.insert(position.to_vec(), NodeState::Active);
                } else {
                    // stay inactive
                }
            }
        }
        for point in cube_points.iter() {
            inactives.push(point.to_vec());
        }

        // space.get()
    }
    for (position) in inactives.iter().unique() {
        // let node_state = space.get(position);

        let cube_points = get_cube_points(&position, &relative_points);
        let active_neighbors_count = get_active_neighbors(&cube_points, &space);
        // these are inactive
        if active_neighbors_count == 3 {
            // println!("[inactives] inserting {:?}", position);
            new_space.insert(position.to_vec(), NodeState::Active);
        } else {
            // stay inactive
        }
    }

    new_space
}
pub fn process_part1(input: &str) -> usize {
    let nodes = parse(input, 3);
    let relative_points = make_points(3);
    let mut new_space = nodes;
    for _ in 0..6 {
        new_space = run(new_space, &relative_points);
    }
    new_space.len()
}

pub fn process_part2(input: &str) -> usize {
    let nodes = parse(input, 4);
    let relative_points = make_points(4);
    let mut new_space = nodes;
    for _ in 0..6 {
        new_space = run(new_space, &relative_points);
    }
    new_space.len()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_input_process_one() {
        assert_eq!(
            process_part1(
                "\
.#.
..#
###"
            ),
            112
        )
    }

    #[test]
    fn test_input_process_two() {
        assert_eq!(
            process_part2(
                "\
.#.
..#
###"
            ),
            848
        )
    }
}
