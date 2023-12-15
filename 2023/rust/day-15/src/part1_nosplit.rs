use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let initial: (usize, u8) = (0, 0);
    let num = input.chars().fold(initial, |mut acc, c| {
        match c {
            ',' => {
                acc.0 += acc.1 as usize;
                acc.1 = 0;
            }
            next_char => {
                acc.1 =
                    (acc.1.wrapping_add(next_char as u8))
                        .wrapping_mul(17);
            }
        }
        acc
    });
    Ok((num.0 + num.1 as usize).to_string())
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
