// use itertools::Itertools;
use lazy_static::lazy_static;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, digit1},
    IResult,
};
use regex::Regex;
use std::io::{Error, ErrorKind};
struct PasswordLine<'a> {
    lower: u8,
    upper: u8,
    character: char,
    password: &'a str,
}

fn password_line(input: &str) -> IResult<&str, PasswordLine> {
    let (input, lower) = digit1(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, upper) = digit1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, parsed_character) = anychar(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, password) = alpha1(input)?;

    Ok((
        input,
        PasswordLine {
            lower: lower.parse::<u8>().unwrap(),
            upper: upper.parse::<u8>().unwrap(),
            character: parsed_character,
            password,
        },
    ))
}

pub fn process_part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| match process_password_line(line) {
            Ok(true) => Some(true),
            Ok(false) => None,
            Err(_) => None,
        })
        .count()
}

fn process_password_line(line: &str) -> Result<bool, std::io::Error> {
    let result = password_line(line).map(|(_, pass)| {
        let char_count = pass
            .password
            .chars()
            .filter(|c| pass.character == *c)
            .count();
        char_count >= pass.lower.into() && char_count <= pass.upper.into()
    });
    match result {
        Ok(b) => Ok(b),
        _ => Err(Error::new(ErrorKind::Other, "failed to parse")),
    }
}

pub fn process_part2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| match process2_password_line(line) {
            Ok(true) => Some(true),
            Ok(false) => None,
            Err(_) => None,
        })
        .count()
}

fn process2_password_line(line: &str) -> Result<bool, std::io::Error> {
    let result = password_line(line).map(|(_, pass)| {
        let pos_1 = pass
            .password
            .chars()
            .nth((pass.lower - 1).into())
            .map(|c| pass.character == c)
            .unwrap_or(false);
        let pos_2 = pass
            .password
            .chars()
            .nth((pass.upper - 1).into())
            .map(|c| pass.character == c)
            .unwrap_or(false);
        pos_1 != pos_2
    });
    match result {
        Ok(b) => Ok(b),
        _ => Err(Error::new(ErrorKind::Other, "failed to parse")),
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_input_one() {
        assert_eq!(
            process_part1(
                "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc",
            ),
            2
        )
    }
    #[test]
    fn test_input_line_a() {
        assert_eq!(process_password_line("1-3 a: abcde").unwrap(), true)
    }
    #[test]
    fn test_input_line_b() {
        assert_eq!(process_password_line("1-3 b: cdefg").unwrap(), false)
    }
    #[test]
    fn test_input_line_c() {
        assert_eq!(process_password_line("2-9 c: ccccccccc").unwrap(), true)
    }

    #[test]
    fn test_input_two() {
        assert_eq!(
            process_part2(
                "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc",
            ),
            1
        )
    }
    #[test]
    fn test2_input_line_a() {
        assert_eq!(process2_password_line("1-3 a: abcde").unwrap(), true)
    }
    #[test]
    fn test2_input_line_b() {
        assert_eq!(process2_password_line("1-3 b: cdefg").unwrap(), false)
    }
    #[test]
    fn test2_input_line_c() {
        assert_eq!(process2_password_line("2-9 c: ccccccccc").unwrap(), false)
    }
}
