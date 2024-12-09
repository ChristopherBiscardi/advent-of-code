#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // let num_files = input.len() / 2;
    let high_index: u32 = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum();

    let mut reverse = ((0..input.len())
        .rev()
        .zip(input.chars().rev()))
    .scan(
        high_index,
        |base_index, (compressed_index, c)| {
            let num_indices = c.to_digit(10).unwrap();
            *base_index -= num_indices;

            Some(
                (*base_index..(*base_index + num_indices))
                    .rev()
                    .filter_map(move |i| {
                        (compressed_index % 2 == 0)
                            .then_some((
                                i,
                                compressed_index / 2,
                            ))
                    }),
            )
        },
    )
    .flatten();

    let mut base_index = 0;
    let mut sum = 0;
    let mut last_uncompressed_index = u32::MAX;

    for (compressed_index, c) in input.chars().enumerate() {
        let num_indices = c.to_digit(10).unwrap() as usize;
        let file_id = compressed_index / 2;

        for uncompressed_index in
            base_index..(base_index + num_indices)
        {
            if uncompressed_index
                >= last_uncompressed_index as usize
            {
                break;
            }
            if compressed_index % 2 == 0 {
                sum += uncompressed_index * file_id;
            } else {
                let (rev_uncompressed_index, file_id) =
                    reverse.next().unwrap();
                sum += uncompressed_index * file_id;
                last_uncompressed_index =
                    rev_uncompressed_index;
            }
        }

        base_index += num_indices;
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
