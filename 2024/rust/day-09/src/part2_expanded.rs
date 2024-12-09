#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut expanded = input
        .chars()
        .enumerate()
        .map(|(compressed_index, num_indices)| {
            std::iter::repeat(if compressed_index % 2 == 0 {
                Some(compressed_index / 2)
            } else {
                None
            })
            .take(num_indices.to_digit(10).unwrap() as usize)
        })
        .flatten()
        .collect::<Vec<_>>();

    // print_blocks("original expanded blocks", &expanded);

    let mut high_index = expanded.len();

    loop {
        // get next chunk of file_id
        let Some(next_file_chunk_end_index) =
            expanded[0..high_index].iter().rposition(|v| v.is_some())
        else {
            panic!("shouldn't happen");
        };

        let Some(start_file_chunk_index) = expanded[0..next_file_chunk_end_index]
            .iter()
            .rposition(|v| v != &expanded[next_file_chunk_end_index])
            .map(|v| v + 1)
        else {
            break;
        };

        // length of the chunk of file_id
        let chunk_length = (start_file_chunk_index..=next_file_chunk_end_index).count();

        // find an empty that
        // is at least as big as the file chunk
        let Some(empty_slot) = expanded
            .windows(chunk_length)
            .position(|slice| slice.iter().all(|opt| opt.is_none()))
        else {
            high_index = start_file_chunk_index;
            continue;
        };

        // if empty is to the left of the file chunk index
        if empty_slot < start_file_chunk_index {
            // split mutable access to left/right of chunk index
            let (left, right) = expanded.split_at_mut(start_file_chunk_index);

            // copy chunk from right into left at empty location
            left[empty_slot..(empty_slot + chunk_length)].copy_from_slice(&right[..chunk_length]);

            // empty out original file chunk location
            for i in 0..chunk_length {
                right[i] = None;
            }
        }

        // high index must come down so we can find the next chunk
        high_index = start_file_chunk_index;
    }

    // print_blocks("final expanded blocks", &expanded);

    let sum: usize = expanded
        .iter()
        .enumerate()
        .filter_map(|(expanded_index, opt_file_id)| {
            opt_file_id.map(|file_id| expanded_index * file_id)
        })
        .sum();

    Ok(sum.to_string())
}

fn print_blocks(debug_id: &str, blocks: &[Option<usize>]) {
    println!("--- {}", debug_id);
    for block in blocks.iter() {
        if let Some(file_id) = block {
            print!("{}", file_id);
        } else {
            print!(".");
        }
    }
    println!("\n---");
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
