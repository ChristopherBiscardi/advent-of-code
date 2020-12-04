#![feature(bool_to_option)]

use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{digit1, hex_digit1, multispace0, multispace1},
    multi::fold_many_m_n,
    IResult,
};

use std::io::{Error, ErrorKind};

pub fn process_part1(input: &str) -> usize {
    input
        .lines()
        .group_by(|line| line.len() > 0)
        .into_iter()
        .filter_map(|(key, passport_group)| {
            let mut raw_passport = passport_group.collect::<String>();
            if raw_passport.len() == 0 {
                return None;
            }
            (raw_passport.contains("byr:")
                && raw_passport.contains("iyr:")
                && raw_passport.contains("eyr:")
                && raw_passport.contains("hgt:")
                && raw_passport.contains("hcl:")
                && raw_passport.contains("ecl:")
                && raw_passport.contains("pid:"))
            .then_some(true)
        })
        .count()
}

struct Passport<'a> {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: Height,
    // done
    hcl: String,
    ecl: EYE_COLOR,
    pid: &'a str, // cid: Option<>,
}
#[derive(Debug, Eq, PartialEq)]
enum PassportParse<'a> {
    BYR(usize),
    IYR(usize),
    EYR(usize),
    HGT(Height),
    HCL(String),
    ECL(EYE_COLOR),
    PID(&'a str),
    CID(()),
}
use PassportParse::*;
fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}
fn hex_digit(input: &str) -> IResult<&str, &str> {
    let (input, color) = take_while_m_n(2, 2, is_hex_digit)(input)?;
    Ok((input, color))
}
fn hcl(input: &str) -> IResult<&str, PassportParse> {
    let (input, _) = tag("hcl:")(input)?;
    let (input, _) = tag("#")(input)?;
    let (input, color) = fold_many_m_n(3, 3, hex_digit, "".to_string(), |mut s: String, item| {
        s + item
    })(input)?;

    Ok((input, HCL(color)))
}
fn year<'a>(
    prefix: &'a str,
    lower: usize,
    upper: usize,
    input: &'a str,
) -> IResult<&'a str, usize> {
    let (input, _) = tag(prefix)(input)?;
    let (input, year) = digit1(input)?;
    match year.parse::<usize>() {
        Ok(digits) => {
            if digits >= lower && digits <= upper {
                Ok((input, digits))
            } else {
                Err(nom::Err::Error(nom::error::Error {
                    input,
                    code: nom::error::ErrorKind::Digit,
                }))
            }
        }
        _ => Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Digit,
        })),
    }
}
fn byr(input: &str) -> IResult<&str, PassportParse> {
    year("byr:", 1920, 2002, input).map(|(i, r)| (i, BYR(r)))
}
fn iyr(input: &str) -> IResult<&str, PassportParse> {
    year("iyr:", 2010, 2020, input).map(|(i, r)| (i, IYR(r)))
}
fn eyr(input: &str) -> IResult<&str, PassportParse> {
    year("eyr:", 2020, 2030, input).map(|(i, r)| (i, EYR(r)))
}
fn cid(input: &str) -> IResult<&str, PassportParse> {
    let (input, _) = tag("cid:")(input)?;
    let (input, _) = digit1(input)?;
    Ok((input, CID(())))
}
fn pid(input: &str) -> IResult<&str, PassportParse> {
    let (input, _) = tag("pid:")(input)?;
    let (input, year) = digit1(input)?;
    if year.len() == 9 {
        Ok((input, PID(year)))
    } else {
        Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Digit,
        }))
    }
}
#[derive(Debug, Eq, PartialEq)]
enum EYE_COLOR {
    AMB,
    BLU,
    BRN,
    GRY,
    GRN,
    HZL,
    OTH,
}
fn ecl(input: &str) -> IResult<&str, PassportParse> {
    let (input, _) = tag("ecl:")(input)?;
    let (input, eye) = nom::branch::alt((
        tag("amb"),
        tag("blu"),
        tag("brn"),
        tag("gry"),
        tag("grn"),
        tag("hzl"),
        tag("oth"),
    ))(input)?;
    let eye_color = match eye {
        "amb" => EYE_COLOR::AMB,
        "blu" => EYE_COLOR::BLU,
        "brn" => EYE_COLOR::BRN,
        "gry" => EYE_COLOR::GRY,
        "grn" => EYE_COLOR::GRN,
        "hzl" => EYE_COLOR::HZL,
        "oth" => EYE_COLOR::OTH,
        _ => panic!("EYE COLOR PANIC! AT THE DISCO"),
    };
    Ok((input, ECL(eye_color)))
}

#[derive(Debug, Eq, PartialEq)]
enum Height {
    Inches(usize),
    Centimeters(usize),
}
fn hgt(input: &str) -> IResult<&str, PassportParse> {
    let (input, _) = tag("hgt:")(input)?;
    let (input, h) = digit1(input)?;
    let (input, measurement) = nom::branch::alt((tag("in"), tag("cm")))(input)?;

    match h.parse::<usize>() {
        Ok(digits) => {
            let height = match (measurement) {
                "in" => {
                    if digits >= 59 && digits <= 76 {
                        Ok((input, HGT(Height::Inches(digits))))
                    } else {
                        Err(nom::Err::Error(nom::error::Error {
                            input,
                            code: nom::error::ErrorKind::Digit,
                        }))
                    }
                }
                "cm" => {
                    if digits >= 150 && digits <= 193 {
                        Ok((input, HGT(Height::Centimeters(digits))))
                    } else {
                        Err(nom::Err::Error(nom::error::Error {
                            input,
                            code: nom::error::ErrorKind::Digit,
                        }))
                    }
                }
                _ => Err(nom::Err::Error(nom::error::Error {
                    input,
                    code: nom::error::ErrorKind::Digit,
                })),
            };
            height
        }
        _ => Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Digit,
        })),
    }
    // Ok((input, result))
}
fn all<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    nom::sequence::delimited(multispace0, inner, multispace0)
}
fn any(input: &str) -> IResult<&str, PassportParse> {
    nom::branch::alt((byr, iyr, eyr, hgt, hcl, ecl, pid, cid))(input)
}
fn passport(input: &str) -> IResult<&str, String> {
    // many1(alt(byr, iyr, ...))
    //     println!(
    //         "input:
    // {}",
    //         input
    //     );
    let (input, passport_values) = nom::multi::separated_list1(multispace1, any)(input)?;
    //     println!(
    //         "
    // leftover:
    // {}

    // {:?}",
    //         input, passport_values
    //     );
    //     println!(
    //         "
    // ---
    // "
    //     );
    if passport_values
        .iter()
        .filter(|c| match c {
            CID(()) => false,
            s => true,
        })
        .count()
        == 7
    {
        Ok((input, "".to_string()))
    } else {
        Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Digit,
        }))
    }
}
pub fn process_part2(input: &str) -> usize {
    input.split("\n\n").flat_map(|v| passport(v).ok()).count()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_input_one() {
        assert_eq!(
            process_part1(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in",
            ),
            2
        )
    }
    #[test]
    fn text_hcl() {
        assert_eq!(hcl("hcl:#1fa9f4").unwrap(), ("", HCL("1fa9f4".to_string())));
    }
    #[test]
    fn test_input_two_single() {
        assert_eq!(
            passport(
                "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f"
            ),
            Ok(("", "".to_string()))
        )
    }
    #[test]
    fn test_input_two_valid() {
        assert_eq!(
            process_part2(
                "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
            ),
            4
        )
    }
    #[test]
    fn test_input_invalid() {
        assert_eq!(
            process_part2(
                "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007",
            ),
            0
        )
    }
}
