use day_15::process_part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part1(&file, 2_000_000));
}
