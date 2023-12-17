use std::{collections::HashMap, fmt::Display};

use glam::IVec2;


use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Rock {
    Movable,
    Immovable,
}
impl Display for Rock {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rock::Movable => "O",
                Rock::Immovable => "#",
            }
        )
    }
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let columns = input.lines().next().unwrap().len();
    let rows = input.lines().count();
    let boundaries =
        IVec2::new(columns as i32, rows as i32);
    // dbg!(rows, columns);
    let rock_map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(
                move |(x, c)| match c {
                    'O' => Some((
                        IVec2::new(x as i32, y as i32),
                        Rock::Movable,
                    )),
                    '#' => Some((
                        IVec2::new(x as i32, y as i32),
                        Rock::Immovable,
                    )),
                    _ => None,
                },
            )
        })
        .collect::<HashMap<IVec2, Rock>>();

    let static_rocks = rock_map
        .iter()
        .filter_map(|(position, rock)| match rock {
            Rock::Movable => None,
            Rock::Immovable => Some((*position, *rock)),
        })
        .collect::<HashMap<IVec2, Rock>>();

    // println!("first grid");
    // print_grid(&rock_map, &boundaries);
    // remember each cycle is 4 directions
    // let cycles = 1000000000;
    let cycles = 1000000;
    // let total_rounds = cycles * 4;
    // let mut cache_hits = 0;
    let mut iteration_maps: Vec<String> = vec![];
    let mut cache =
        HashMap::<String, HashMap<IVec2, Rock>>::new();

    // |old_map, iteration| {
    let mut old_map = rock_map;
    for _iteration in 0..cycles {
        // println!("v-iter-{iteration}-v");
        let next_state = match cache
            .get(&grid_to_string(&old_map, &boundaries))
        {
            Some(_cached_next_state) => {
                // dbg!("cache hit at", iteration);
                break;
                // panic!("here");
                // cache_hits += 1;
                // dbg!("hit cache");
                // cached_next_state.clone()
            }
            None => {
                let next_state = {
                    let next_state = rock_shift_north(
                        &old_map,
                        &boundaries,
                        &static_rocks,
                    );
                    let next_state = rock_shift_west(
                        &next_state,
                        &boundaries,
                        &static_rocks,
                    );
                    let next_state = rock_shift_south(
                        &next_state,
                        &boundaries,
                        &static_rocks,
                    );
                    
                    rock_shift_east(
                        &next_state,
                        &boundaries,
                        &static_rocks,
                    )
                };
                let s =
                    grid_to_string(&old_map, &boundaries);
                iteration_maps.push(s.clone());
                cache.insert(s, next_state.clone());
                next_state
            }
        };

        // print_grid(&next_state, &boundaries);
        old_map = next_state;
    }

    let last_seen_grid_string =
        grid_to_string(&old_map, &boundaries);
    let mut it = iteration_maps.iter();
    let cycle_start_index = it
        .position(|grid_string| {
            grid_string == &last_seen_grid_string
        })
        .expect("should be a loop");

    // dbg!(cycle_start_index);
    let map_scores: Vec<i32> = iteration_maps
        .iter()
        .map(|v| cache.get(v).unwrap())
        .map(|map| {
            map.iter()
                .filter_map(|(position, rock)| match rock {
                    Rock::Movable => {
                        Some(boundaries.y - position.y)
                    }
                    Rock::Immovable => None,
                })
                .sum::<i32>()
        })
        .collect();
    let loop_size =
        iteration_maps.len() - cycle_start_index;
    // dbg!(&map_scores);
    let leftover_cycles =
        (1_000_000_000 - cycle_start_index) % loop_size;
    // dbg!(
    //     leftover_cycles,
    //     cycle_start_index,
    //     leftover_cycles + cycle_start_index
    // );
    // dbg!(
    //     map_scores[leftover_cycles + cycle_start_index - 1]
    // );

    Ok(
        map_scores[leftover_cycles + cycle_start_index - 1]
            .to_string(),
    )
}

fn print_grid(
    map: &HashMap<IVec2, Rock>,
    boundaries: &IVec2,
) {
    for y in 0..boundaries.y {
        for x in 0..boundaries.x {
            match map.get(&IVec2::new(x, y)) {
                Some(rock) => {
                    print!("{rock}");
                }
                None => print!("."),
            }
        }
        println!();
    }
}

