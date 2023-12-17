use itertools::Itertools;

use crate::custom_error::AocError;

#[derive(Debug, Eq, PartialEq)]
pub enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (horizontal, vertical) =
        input.split("\n\n").flat_map(detect_fold).fold(
            (0usize, 0usize),
            |mut acc, item| match item {
                Fold::Horizontal(num) => {
                    acc.0 += 100 * num;
                    acc
                }
                Fold::Vertical(num) => {
                    acc.1 += num;
                    acc
                }
            },
        );
    Ok((horizontal + vertical).to_string())
}

pub fn detect_fold(input: &str) -> Option<Fold> {
    detect_vertical_fold(input)
        .or(detect_horizontal_fold(input))
}
pub fn detect_vertical_fold(input: &str) -> Option<Fold> {
    let mut columns_iter_collection = input
        .lines()
        .map(|line| line.chars())
        .collect::<Vec<_>>();
    let columns = std::iter::from_fn(move || {
        let mut items = vec![];
        for iter in &mut columns_iter_collection {
            match iter.next() {
                Some(item) => {
                    items.push(item);
                }
                None => return None,
            }
        }
        Some(items)
    })
    .collect::<Vec<Vec<char>>>();

    let result = columns
        .iter()
        .enumerate()
        .tuple_windows()
        .filter(|((_, line_a), (_, line_b))| {
            line_a == line_b
        })
        .find_map(|((index_a, _), (index_b, _))| {
            let lines_a =
                columns[0..=index_a].iter().rev();
            let lines_b = columns[index_b..].iter();

            lines_a
                .zip(lines_b)
                .inspect(|_v| {
                    // dbg!(v);
                })
                .all(|(a, b)| a == b)
                .then_some(index_a + 1)
        });
    result.map(Fold::Vertical)
}
pub fn detect_horizontal_fold(input: &str) -> Option<Fold> {
    let lines: Vec<&str> = input.lines().collect();
    let result = input
        .lines()
        .enumerate()
        .tuple_windows()
        .filter(|((_, line_a), (_, line_b))| {
            line_a == line_b
        })
        .find_map(|((index_a, _), (index_b, _))| {
            let lines_a =
                lines[0..=index_a].iter().rev();
            let lines_b = lines[index_b..].iter();

            lines_a
                .zip(lines_b)
                .inspect(|_v| {
                    // dbg!(v);
                })
                .all(|(a, b)| a == b)
                .then_some(index_a + 1)
        });
    result.map(Fold::Horizontal)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        Fold::Vertical(5)
    )]
    #[case(
        "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        Fold::Horizontal(4)
    )]
    fn test_vert_horizontal(
        #[case] input: &str,
        #[case] expected: Fold,
    ) -> miette::Result<()> {
        assert_eq!(expected, detect_fold(input).unwrap());
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!("405", process(input)?);
        Ok(())
    }
}
