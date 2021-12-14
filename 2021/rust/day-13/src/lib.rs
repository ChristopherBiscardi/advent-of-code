#![feature(iter_intersperse)]
use std::io::Write;
use std::{collections::BTreeMap, fs::File};

use ndarray::{s, Array2, Axis, Zip};
use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar, newline, u32},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::fs;

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
    Row(u32),
    Column(u32),
}
impl Fold {
    fn apply_to(
        &self,
        dots: &Array2<Mark>,
    ) -> Array2<Mark> {
        dbg!(&self);
        let (fold_axis, idx) = match self {
            Fold::Row(idx) => (Axis(0), idx),
            Fold::Column(idx) => (Axis(1), idx),
        };

        let folding_axis_length = dots.len_of(fold_axis);
        // dbg!(folding_axis_length);
        let skip_amount = folding_axis_length % 2;
        let (range_a, range_b) = (
            0..*idx as usize,
            (*idx as usize + skip_amount)
                ..folding_axis_length,
        );
        // dbg!(&range_a, &range_b);

        let matrix_a = dots.slice(match self {
            Fold::Row(_) => s!(range_a, ..),
            Fold::Column(_) => s!(.., range_a),
        });
        let mut matrix_b = dots.slice(match self {
            Fold::Row(_) => s!(range_b, ..),
            Fold::Column(_) => s!(.., range_b),
        });
        matrix_b.invert_axis(fold_axis);

        // union matrix a + b
        let smol = Zip::from(matrix_a)
            .and(matrix_b)
            .map_collect(|a, b| match (a, b) {
                (Mark::Marked, _) | (_, Mark::Marked) => {
                    Mark::Marked
                }
                (Mark::UnMarked, Mark::UnMarked) => {
                    Mark::UnMarked
                }
            });
        // dbg!(smol.shape());
        smol
    }
}
fn fold(input: &str) -> IResult<&str, Fold> {
    let (input, _) = tag("fold along ")(input)?;
    let (input, (axis, num)) =
        separated_pair(anychar, tag("="), u32)(input)?;
    Ok((
        input,
        match axis {
            'y' => Fold::Row(num),
            'x' => Fold::Column(num),
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
    let smol_matrix = operation.apply_to(&dots);
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

    let smol_matrix = folds.iter().enumerate().fold(
        dots,
        |dots, (i, operation)| {
            // dbg!(&operation);
            let smol = operation.apply_to(&dots);
            to_file(&smol, i);
            smol
        },
    );

    to_file(&smol_matrix, 999999999);
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

fn to_file(smol: &Array2<Mark>, step: usize) {
    let mut file =
        File::create(format!("step-{:02}.txt", step))
            .unwrap();
    let t = smol
        .rows()
        .into_iter()
        .map(|row| {
            format!(
                "{}",
                row.iter()
                    .map(|point| match point {
                        Mark::Marked => "█",
                        Mark::UnMarked => ".",
                    })
                    .collect::<String>()
            )
        })
        .intersperse("\n".to_string())
        .collect::<String>();
    file.write_all(t.as_bytes()).unwrap();
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
