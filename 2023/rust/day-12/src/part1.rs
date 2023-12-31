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
            Vec<(&str, Puzzle<'_>)>,
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
struct Puzzle<'a> {
    spaces_to_fill: u32,
    line: &'a str,
    batches: Vec<u32>,
}
fn parse_line(input: &str) -> IResult<&str, Puzzle> {
    let (input, (line, batches)) = separated_pair(
        is_a("?.#"),
        space1,
        separated_list1(tag(","), complete::u32),
    )(input)?;

    let spaces_to_fill =
        line.chars().filter(|c| c == &'?').count() as u32;

    Ok((
        input,
        Puzzle {
            spaces_to_fill,
            line,
            batches,
        },
    ))
}

impl<'a> Puzzle<'a> {
    #[tracing::instrument]
    fn generate_permutations(&self) -> Vec<String> {
        let options: Vec<String> = repeat_n(
            [".", "#"].into_iter(),
            self.spaces_to_fill as usize,
        )
        .multi_cartesian_product()
        .map(|v| v.join(""))
        .collect();

        options
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
            .filter(|(is_hashes, _)| *is_hashes)
            .map(|(_, group)| {
                group.into_iter().count() as u32
            })
            .collect::<Vec<u32>>();
        info!(?counts);
        self.batches == counts
    }
    fn possible_solution_count(&self) -> usize {
        let options = self.generate_permutations();
        let count = options
            .iter()
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
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
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
        assert_eq!("21", process(input)?);
        Ok(())
    }
}
