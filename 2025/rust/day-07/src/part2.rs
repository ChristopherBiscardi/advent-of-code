use rustc_hash::FxHashMap;

use tracing::info;

type Count = usize;
type Position = usize;

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
    let map = line_iter.fold(
        {
            let mut m =
                FxHashMap::<Position, Count>::default();
            m.insert(s_position, 1);
            m
        },
        |positions, (index, line)| {
            let mut new_positions =
                FxHashMap::<Position, Count>::default();
            for (index, count) in positions {
                if line.as_bytes()[index] == '^' as u8 {
                    new_positions
                        .entry(index - 1)
                        .and_modify(|value| {
                            *value += count;
                        })
                        .or_insert(count);
                    new_positions
                        .entry(index + 1)
                        .and_modify(|value| {
                            *value += count;
                        })
                        .or_insert(count);
                } else {
                    new_positions
                        .entry(index)
                        .and_modify(|value| {
                            *value += count;
                        })
                        .or_insert(count);
                }
            }

            new_positions
        },
    );
    Ok(map.values().sum::<usize>().to_string())
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
        assert_eq!("40", process(input)?);
        Ok(())
    }
}
