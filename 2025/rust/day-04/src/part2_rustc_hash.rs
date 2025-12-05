use rustc_hash::FxHashSet;

use glam::IVec2;
use tracing::info;

const NEIGHBORS: [IVec2; 8] = [
    IVec2::X,
    IVec2::Y,
    IVec2::NEG_X,
    IVec2::NEG_Y,
    IVec2::ONE,
    IVec2::NEG_ONE,
    IVec2::new(1, -1),
    IVec2::new(-1, 1),
];

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let mut positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(
                move |(x, value)| {
                    (value == '@').then_some(IVec2::new(
                        x as i32, y as i32,
                    ))
                },
            )
        })
        .collect::<FxHashSet<IVec2>>();

    let mut removed_count = 0;
    loop {
        let rolls_to_remove: FxHashSet<IVec2> = positions
            .iter()
            .filter(|&position| {
                NEIGHBORS
                    .iter()
                    .filter(|&offset| {
                        positions
                            .contains(&(position + offset))
                    })
                    .count()
                    < 4
            })
            .cloned()
            .collect();
        if rolls_to_remove.len() == 0 {
            break;
        } else {
            removed_count += rolls_to_remove.len();
        }
        positions = positions
            .difference(&rolls_to_remove)
            .cloned()
            .collect();
    }

    Ok(removed_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("43", process(input)?);
        Ok(())
    }
}
