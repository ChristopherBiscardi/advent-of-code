use itertools::Itertools;
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};

fn process(input: &str) -> Option<u64> {
    input
        .lines()
        .map(|string_num| string_num.parse::<u64>().unwrap())
        .combinations(3)
        .find(|perm| perm.iter().sum::<u64>() == 2020)
        .map(|v| v.iter().product())
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
            241861950
        )
    }
}
