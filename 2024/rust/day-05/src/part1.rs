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

    let results: Vec<usize> = updates
        .iter()
        .enumerate()
        .filter_map(|(index, original_update)| {
            let mut current_item = original_update[0];
            let mut update = &original_update[1..];
            let mut before_pages = &original_update[0..0];

            while before_pages.len()
                != original_update.len()
            {
                if let Some(pages_that_must_come_after) =
                    rules.get(&current_item)
                {
                    if !pages_that_must_come_after
                        .iter()
                        .all(|page| {
                            !before_pages.contains(page)
                        })
                    {
                        return None;
                    }
                }
                // next iteration
                before_pages = &original_update
                    [0..(before_pages.len() + 1)];

                if let Some(page) = update.get(0) {
                    current_item = *page;
                    update = &update[1..];
                }
            }

            Some(index)
        })
        .collect();

    let result: u32 = results
        .iter()
        .map(|index| {
            let middle = updates[*index].len() / 2;
            updates[*index][middle]
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
