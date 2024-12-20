fn main() {
    let all_reachable_path_positions =
        all_reachable_positions
            .iter()
            .filter_map(
                |((position, cell_type), (_, cost))| {
                    if cell_type == &Some(".") {
                        Some((position, cost))
                    } else {
                        None
                    }
                },
            )
            .map(
                |(
                    destination_path_position,
                    cheat_path_cost,
                )| {
                    let (path_position_cost, _) = first_run
                        .0
                        .iter()
                        .enumerate()
                        .find(|(_, path_pos)| {
                            destination_path_position
                                == *path_pos
                        })
                        .unwrap();
                    path_start_cost
                        + cheat_path_cost
                        + first_run.1
                        - path_position_cost
                },
            )
            .collect::<Vec<_>>();
}
