use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let groups = input.chars().group_by(|c| c != &',');
    let num = groups
        .into_iter()
        .filter(|t| t.0)
        .map(|(_, hash_group)| {
            hash_group.fold(0, |acc, next_char| {
                (acc + (next_char as usize)) * 17 % 256
            })
        })
        .sum::<usize>();
    Ok(num.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("1320", process(input)?);
        Ok(())
    }
}
