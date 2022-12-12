use itertools::Itertools; // 0.10.5

/// This is an example that showed up in the video
/// for day 12.
///
/// It shows an issue that occurs because we are
/// returning an iterator that references local
/// variables

fn main() {
    let grid: Vec<Vec<char>> =
        "abcdefghijklmnopqrstuvwxyzx"
            .chars()
            .chunks(3)
            .into_iter()
            .map(|v| v.collect())
            .collect();

    let edges = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .flat_map(|(y, x)| {
            let neighbors = vec![(x + 1, y), (x, y + 1)];
            let current_node_id = (x, y);
            neighbors.iter().filter_map(|cell| {
                grid.get(cell.1 as usize)
                    .and_then(|vec| {
                        vec.get(cell.0 as usize)
                    })
                    .and_then(|existing_cell| {
                        // if reachable
                        let current_node_height =
                            grid[y as usize][x as usize];

                        if current_node_height as u8 + 1
                            >= *existing_cell as u8
                        {
                            Some((
                                (
                                    current_node_id.0,
                                    current_node_id.1,
                                    current_node_height,
                                ),
                                (
                                    cell.0,
                                    cell.1,
                                    *existing_cell,
                                ),
                            ))
                        } else {
                            None
                        }
                    })
            })
        })
        .collect::<Vec<(
            (usize, usize, char),
            (usize, usize, char),
        )>>();

    for (a, b) in edges.iter() {
        println!(
            "({},{}): {} -> ({},{}): {}",
            a.0, a.1, a.2, b.0, b.1, b.2
        )
    }
}
