use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::streaming::take_until,
    character::streaming::{
        alpha1, anychar, char, digit1, newline, one_of, u16,
    },
    combinator::opt,
    error::context,
    multi::{many0, many1, separated_list1},
    IResult,
};
use nom_supreme::error::StackContext::Context;
use nom_supreme::tag::streaming::tag;
use nom_supreme::{
    error::ErrorTree, final_parser::final_parser,
};

// ): 3 points.
// ]: 57 points.
// }: 1197 points.
// >: 25137 points.

fn chunk(
    original_input: &str,
) -> IResult<&str, (), ErrorTree<&str>> {
    let (input, open_char) =
        one_of("({<[")(original_input)?;
    let c_res: IResult<&str, &str> = tag(match open_char {
        '{' => "}",
        '(' => ")",
        '[' => "]",
        '<' => ">",
        _ => panic!("unrecognized char"),
    })(input);
    if let Ok((input, _)) = c_res {
        Ok((input, ()))
    } else {
        let (input, _chunks) =
            context("chunk", many1(chunk))(input)?;
        let mut input = input;
        loop {
            match input.chars().next() {
                Some('{') | Some('(') | Some('[')
                | Some('<') => chunk(input)?,
                // we aren't setting input again
                _ => {
                    break;
                }
            };
        }
        let (input, _) = context(
            "chars",
            tag(match open_char {
                '{' => "}",
                '(' => ")",
                '[' => "]",
                '<' => ">",
                _ => panic!("unrecognized char"),
            }),
        )(input)?;
        Ok((input, ()))
    }
}

pub fn process_part1(input: &str) -> u32 {
    let scoring: HashMap<char, u32> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);
    let results: u32 = input
        .lines()
        .map(|line| chunk(line))
        .enumerate()
        // filter out lines that end early
        .filter_map(|(i, res)| {
            // dbg!(&res);
            // match res {
            //     Ok(_) => None,
            //     Err(e) => {
            //         dbg!(e);
            //         Some(0)
            //     }
            // }
            match res {
                Ok(_) => None,
                Err(nom::Err::Incomplete(e)) => None,
                Err(nom::Err::Error(
                    ErrorTree::Stack { base, contexts },
                )) => {
                    // dbg!(&base, &contexts);
                    let ctx = contexts
                        .iter()
                        .find(|v| v.1 == Context("chars"))
                        .unwrap();
                    // dbg!(ctx);
                    let c = ctx.0.chars().next().unwrap();
                    let res = scoring.get(&c);
                    // dbg!(c);
                    res
                }

                _ => panic!("uh oh"),
            }
        })
        // .inspect(|v| {
        //     dbg!(v);
        // })
        .sum();
    results
}

fn puzzle_input_2(input: &str) -> IResult<&str, ()> {
    // let (input, outputs) =
    //     separated_list1(newline, row_2)(input)?;
    todo!("part 2")
}

pub fn process_part2(input: &str) -> usize {
    let (_, heightmap) = puzzle_input_2(input).unwrap();
    todo!("part2")
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &'static str =
        include_str!("./test-input.txt");

    #[test]
    fn part1_test_demo_data() {
        assert_eq!(26397, process_part1(input));
    }
    #[test]
    fn parses_parens() {
        match chunk("((()))") {
            Ok((i, _)) => {
                dbg!(i);
                assert_eq!(true, true);
            }
            Err(e) => {
                panic!("shouldn't happen")
            }
        }
    }
    #[test]
    fn parses_multi_many1() {
        match chunk("(<>[<>}") {
            Ok((i, _)) => {
                dbg!(i);
                assert_eq!(true, false);
            }
            Err(nom::Err::Error(ErrorTree::Stack {
                contexts,
                base,
            })) => {
                dbg!(base, &contexts);
                // ignore lines that end early
                let ctx = contexts
                    .iter()
                    .find(|v| v.1 == Context("chars"))
                    .unwrap();
                let maybe_chunk = chunk(ctx.0);
                dbg!(maybe_chunk);
                // dbg!(ctx);
                let c = ctx.0.chars().next().unwrap();

                // assert_ne!(location, "");
                assert_eq!(c, '}');
                // assert_eq!(false, true);
            }
            Err(e) => {
                dbg!(e);
                panic!("shouldnt happen")
            }
        }
    }
    #[test]
    fn parses_parens_and_more() {
        match chunk("([{<>}])") {
            Ok((i, _)) => {
                dbg!(i);
                assert_eq!(true, true);
            }
            Err(e) => {
                dbg!(e);
                panic!("shouldn't happen")
            }
        }
    }
    #[test]
    fn parses_nested() {
        match chunk("((())[])") {
            Ok((i, _)) => {
                dbg!(i);
                assert_eq!(true, true);
            }
            Err(e) => {
                dbg!(e);
                panic!("shouldn't happen")
            }
        }
    }
    #[test]
    fn end_of_line_parser() {
        match chunk("({()(") {
            Ok((i, _)) => {
                dbg!(i);
                assert_eq!(true, false);
            }
            Err(nom::Err::Incomplete(e)) => {
                // ignore lines that end early
                // assert_eq!(e.input, "");
                assert_eq!(true, true)
            }
            Err(e) => {
                dbg!(e);
                panic!("shouldnt happen")
            }
        }
    }

    #[test]
    fn mismatched_brace() {
        match chunk("(})") {
            Ok((i, _)) => {
                dbg!(i);
                assert_eq!(true, false);
            }
            Err(nom::Err::Error(ErrorTree::Stack {
                contexts,
                base,
            })) => {
                dbg!(base, &contexts);
                // ignore lines that end early
                let ctx = contexts
                    .iter()
                    .find(|v| v.1 == Context("chunk"))
                    .unwrap();
                // dbg!(ctx);
                let c = ctx.0.chars().next().unwrap();

                // assert_ne!(location, "");
                assert_eq!(c, '}');
                // assert_eq!(false, true);
            }
            Err(e) => {
                dbg!(e);
                panic!("shouldnt happen")
            }
        }
    }

    // #[test]
    // fn part2_test_demo_data() {
    //     assert_eq!(1134, process_part2(input));
    // }
}