fn grid_to_string(
    map: &HashMap<IVec2, Rock>,
    boundaries: &IVec2,
) -> String {
    (0..boundaries.y)
        .flat_map(|y| {
            (0..boundaries.x).map(move |x| {
                match map.get(&IVec2::new(x, y)) {
                    Some(Rock::Immovable) => "#",
                    Some(Rock::Movable) => "O",
                    None => ".",
                }
            })
        })
        .collect::<String>()
}

fn rock_shift_north(
    rock_map: &HashMap<IVec2, Rock>,
    boundaries: &IVec2,
    static_rocks: &HashMap<IVec2, Rock>,
) -> HashMap<IVec2, Rock> {
    let mut results = static_rocks.clone();
    // dbg!(results);
    let mut next_potentially_available_position =
        IVec2::new(0, 0);
    for x in 0..boundaries.x {
        next_potentially_available_position =
            IVec2::new(x, 0);
        for y in 0..boundaries.y {
            match rock_map.get(&IVec2::new(x, y)) {
                Some(Rock::Immovable) => {
                    next_potentially_available_position =
                        IVec2::new(x, y + 1);
                }
                Some(Rock::Movable) => {
                    let next_pos =
                        next_potentially_available_position;
                    results.insert(next_pos, Rock::Movable);

                    next_potentially_available_position
                        .y += 1;
                }
                None => {}
            }
        }
    }
    results
}

fn rock_shift_west(
    rock_map: &HashMap<IVec2, Rock>,
    boundaries: &IVec2,
    static_rocks: &HashMap<IVec2, Rock>,
) -> HashMap<IVec2, Rock> {
    let mut results = static_rocks.clone();
    // dbg!(results);
    let mut next_potentially_available_position =
        IVec2::new(0, 0);
    for y in 0..boundaries.y {
        next_potentially_available_position =
            IVec2::new(0, y);
        for x in 0..boundaries.x {
            match rock_map.get(&IVec2::new(x, y)) {
                Some(Rock::Immovable) => {
                    next_potentially_available_position =
                        IVec2::new(x + 1, y);
                }
                Some(Rock::Movable) => {
                    let next_pos =
                        next_potentially_available_position;
                    results.insert(next_pos, Rock::Movable);

                    next_potentially_available_position
                        .x += 1;
                }
                None => {}
            }
        }
    }
    results
}

fn rock_shift_south(
    rock_map: &HashMap<IVec2, Rock>,
    boundaries: &IVec2,
    static_rocks: &HashMap<IVec2, Rock>,
) -> HashMap<IVec2, Rock> {
    let mut results = static_rocks.clone();
    // dbg!(results);
    let mut next_potentially_available_position =
        IVec2::new(0, boundaries.y - 1);
    for x in 0..boundaries.x {
        next_potentially_available_position =
            IVec2::new(x, boundaries.y - 1);
        for y in (0..(boundaries.y)).rev() {
            match rock_map.get(&IVec2::new(x, y)) {
                Some(Rock::Immovable) => {
                    next_potentially_available_position =
                        IVec2::new(x, y - 1);
                }
                Some(Rock::Movable) => {
                    let next_pos =
                        next_potentially_available_position;
                    results.insert(next_pos, Rock::Movable);

                    next_potentially_available_position
                        .y -= 1;
                }
                None => {}
            }
        }
    }
    results
}

fn rock_shift_east(
    rock_map: &HashMap<IVec2, Rock>,
    boundaries: &IVec2,
    static_rocks: &HashMap<IVec2, Rock>,
) -> HashMap<IVec2, Rock> {
    let mut results = static_rocks.clone();
    // dbg!(results);
    let mut next_potentially_available_position =
        IVec2::new(boundaries.x - 1, 0);
    for y in 0..boundaries.y {
        next_potentially_available_position =
            IVec2::new(boundaries.x - 1, y);
        for x in (0..boundaries.x).rev() {
            match rock_map.get(&IVec2::new(x, y)) {
                Some(Rock::Immovable) => {
                    next_potentially_available_position =
                        IVec2::new(x - 1, y);
                }
                Some(Rock::Movable) => {
                    let next_pos =
                        next_potentially_available_position;
                    results.insert(next_pos, Rock::Movable);

                    next_potentially_available_position
                        .x -= 1;
                }
                None => {}
            }
        }
    }
    results
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!("64", process(input)?);
        Ok(())
    }
}
