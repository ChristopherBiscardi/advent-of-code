use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    combinator::recognize,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // 5, 3x3 presents
    // let present_size = 7;
    let (_input, (presents, lines)) = parse(input).unwrap();

    let result = lines
        .iter()
        .filter_map(|((x, y), present_counts)| {
            (x * y
                > present_counts
                    .iter()
                    .enumerate()
                    .map(|(index, num_presents)| {
                        presents[index].1 as u32
                            * num_presents
                    })
                    .sum::<u32>())
            .then_some(())
        })
        .count();

    Ok(result.to_string())
}

fn parse(
    input: &str,
) -> IResult<
    &str,
    (
        Vec<(u32, usize)>,
        Vec<((u32, u32), Vec<u32>)>,
    ),
> {
    (
        present,
        separated_list1(
            line_ending,
            separated_pair(
                separated_pair(
                    complete::u32,
                    tag("x"),
                    complete::u32,
                ),
                (tag(":"), space1),
                separated_list1(space1, complete::u32),
            ),
        ),
    )
        .parse(input)
}

fn present(
    input: &str,
) -> IResult<&str, Vec<(u32, usize)>> {
    many1((
        terminated(complete::u32, (tag(":"), line_ending)),
        recognize(many1(alt((
            tag("#"),
            tag("."),
            line_ending,
        ))))
        .map(|val: &str| {
            val.chars().filter(|c| *c == '#').count()
        }),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
