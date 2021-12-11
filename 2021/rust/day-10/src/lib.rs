use std::{collections::HashMap, fmt};

use nom::{
    character::{
        complete::{self},
        streaming::{char, one_of},
    },
    error::context,
    multi::{many0, many1},
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

// fn chunk_2(
//     original_input: &str,
// ) -> IResult<&str, String, ErrorTree<&str>> {
//     dbg!("chunk2");
//     if original_input == "" {
//         Ok(("", "".to_string()))
//     } else {
//         let (input, open_char) =
//             one_of("({<[")(original_input)?;
//         let c_res: IResult<&str, &str, ErrorTree<&str>> =
//             tag(match open_char {
//                 '{' => "}",
//                 '(' => ")",
//                 '[' => "]",
//                 '<' => ">",
//                 _ => panic!("unrecognized char"),
//             })(input);
//         match c_res {
//             Ok((input, something)) => {
//                 dbg!(&input, &something);
//                 Ok((
//                     input,
//                     open_char.to_string() + something,
//                 ))
//             }
//             Err(nom::Err::Incomplete(_)) => {
//                 dbg!("incomplete here");
//                 Ok((
//                     input,
//                     match open_char {
//                         '{' => "}",
//                         '(' => ")",
//                         '[' => "]",
//                         '<' => ">",
//                         _ => panic!("unrecognized char"),
//                     }
//                     .to_string(),
//                 ))
//             }
//             Err(e) => {
//                 // dbg!(&e);
//                 dbg!(&input);
//                 let (input, leftovers) =
//                     many1(chunk_2)(input)?;
//                 dbg!(&leftovers);
//                 Ok((
//                     input,
//                     leftovers
//                         .into_iter()
//                         .collect::<String>(),
//                 ))
//             }
//         }
//     }

// if let Ok((input, _)) = c_res {
//     Ok((input, "".to_string()))
// } else {
//     let (input, leftovers) =
//         context("chunk", many0(chunk))(input)?;

//     let chars_res = context(
//         "chars",
//         tag(match open_char {
//             '{' => "}",
//             '(' => ")",
//             '[' => "]",
//             '<' => ">",
//             _ => panic!("unrecognized char"),
//         }),
//     )(input);
//     dbg!(&chars_res);
//     match chars_res {
//         Ok((input, c)) => {
//             dbg!(&input, &c);
//             Ok((input, "".to_string()))
//         }
//         Err(e) => {
//             dbg!(&e);
//             match e {
//                 nom::Err::Incomplete(_) => Ok((
//                     input,
//                     match open_char {
//                         '{' => "}",
//                         '(' => ")",
//                         '[' => "]",
//                         '<' => ">",
//                         _ => panic!("here"),
//                     }
//                     .to_string(),
//                 )),
//                 e => Err(e),
//             }
//         }
//     }
//     // Ok((input, "".to_string()))
// }
// }

fn chunk_2(original_input: &str) -> IResult<&str, Ast> {
    // println!("chunk_2: {}", &original_input);
    let (input, open_char) =
        complete::one_of("({<[")(original_input)?;
    let c_res: IResult<&str, char> =
        complete::char(match open_char {
            '{' => '}',
            '(' => ')',
            '[' => ']',
            '<' => '>',
            _ => panic!("unrecognized char"),
        })(input);
    match c_res {
        Ok((input, close_char)) => {
            // dbg!(&input, &close_char);
            Ok((
                input,
                Ast {
                    left: open_char,
                    right: Some(close_char),
                    children: vec![],
                },
            ))
        }
        // Err(nom::Err::Incomplete(_)) => {
        //     // println!("incomplete: {}", &input);
        //     Ok((
        //         input,
        //         Ast {
        //             left: open_char,
        //             right: None,
        //             children: vec![],
        //         },
        //     ))
        // }
        Err(e) => {
            if input == "" {
                Ok((
                    input,
                    Ast {
                        left: open_char,
                        right: None,
                        children: vec![],
                    },
                ))
            } else {
                // dbg!(input);
                let (input, output) =
                    many1(chunk_2)(input)?;
                //                 println!(
                //                     "i: {}
                // o: {:?}",
                //                     &e, &output
                //                 );
                let c_res: IResult<&str, char> =
                    char(match open_char {
                        '{' => '}',
                        '(' => ')',
                        '[' => ']',
                        '<' => '>',
                        _ => panic!("unrecognized char"),
                    })(input);
                match c_res {
                    Ok((input, c)) => Ok((
                        input,
                        Ast {
                            left: open_char,
                            right: Some(c),
                            children: output,
                        },
                    )),
                    Err(nom::Err::Incomplete(_)) => Ok((
                        input,
                        Ast {
                            left: open_char,
                            right: None,
                            children: output,
                        },
                    )),
                    Err(e) => Err(e),
                }
            }
        }
    }
}
#[derive(Debug, PartialEq)]
struct Ast {
    left: char,
    right: Option<char>,
    children: Vec<Ast>,
}

impl fmt::Display for Ast {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "{}{}",
            // self.left,
            self.children
                .iter()
                .map(|v| v.to_string())
                .collect::<String>(),
            if let None = self.right {
                match self.left {
                    '{' => "}",
                    '(' => ")",
                    '[' => "]",
                    '<' => ">",
                    _ => panic!("unrecognized char"),
                }
            } else {
                // self.right.unwrap()
                ""
            }
        )
    }
}

