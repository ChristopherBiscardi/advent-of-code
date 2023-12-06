use std::collections::{BTreeMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, digit1, line_ending, space0, space1,
    },
    combinator::{eof, iterator},
    multi::fold_many1,
    sequence::{
        delimited, separated_pair, terminated, tuple,
    },
    IResult, Parser,
};
use tracing::{info, span, Level};

use crate::custom_error::AocError;

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl Card {
    #[allow(dead_code)]
    #[tracing::instrument]
    fn score(&self) -> u32 {
        match self.num_matches().checked_sub(1) {
            Some(num) => 2u32.pow(num as u32),
            None => 0,
        }
    }
    #[tracing::instrument]
    fn num_matches(&self) -> usize {
        self.winning_numbers
            .intersection(&self.my_numbers)
            .count()
    }
}

#[tracing::instrument]
fn set(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        HashSet::new,
        |mut acc: HashSet<_>, item| {
            acc.insert(item);
            acc
        },
    )(input)
}

#[tracing::instrument]
fn card(input: &str) -> IResult<&str, Card> {
    let (input, _) = delimited(
        tuple((tag("Card"), space1)),
        digit1,
        tuple((tag(":"), space1)),
    )(input)?;
    separated_pair(set, tuple((tag("|"), space1)), set)
        .map(|(winning_numbers, my_numbers)| Card {
            winning_numbers,
            my_numbers,
        })
        .parse(input)
}

#[tracing::instrument(skip(input))]
fn cards(input: &str) -> IResult<&str, u32> {
    let mut it = iterator(
        input,
        terminated(card, alt((line_ending, eof))),
    );

    let result = it.enumerate().fold(
        (0, BTreeMap::<usize, u32>::new()),
        |(mut sum, mut acc), (index, card)| {
            let my_span = span!(
                Level::INFO,
                "fold_span",
                card = index + 1,
                num_matches = card.num_matches(),
                // duplicates = acc.get(&index)
            );
            my_span.in_scope(|| {
                info!(?acc);
                let duplicates = match acc.get(&index) {
                    Some(num) => num + 1,
                    None => 1,
                };
                info!(duplicates);
                sum += duplicates;

                for i in (index + 1)
                    ..(index + 1 + card.num_matches())
                {
                    acc.entry(i)
                        .and_modify(|value| {
                            *value += duplicates;
                        })
                        .or_insert(duplicates);
                }
                (sum, acc)
            })
        },
    );

    let res: IResult<&str, ()> = it.finish();

    info!(result = ?result.1);
    res.map(|(input, _)| (input, result.0))
}

#[tracing::instrument(skip(input), fields(short_input = &input[0..20]))]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (_, card_data) =
        cards(input).expect("a valid parse");

    Ok(card_data.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        8
    )]
    #[case(
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        2
    )]
    #[case(
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        2
    )]
    #[case(
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        1
    )]
    #[case(
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        0
    )]
    #[case(
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        0
    )]
    fn line_test(
        #[case] line: &str,
        #[case] expected: u32,
    ) {
        let (input, card) =
            card(line).expect("should be a valid card");
        assert_eq!(input, "");
        assert_eq!(expected, card.score())
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", process(input)?);
        Ok(())
    }
}
