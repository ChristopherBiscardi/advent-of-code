use bitvec::prelude::*;
use nom::{
    bits::complete::{tag, take},
    character::complete::{
        self, alpha1, anychar, newline, one_of, u32,
    },
    multi::{
        length_value, many0, many1, many_m_n,
        separated_list1,
    },
    sequence::{
        pair, preceded, separated_pair, terminated,
    },
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Literal {
        version: u8,
        type_id: u8,
        value: usize,
    },
    Operator {
        version: u8,
        type_id: u8,
        packets: Vec<Packet>,
    },
}

fn operator(
    input: (&[u8], usize),
) -> IResult<(&[u8], usize), Vec<Packet>> {
    // dbg!(input);
    let (input, length_type_id) = take(1_usize)(input)?;
    // dbg!(length_type_id);
    match length_type_id {
        0 => {
            // println!("offset is {} bits", input.1);
            // println!("original input is:");
            // for byte in input.0.iter() {
            //     println!("{:08b}", byte);
            // }
            let (input, length_in_bits): (_, usize) =
                take(15_usize)(input)?;
            // dbg!(&length_in_bits);

            // println!("offset is {} bits", input.1);
            // println!("original input is:");
            // for byte in input.0.iter() {
            //     println!("{:08b}", byte);
            // }

            let (first_bits, offset, input) =
                if length_in_bits % 8 > 0 {
                    let (input, first): (_, u8) =
                        take(length_in_bits % 8)(input)?;
                    (
                        Some(first),
                        8 - length_in_bits % 8,
                        input,
                    )
                } else {
                    (None, 0, input)
                };

            // println!(
            //     "first bits: {:08b}",
            //     &first_bits.unwrap()
            // );
            // println!("input bits");
            // for byte in input.0.iter() {
            //     println!("{:08b}", byte);
            // }
            // dbg!(length_in_bits / 8);
            let (input, rest_of_bits): (_, Vec<u8>) =
                many_m_n(
                    length_in_bits / 8,
                    length_in_bits / 8,
                    take(8_usize),
                )(input)?;

            let bits = match first_bits {
                Some(byte) => {
                    let mut bits: Vec<u8> =
                        vec![first_bits.unwrap()];
                    bits.extend(rest_of_bits);
                    bits
                }
                None => rest_of_bits,
            };

            // println!("bits at offset {}", offset);
            // for byte in bits.iter() {
            //     println!("{:08b}", byte);
            // }
            let (_input, packets) = many1(puzzle_input)((
                &bits,  // 63 - length_in_bits - 1,
                offset, // wtf magic number
            ))
            .unwrap();

            // dbg!(&input);
            Ok((input, packets))
        }
        1 => {
            let (input, num_subpackets) =
                take(11_usize)(input)?;

            // dbg!(num_subpackets);
            // TODO: What is this number actually?
            let (input, packets) = many_m_n(
                num_subpackets,
                num_subpackets,
                puzzle_input,
            )(input)?;
            Ok((input, packets))
        }
        _ => panic!("invalid length type id"),
    }
}
fn literal(
    input: (&[u8], usize),
) -> IResult<(&[u8], usize), u8> {
    let (input, bits) = take(4_usize)(input)?;
    Ok((input, bits))
}
fn literals(
    input: (&[u8], usize),
) -> IResult<(&[u8], usize), (usize, usize)> {
    // let input_len = input.len();
    let (input, bits) =
        many0(preceded(tag(0b1, 1_usize), literal))(input)?;
    let (input, ending_literal) =
        preceded(tag(0b0, 1_usize), literal)(input)?;
    // dbg!(&bits, ending_literal);
    let mut bitshift: usize = 0;
    for byte in bits.iter() {
        bitshift = bitshift.checked_shl(4).unwrap()
            | *byte as usize;
    }
    // dbg!(&ending_literal);
    let value = bitshift.checked_shl(4).unwrap()
        | ending_literal as usize;
    // dbg!(&value);
    let num_parsed_bits = bits.len() * 5 + 5;
    Ok((input, (num_parsed_bits % 4, value)))
}

fn puzzle_input(
    input: (&[u8], usize),
) -> IResult<(&[u8], usize), Packet> {
    let (input, version) = take(3_usize)(input)?;
    let (input, type_id) = take(3_usize)(input)?;
    // let version =
    // u8::from_str_radix(binary_version, 2).unwrap();
    // let type_id =
    //     u8::from_str_radix(binary_type_id, 2).unwrap();
    // dbg!(version, type_id, input);
    // dbg!(version, type_id);
    // if type_id != 4 {
    //     panic!("");
    // };
    match type_id {
        4 => {
            // TODO: here doesn't deal with starting padding 0s
            let (input, (_skip, value)) = literals(input)?;
            // dbg!(&values);
            // dbg!(input);
            Ok((
                input,
                Packet::Literal {
                    version,
                    type_id,
                    value,
                },
            ))
        }
        _ => {
            let (input, packet) = operator(input)?;
            Ok((
                input,
                Packet::Operator {
                    version,
                    type_id,
                    packets: packet,
                },
            ))
        }
    }
}

