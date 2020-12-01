#![feature(try_find)]

use itertools::Itertools;
// use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};

fn process(input: &str) -> Result<u64, Error> {
    let result = input
        .lines()
        .map(|string_num| string_num.parse::<u64>().unwrap())
        .permutations(2)
        .map(|v| (v[0], v[1]))
        .try_find(|perm| {
            let matchable = match *perm {
                (one, two) => {
                    if one + two == 2020 {
                        return Ok(true);
                    } else {
                        return Ok(false);
                    }
                }
                _ => return Err(Error::new(ErrorKind::Other, "parsing failed I guess")),
            };
        });
    match result {
        Ok(Some((one, two))) => Ok(one * two),
        Err(err) => Err(err),
        Ok(None) => Err(Error::new(ErrorKind::Other, "ok-none")),
    }
}
fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::new(ErrorKind::Other, "An input file is required"));
    }
    let input = fs::read_to_string(&args[1])?;
    let result = process(&input);
    println!("{:?}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_input_one() {
        assert_eq!(
            process(
                "1721
979
366
299
675
1456",
            )
            .unwrap(),
            514579
        )
    }
}
