use day_03::process_part1;
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::new(ErrorKind::Other, "An input file is required"));
    }
    let input = fs::read_to_string(&args[1])?;
    let result = process_part1(&input);
    println!("{:?}", result);
    Ok(())
}
