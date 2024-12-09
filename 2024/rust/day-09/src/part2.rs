#[derive(Debug)]
struct Chunk {
    uncompressed_index: usize,
    count: usize,
    file_id: usize,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // let num_files = input.len() / 2;
    let high_index: usize = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .sum();

    let uncompressed_reversed =
        (0..input.len()).rev().zip(input.chars().rev());
    let reverse = uncompressed_reversed
        .scan(
            high_index,
            |base_index, (compressed_index, c)| {
                let num_indices =
                    c.to_digit(10).unwrap() as usize;
                *base_index -= num_indices;

                if compressed_index % 2 == 0 {
                    Some(Some(Chunk {
                        uncompressed_index: *base_index,
                        count: num_indices,
                        file_id: compressed_index as usize
                            / 2,
                    }))
                } else {
                    Some(None)
                }
            },
        )
        .flatten();
    // .filter_map(|v| v);

    // (uncompressed_index, space_count)
    let mut empties = input
        .chars()
        .enumerate()
        .fold(
            (0, vec![]),
            |(mut uncompressed_index, mut empties),
             (compressed_index, c)| {
                let num_indices =
                    c.to_digit(10).unwrap() as usize;
                if compressed_index % 2 != 0 {
                    empties.push((
                        uncompressed_index,
                        num_indices,
                    ))
                }
                uncompressed_index += num_indices;
                (uncompressed_index, empties)
            },
        )
        .1;

    // let mut moved_ids: Vec<usize> = vec![];
    let mut moved_chunks: Vec<Chunk> = vec![];
    for chunk in reverse {
        let Some(empty) =
            empties.iter_mut().find(|(i, empty_space)| {
                chunk.count <= (*empty_space as usize)
                    && *i < chunk.uncompressed_index
            })
        else {
            continue;
        };

        // moved_ids.push(chunk.file_id);
        moved_chunks.push(Chunk {
            uncompressed_index: empty.0 as usize,
            ..chunk
        });
        empty.0 += chunk.count;
        empty.1 -= chunk.count;
    }

    let mut base_index = 0;
    let mut sum = 0;
    // let mut last_uncompressed_index = usize::MAX;

    for (compressed_index, c) in input.chars().enumerate() {
        let num_indices = c.to_digit(10).unwrap() as usize;
        let file_id = compressed_index / 2;

        for uncompressed_index in
            base_index..(base_index + num_indices)
        {
            if compressed_index % 2 == 0
                && !moved_chunks.iter().any(|chunk| {
                    chunk.file_id == file_id as usize
                })
            {
                sum += uncompressed_index * file_id;
            }
        }

        base_index += num_indices;
    }

    for chunk in moved_chunks.iter() {
        for index in chunk.uncompressed_index
            ..(chunk.uncompressed_index + chunk.count)
        {
            sum += index * chunk.file_id;
        }
        // sum +=    chunk.
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input)?);
        Ok(())
    }
}
