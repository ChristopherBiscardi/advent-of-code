use day_23::process_part2;
use std::fs;

fn main() {
    tracing_subscriber::fmt::init();

    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part2(&file));
}