pub fn process_part2(input: &str) -> u64 {
    let mut results: Vec<u64> = input
        .lines()
        .map(|line| {
            let mut res = chunk_2(line);
            loop {
                match res {
                    Ok((input, output)) => {
                        if input.len() > 0 {
                            res = chunk_2(input);
                        } else {
                            break Ok((input, output));
                        }
                    }
                    Err(e) => break Err(e),
                }
            }
        })
        .enumerate()
        // filter out lines that end early
        .filter_map(|(i, res)| {
            // dbg!(&res);
            match res {
                Ok((input, v)) => {
                    let num = v.to_string().chars().fold(
                        0,
                        |acc, v| {
                            acc * 5
                                + match v {
                                    ')' => 1,
                                    ']' => 2,
                                    '}' => 3,
                                    '>' => 4,
                                    _ => panic!("askflj"),
                                }
                        },
                    );
                    Some(num)
                }
                Err(e) => {
                    // dbg!(e);
                    None
                }

                // Err(ErrorTree::Stack {
                //     base,
                //     contexts,
                // }) => {
                //     dbg!(base);
                //     // dbg!(&base, &contexts);
                //     // let ctx = contexts
                //     //     .iter()
                //     //     .find(|v| v.1 == Context("chars"))
                //     //     .unwrap();
                //     // // dbg!(ctx);
                //     // let c = ctx.0.chars().next().unwrap();
                //     // let res = scoring.get(&c);
                //     // // dbg!(c);
                //     // res
                //     Some(0)
                // }
                _ => panic!("uh oh"),
            }
        })
        .collect();
    results.sort();
    results[results.len() / 2]
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

    #[test]
    fn part2_test_demo_data() {
        assert_eq!(288957, process_part2(input));
    }

    #[test]
    fn part2_builds_paren() {
        match chunk_2("(") {
            Ok((leftover_input, v)) => {
                assert_eq!(
                    v,
                    Ast {
                        left: '(',
                        right: None,
                        children: vec![]
                    }
                );
            }
            Err(e) => {
                dbg!(e);
                assert_eq!(true, false);
            }
        }
    }
    #[test]
    fn part2_builds_parens_and_more() {
        match chunk_2("({") {
            Ok((leftover_input, v)) => {
                assert_eq!(
                    v,
                    Ast {
                        left: '(',
                        right: None,
                        children: vec![Ast {
                            left: '{',
                            right: None,
                            children: vec![]
                        }]
                    }
                );
            }
            Err(e) => {
                dbg!(e);
                assert_eq!(true, false);
            }
        }
    }
}
