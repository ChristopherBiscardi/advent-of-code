use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, (rules, updates)) = parse(input)
        .map_err(|e| miette!("parse failed {}", e))?;

    let result: u32 = updates
        .iter()
        .filter(|update| {
            update.is_sorted_by(|a, b| {
                rules
                    .get(a)
                    .is_some_and(|pages| pages.contains(b))
            })
        })
        .map(|update| {
            let middle = update.len() / 2;
            update[middle]
        })
        .sum();

    Ok(result.to_string())
}

fn rules(
    input: &str,
) -> IResult<&str, HashMap<u32, Vec<u32>>> {
    fold_many1(
        terminated(
            separated_pair(
                complete::u32,
                tag("|"),
                complete::u32,
            ),
            line_ending,
        ),
        HashMap::default,
        |mut acc: HashMap<u32, Vec<u32>>, (page, after)| {
            acc.entry(page)
                .and_modify(|afters| {
                    afters.push(after);
                })
                .or_insert(vec![after]);
            acc
        },
    )(input)
}

fn updates(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(
        line_ending,
        separated_list1(tag(","), complete::u32),
    )(input)
}

fn parse(
    input: &str,
) -> IResult<&str, (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>)>
{
    let (input, parsed_rules) =
        terminated(rules, line_ending)(input)?;
    let (input, parsed_updates) = updates(input)?;
    Ok((input, (parsed_rules, parsed_updates)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("143", process(input)?);
        Ok(())
    }
}
