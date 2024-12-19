use cached::proc_macro::cached;
use nom::{
    bytes::complete::tag,
    character::complete::{
        alpha1, line_ending, multispace1,
    },
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, (towels, designs)) = parse(input)
        .map_err(|e| {
            miette::miette!("parse failed {}", e)
        })?;

    let count: usize = designs
        .iter()
        .map(|design| validate_design(design, &towels))
        .sum();

    Ok(count.to_string())
}

#[cached(
    key = "String",
    convert = r##"{ format!("{design}") }"##
)]
fn validate_design(design: &str, towels: &[&str]) -> usize {
    return towels
        .iter()
        .filter_map(|towel| {
            if design.starts_with(*towel) {
                let new_design = &design[towel.len()..];
                if new_design.is_empty() {
                    return Some(1);
                }
                Some(validate_design(new_design, towels))
            } else {
                None
            }
        })
        .sum();
}

fn parse(
    input: &str,
) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    separated_pair(
        separated_list1(tag(", "), alpha1),
        multispace1,
        separated_list1(line_ending, alpha1),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!("16", process(input)?);
        Ok(())
    }
}
