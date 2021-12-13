use std::collections::BTreeMap;

use ndarray::{s, Array2, Axis, Zip};
use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar, newline, u32},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone)]
enum Mark {
    Marked,
    UnMarked,
}
fn dots(input: &str) -> IResult<&str, Array2<Mark>> {
    let (input, outputs) = separated_list1(
        newline,
        separated_pair(u32, tag(","), u32),
    )(input)?;

    let max_x = outputs
        .iter()
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap()
        .0;
    let max_y = outputs
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .1;

    let nrows = max_y + 1;
    let ncols = max_x + 1;

    let data =
        vec![Mark::UnMarked; (nrows * ncols) as usize];

    let mut dots = Array2::from_shape_vec(
        (nrows as usize, ncols as usize),
        data,
    )
    .unwrap();
    for (x, y) in outputs.iter() {
        // dbg!(x, y);
        let point = dots
            .get_mut((*y as usize, *x as usize))
            .unwrap();
        // dbg!(point);
        *point = Mark::Marked;
    }

    Ok((input, dots))
}
#[derive(Debug)]
enum Fold {
    Y(u32),
    X(u32),
}
fn fold(input: &str) -> IResult<&str, Fold> {
    let (input, _) = tag("fold along ")(input)?;
    let (input, (axis, num)) =
        separated_pair(anychar, tag("="), u32)(input)?;
    Ok((
        input,
        match axis {
            'y' => Fold::Y(num),
            'x' => Fold::X(num),
            _ => panic!("not x or y for fold"),
        },
    ))
}
fn puzzle_input(
    input: &str,
) -> IResult<&str, (Array2<Mark>, Vec<Fold>)> {
    let (input, parsed_dots) = dots(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let (input, parsed_folds) =
        separated_list1(newline, fold)(input)?;

    Ok((input, (parsed_dots, parsed_folds)))
}
pub fn process_part1(input: &str) -> usize {
    let (_, (dots, folds)) = puzzle_input(input).unwrap();

    let mut it = folds.iter();
    let operation = it.next().unwrap();
    let smol_matrix = match operation {
        Fold::Y(row_idx) => {
            let axis_0_len = dots.len_of(Axis(0));
            let num_axis_0 = axis_0_len / 2;
            let matrix_a =
                dots.slice(s!(0..num_axis_0, ..));
            let mut matrix_b = dots.slice(s!(
                (*row_idx as usize + 1)..axis_0_len,
                ..,
            ));
            matrix_b.invert_axis(Axis(0));

            // union matrix a + b
            Zip::from(matrix_a).and(matrix_b).map_collect(
                |a, b| match (a, b) {
                    (Mark::Marked, _)
                    | (_, Mark::Marked) => Mark::Marked,
                    (Mark::UnMarked, Mark::UnMarked) => {
                        Mark::UnMarked
                    }
                },
            )
        }
        Fold::X(col_idx) => {
            let axis_1_len = dots.len_of(Axis(1));
            let num_axis_1 = axis_1_len / 2;
            let matrix_a =
                dots.slice(s!(.., 0..num_axis_1));
            let mut matrix_b = dots.slice(s!(
                ..,
                (*col_idx as usize + 1)..axis_1_len,
            ));
            matrix_b.invert_axis(Axis(1));

            // union matrix a + b
            Zip::from(matrix_a).and(matrix_b).map_collect(
                |a, b| match (a, b) {
                    (Mark::Marked, _)
                    | (_, Mark::Marked) => Mark::Marked,
                    (Mark::UnMarked, Mark::UnMarked) => {
                        Mark::UnMarked
                    }
                },
            )
        }
    };
    smol_matrix
        .iter()
        .map(|point| match point {
            Mark::Marked => 1,
            Mark::UnMarked => 0,
        })
        .sum()
}

pub fn process_part2(input: &str) -> usize {
    let (_, (dots, folds)) = puzzle_input(input).unwrap();

    let smol_matrix =
        folds.iter().fold(dots, |dots, operation| {
            match operation {
                Fold::Y(row_idx) => {
                    let axis_0_len = dots.len_of(Axis(0));
                    let num_axis_0 = axis_0_len / 2;
                    let skip_amount = axis_0_len % 2;

                    let matrix_a =
                        dots.slice(s!(0..num_axis_0, ..));
                    let mut matrix_b = dots.slice(s!(
                        (*row_idx as usize + skip_amount)
                            ..axis_0_len,
                        ..,
                    ));
                    matrix_b.invert_axis(Axis(0));

                    // union matrix a + b
                    let smol = Zip::from(matrix_a)
                        .and(matrix_b)
                        .map_collect(|a, b| match (a, b) {
                            (Mark::Marked, _)
                            | (_, Mark::Marked) => {
                                Mark::Marked
                            }
                            (
                                Mark::UnMarked,
                                Mark::UnMarked,
                            ) => Mark::UnMarked,
                        });
                    println!("-------------");
                    for row in smol.rows() {
                        println!(
                            "{}",
                            row.iter()
                                .map(|point| match point {
                                    Mark::Marked => "█",
                                    Mark::UnMarked => " ",
                                })
                                .collect::<String>()
                        );
                    }
                    smol
                }
                Fold::X(col_idx) => {
                    let axis_1_len = dots.len_of(Axis(1));
                    let num_axis_1 = axis_1_len / 2;
                    let skip_amount = axis_1_len % 2;

                    let matrix_a =
                        dots.slice(s!(.., 0..num_axis_1));
                    let mut matrix_b = dots.slice(s!(
                        ..,
                        (*col_idx as usize + skip_amount)
                            ..axis_1_len,
                    ));
                    matrix_b.invert_axis(Axis(1));

                    // union matrix a + b
                    let smol = Zip::from(matrix_a)
                        .and(matrix_b)
                        .map_collect(|a, b| match (a, b) {
                            (Mark::Marked, _)
                            | (_, Mark::Marked) => {
                                Mark::Marked
                            }
                            (
                                Mark::UnMarked,
                                Mark::UnMarked,
                            ) => Mark::UnMarked,
                        });
                    println!("-------------");
                    for row in smol.rows() {
                        println!(
                            "{}",
                            row.iter()
                                .map(|point| match point {
                                    Mark::Marked => "█",
                                    Mark::UnMarked => " ",
                                })
                                .collect::<String>()
                        );
                    }
                    smol
                }
            }
        });
    println!("-----FINAL-----");
    for row in smol_matrix.rows() {
        println!(
            "{}",
            row.iter()
                .map(|point| match point {
                    Mark::Marked => "█",
                    Mark::UnMarked => " ",
                })
                .collect::<String>()
        );
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
        include_str!("./test-input.txt");

    #[test]
    fn part1_test_demo_data() {
        assert_eq!(17, process_part1(INPUT));
    }
    // #[test]
    // fn part2_test_demo_data() {
    //     assert_eq!(36, process_part2(INPUT));
    // }
}
