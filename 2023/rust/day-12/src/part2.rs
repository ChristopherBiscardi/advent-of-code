use itertools::{repeat_n, Itertools};
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{self, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use tracing::info;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let puzzles = input
        .lines()
        .map(parse_line)
        .collect::<Result<
            Vec<(&str, Puzzle)>,
            nom::Err<nom::error::Error<&str>>,
        >>()
        .expect("parsing to succeed");
    let sum = puzzles
        .iter()
        .map(|(_, puzzle)| puzzle.possible_solution_count())
        .sum::<usize>();

    info!(sum);
    Ok(sum.to_string())
}

#[derive(Debug)]
struct Puzzle {
    spaces_to_fill: u32,
    line: String,
    batches: Vec<u32>,
}
fn parse_line(input: &str) -> IResult<&str, Puzzle> {
    let (input, (line, batches)) = separated_pair(
        is_a("?.#"),
        space1,
        separated_list1(tag(","), complete::u32),
    )(input)?;

    let expanded_line =
        std::iter::repeat(line).take(5).join("?");
    let spaces_to_fill =
        expanded_line.chars().filter(|c| c == &'?').count()
            as u32;

    Ok((
        input,
        Puzzle {
            spaces_to_fill,
            line: expanded_line,
            batches: std::iter::repeat(batches)
                .take(5)
                .flatten()
                .collect(),
        },
    ))
}

impl Puzzle {
    #[tracing::instrument]
    fn generate_permutations(
        &self,
    ) -> impl Iterator<Item = String> {
        repeat_n(
            [".", "#"].into_iter(),
            self.spaces_to_fill as usize,
        )
        .multi_cartesian_product()
        .map(|v| v.join(""))
    }
    #[tracing::instrument]
    fn check_option(&self, option: &str) -> bool {
        let mut option_iter = option.chars();
        let filled_option = self.line.chars().map(|c| match c {
            '?' => option_iter.next().expect("should have a length similar to needed gaps"),
            value => value
        }).collect::<String>();
        info!(filled_option);
        let counts = filled_option
            .chars()
            .group_by(|c| c == &'#')
            .into_iter()
            .filter_map(|(is_hashes, group)| {
                is_hashes.then_some(
                    group.into_iter().count() as u32,
                )
            })
            .collect::<Vec<u32>>();
        info!(?counts);
        &self.batches[..] == &counts[..]
    }
    fn possible_solution_count(&self) -> usize {
        let count = self
            .generate_permutations()
            .filter(|option| self.check_option(option))
            .count();
        count
    }
}
#[tracing::instrument(skip(input))]
fn process_line(input: &str) -> usize {
    let (_input, puzzle) = parse_line(input)
        .expect("should parse a valid line");
    let possible_solution_count =
        puzzle.possible_solution_count();

    info!(possible_solution_count);
    possible_solution_count
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 16384)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 16)]
    #[case("????.######..#####. 1,6,5", 2500)]
    #[case("?###???????? 3,2,1", 506250)]
    #[test_log::test]
    fn test_line(
        #[case] input: &str,
        #[case] output: usize,
    ) {
        assert_eq!(output, process_line(input));
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!("525152", process(input)?);
        Ok(())
    }
}
