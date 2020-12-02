use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

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
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([\d]+)-([\d]+) ([a-z]): ([a-z]+)").unwrap();
    }
    let caps = RE.captures(line).unwrap();
    let lower_bound = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let upper_bound = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let character = caps.get(3).unwrap().as_str();
    let password = caps.get(4).unwrap().as_str();
    let char_count = password.chars().filter(|c| character.contains(*c)).count();

    if char_count < lower_bound || char_count > upper_bound {
        Ok(false)
    } else {
        Ok(true)
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
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([\d]+)-([\d]+) ([a-z]): ([a-z]+)").unwrap();
    }
    let caps = RE.captures(line).unwrap();
    let lower_bound = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let upper_bound = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let character = caps.get(3).unwrap().as_str();
    let password = caps.get(4).unwrap().as_str();

    let pos_1 = password
        .chars()
        .nth(lower_bound - 1)
        .map(|c| character.contains(c))
        .unwrap_or(false);
    let pos_2 = password
        .chars()
        .nth(upper_bound - 1)
        .map(|c| character.contains(c))
        .unwrap_or(false);
    if pos_1 && pos_2 || !pos_1 && !pos_2 {
        Ok(false)
    } else {
        Ok(true)
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