fn process_packet(packet: &Packet) -> usize {
    match packet {
        Packet::Operator {
            version,
            type_id: _,
            packets,
        } => {
            let sum: usize =
                packets.iter().map(process_packet).sum();
            (*version as usize) + sum
        }

        Packet::Literal { version, .. } => {
            *version as usize
        }
    }
}
fn process_packet2(packet: &Packet) -> usize {
    match packet {
        Packet::Operator {
            version,
            type_id,
            packets,
        } => {
            let mut packets =
                packets.iter().map(process_packet2);
            match type_id {
                0 => packets.sum(),
                1 => packets.product(),
                2 => packets.min().unwrap(),
                3 => packets.max().unwrap(),
                5 => {
                    let a = packets.next().unwrap();
                    let b = packets.next().unwrap();
                    let c = packets.next();
                    assert_eq!(c, None);
                    if a > b {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    let a = packets.next().unwrap();
                    let b = packets.next().unwrap();
                    let c = packets.next();
                    assert_eq!(c, None);
                    if a < b {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    let a = packets.next().unwrap();
                    let b = packets.next().unwrap();
                    let c = packets.next();
                    assert_eq!(c, None);
                    if a == b {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("askfjlasf"),
            }
        }
        Packet::Literal { value, .. } => *value as usize,
    }
}
pub fn process_part1(input: &[u8]) -> usize {
    let bytes = hex::decode(input).unwrap();
    let (_, packet) = puzzle_input((&bytes, 0)).unwrap();

    process_packet(&packet)
}

pub fn process_part2(input: &[u8]) -> usize {
    let bytes = hex::decode(input).unwrap();
    let (_, packet) = puzzle_input((&bytes, 0)).unwrap();

    process_packet2(&packet)
}

fn hex_as_binary_str(input: &str) -> String {
    input
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .collect::<String>()
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
        include_str!("./test-input.txt");

    // #[test]
    // fn test_packet_literal_parser() {
    //     let bits = 0b101111111000101000_usize.to_be_bytes();
    //     let lits = literals((&bits, 0)).unwrap();
    // dbg!(lits.0);
    //     for lit in lits.0 .0.iter() {
    //         print!("{:b}", lit);
    //     }
    //     assert_eq!((3, 2021), lits.1);
    // }
    #[test]
    fn test_packet_literal_parser_A() {
        let bits = 0b00001_u8.to_be_bytes();
        assert_eq!((1, 1), literals((&bits, 3)).unwrap().1);
    }
    #[test]
    fn test_packet_literal_parser_B() {
        let bits = 0b00010_u8.to_be_bytes();
        assert_eq!((1, 2), literals((&bits, 3)).unwrap().1);
    }
    #[test]
    fn test_packet_literal_parser_C() {
        let bits = 0b00011_u8.to_be_bytes();

        assert_eq!((1, 3), literals((&bits, 3)).unwrap().1);
    }

    #[test]
    fn test_puzzle_input() {
        let bytes =
            hex::decode("D2FE28".as_bytes()).unwrap();

        let result = puzzle_input((&bytes, 0)).unwrap();
        assert_eq!(
            Packet::Literal {
                version: 6,
                type_id: 4,
                value: 2021
            },
            result.1
        );
    }
    // #[test]
    // fn test_test_operator_1_parser() {
    //     let bits = 0b10000000001101010000001100100000100011000001100000_usize.to_be_bytes();
    // dbg!(&bits);
    //     println!("asfk");

    //     for bit in bits.iter() {
    //         print!("{:b}", bit);
    //     }
    //     println!("\nAsfklj");

    //     assert_eq!(
    //         vec![
    //             Packet::Literal {
    //                 version: 2,
    //                 type_id: 4,
    //                 value: 1,
    //             },
    //             Packet::Literal {
    //                 version: 4,
    //                 type_id: 4,
    //                 value: 2,
    //             },
    //             Packet::Literal {
    //                 version: 1,
    //                 type_id: 4,
    //                 value: 3,
    //             }
    //         ],
    //         operator((&bits, 0)).unwrap().1
    //     );
    // }
    // // #[test]
    // // fn part1_test_demo_data() {
    // //     assert_eq!(40, process_part1(INPUT));
    // // }
    #[test]
    fn part1_test_A() {
        assert_eq!(
            16,
            process_part1("8A004A801A8002F478".as_bytes())
        );
    }
    #[test]
    fn part1_test_B() {
        assert_eq!(
            12,
            process_part1(
                "620080001611562C8802118E34".as_bytes()
            )
        );
    }
    #[test]
    fn part1_test_C() {
        assert_eq!(
            23,
            process_part1(
                "C0015000016115A2E0802F182340".as_bytes()
            )
        );
    }
    #[test]
    fn part1_test_D() {
        assert_eq!(
            31,
            process_part1(
                "A0016C880162017C3686B18A3D4780".as_bytes()
            )
        );
    }
    // // #[test]
    // // fn part2_test_demo_data() {
    // //     assert_eq!(315, process_part2(INPUT));
    // // }

    #[test]
    fn test_part2_C200B40A82() {
        assert_eq!(
            3,
            process_part2("C200B40A82".as_bytes())
        );
    }
    // #[test]
    // fn test_part2_04005AC33890() {
    //     assert_eq!(54, process_part2("04005AC33890"));
    // }
    // #[test]
    // fn test_part2_880086C3E88112() {
    //     assert_eq!(7, process_part2("880086C3E88112"));
    // }
    // #[test]
    // fn test_part2_CE00C43D881120() {
    //     assert_eq!(9, process_part2("CE00C43D881120"));
    // }
    // #[test]
    // fn test_part2_D8005AC2A8F0() {
    //     assert_eq!(1, process_part2("D8005AC2A8F0"));
    // }
    // #[test]
    // fn test_part2_F600BC2D8F() {
    //     assert_eq!(0, process_part2("F600BC2D8F"));
    // }
    // #[test]
    // fn test_part2_9C005AC2F8F0() {
    //     assert_eq!(0, process_part2("9C005AC2F8F0"));
    // }
    // #[test]
    // fn test_part2_9C0141080250320F1802104A08() {
    //     assert_eq!(
    //         1,
    //         process_part2("9C0141080250320F1802104A08")
    //     );
    // }
}
