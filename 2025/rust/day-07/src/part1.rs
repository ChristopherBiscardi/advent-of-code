use rustc_hash::FxHashSet;

use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let mut line_iter = input.lines().enumerate();
    let s_position = line_iter
        .next()
        .unwrap()
        .1
        .chars()
        .position(|val| val == 'S')
        .unwrap();
    let (_, set): (
        FxHashSet<usize>,
        FxHashSet<(usize, usize)>,
    ) = line_iter.fold(
        (
            {
                let mut set = FxHashSet::default();
                set.insert(s_position);
                set
            },
            FxHashSet::default(),
        ),
        |(positions, mut splitters), (y_index, line)| {
            let mut new_positions =
                FxHashSet::<usize>::default();
            for index in positions {
                if line.as_bytes()[index] == '^' as u8 {
                    info!(?index, "split at");
                    new_positions.insert(index - 1);
                    new_positions.insert(index + 1);
                    splitters.insert((index, y_index));
                } else {
                    new_positions.insert(index);
                }
            }

            (new_positions, splitters)
        },
    );
    Ok(set.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!("21", process(input)?);
        Ok(())
    }
}
