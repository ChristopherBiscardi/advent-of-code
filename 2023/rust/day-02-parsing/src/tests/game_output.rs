use std::sync::OnceLock;

use crate::game::{Game, Round};

pub const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

static CELL: OnceLock<Vec<Game>> = OnceLock::new();

pub fn output() -> &'static Vec<Game<'static>> {
    let games = vec![
        Game {
            id: "1",
            rounds: vec![
                Round {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                Round {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Round {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ],
        },
        Game {
            id: "2",
            rounds: vec![
                Round {
                    red: 0,
                    green: 2,
                    blue: 1,
                },
                Round {
                    red: 1,
                    green: 3,
                    blue: 4,
                },
                Round {
                    red: 0,
                    green: 1,
                    blue: 1,
                },
            ],
        },
        Game {
            id: "3",
            rounds: vec![
                Round {
                    red: 20,
                    green: 8,
                    blue: 6,
                },
                Round {
                    red: 4,
                    green: 13,
                    blue: 5,
                },
                Round {
                    red: 1,
                    green: 5,
                    blue: 0,
                },
            ],
        },
        Game {
            id: "4",
            rounds: vec![
                Round {
                    red: 3,
                    green: 1,
                    blue: 6,
                },
                Round {
                    red: 6,
                    green: 3,
                    blue: 0,
                },
                Round {
                    red: 14,
                    green: 3,
                    blue: 15,
                },
            ],
        },
        Game {
            id: "5",
            rounds: vec![
                Round {
                    red: 6,
                    green: 3,
                    blue: 1,
                },
                Round {
                    red: 1,
                    green: 2,
                    blue: 2,
                },
            ],
        },
    ];

    CELL.get_or_init(|| games)
}
