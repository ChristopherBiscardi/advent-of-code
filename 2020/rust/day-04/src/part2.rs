use day_04::process_part2;
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};

fn main() -> Result<(), std::io::Error> {
    //     Right 1, down 1.
    // Right 3, down 1. (This is the slope you already checked.)
    // Right 5, down 1.
    // Right 7, down 1.
    // Right 1, down 2.

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::new(ErrorKind::Other, "An input file is required"));
    }
    let input = fs::read_to_string(&args[1])?;
    let result = process_part2(&input);
    println!("{:?}", result);
    Ok(())
}
