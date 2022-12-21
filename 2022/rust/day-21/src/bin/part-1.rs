use day_21::process_part1;
use std::fs;

fn main() {
    tracing_subscriber::fmt::init();

    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part1(&file));
}
