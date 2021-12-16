use ndarray::{concatenate, Array2, Axis};
use nom::{
    bytes::complete::{tag, take},
    character::complete::{
        self, alpha1, anychar, newline, one_of, u32,
    },
    multi::{many0, many1, many_m_n, separated_list1},
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

fn operator(input: &str) -> IResult<&str, Vec<Packet>> {
    let (input, length_type_id) = take(1_usize)(input)?;
    match length_type_id {
        "0" => {
            let (input, length_in_bits) =
                take(15_usize)(input)?;
            let (input, bits) = take(
                u32::from_str_radix(length_in_bits, 2)
                    .unwrap(),
            )(input)?;
            let (_input, packets) =
                many1(puzzle_input)(bits)?;

            Ok((input, packets))
        }
        "1" => {
            let (input, parsed_number_of_subpackets) =
                take(11_usize)(input)?;

            let num_subpackets = usize::from_str_radix(
                parsed_number_of_subpackets,
                2,
            )
            .unwrap();

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
fn literal(input: &str) -> IResult<&str, &str> {
    let (input, bits) = take(4_usize)(input)?;
    Ok((input, bits))
}
fn literals(input: &str) -> IResult<&str, (usize, usize)> {
    let input_len = input.len();
    let (input, bits) =
        many0(preceded(tag("1"), literal))(input)?;
    let (input, ending_literal) =
        preceded(tag("0"), literal)(input)?;
    let num_parsed_chars = input_len - input.len();
    // dbg!(num_parsed_chars);

    let value = usize::from_str_radix(
        &format!(
            "{}{}",
            bits.into_iter().collect::<String>(),
            ending_literal
        ),
        2,
    )
    .unwrap();
    Ok((input, (num_parsed_chars % 4, value)))
}

fn puzzle_input(input: &str) -> IResult<&str, Packet> {
    let (input, binary_version) = take(3_usize)(input)?;
    let (input, binary_type_id) = take(3_usize)(input)?;
    let version =
        u8::from_str_radix(binary_version, 2).unwrap();
    let type_id =
        u8::from_str_radix(binary_type_id, 2).unwrap();
    // dbg!(version, type_id, input);
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
pub fn process_part1(input: &str) -> usize {
    let binary_as_string = hex_as_binary_str(input);
    let (_, packet) =
        puzzle_input(&binary_as_string).unwrap();

    process_packet(&packet)
}

pub fn process_part2(input: &str) -> usize {
    let binary_as_string = hex_as_binary_str(input);
    let (_, packet) =
        puzzle_input(&binary_as_string).unwrap();

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

    #[test]
    fn test_packet_literal_parser() {
        assert_eq!(
            ("000", (3, 2021)),
            literals("101111111000101000").unwrap()
        );
    }
    #[test]
    fn test_packet_literal_parser_A() {
        assert_eq!(
            ("", (1, 1)),
            literals("00001").unwrap()
        );
    }
    #[test]
    fn test_packet_literal_parser_B() {
        assert_eq!(
            ("", (1, 2)),
            literals("00010").unwrap()
        );
    }
    #[test]
    fn test_packet_literal_parser_C() {
        assert_eq!(
            ("", (1, 3)),
            literals("00011").unwrap()
        );
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(
            (
                "000",
                Packet::Literal {
                    version: 6,
                    type_id: 4,
                    value: 2021
                }
            ),
            puzzle_input(&hex_as_binary_str("D2FE28"),)
                .unwrap()
        );
    }
    #[test]
    fn test_test_operator_1_parser() {
        assert_eq!(
            ("00000", vec![
                Packet::Literal {
                    version: 2,
                    type_id: 4,
                    value: 1,
                }, Packet::Literal {
                    version: 4,
                    type_id: 4,
                    value: 2,
                }, Packet::Literal {
                    version: 1,
                    type_id: 4,
                    value: 3,
                }]
            ),
            operator(&"10000000001101010000001100100000100011000001100000")
                .unwrap()
        );
    }
    // #[test]
    // fn part1_test_demo_data() {
    //     assert_eq!(40, process_part1(INPUT));
    // }
    #[test]
    fn part1_test_A() {
        assert_eq!(16, process_part1("8A004A801A8002F478"));
    }
    #[test]
    fn part1_test_B() {
        assert_eq!(
            12,
            process_part1("620080001611562C8802118E34")
        );
    }
    #[test]
    fn part1_test_C() {
        assert_eq!(
            23,
            process_part1("C0015000016115A2E0802F182340")
        );
    }
    #[test]
    fn part1_test_D() {
        assert_eq!(
            31,
            process_part1("A0016C880162017C3686B18A3D4780")
        );
    }
    // #[test]
    // fn part2_test_demo_data() {
    //     assert_eq!(315, process_part2(INPUT));
    // }

    #[test]
    fn test_part2_C200B40A82() {
        assert_eq!(3, process_part2("C200B40A82"));
    }
    #[test]
    fn test_part2_04005AC33890() {
        assert_eq!(54, process_part2("04005AC33890"));
    }
    #[test]
    fn test_part2_880086C3E88112() {
        assert_eq!(7, process_part2("880086C3E88112"));
    }
    #[test]
    fn test_part2_CE00C43D881120() {
        assert_eq!(9, process_part2("CE00C43D881120"));
    }
    #[test]
    fn test_part2_D8005AC2A8F0() {
        assert_eq!(1, process_part2("D8005AC2A8F0"));
    }
    #[test]
    fn test_part2_F600BC2D8F() {
        assert_eq!(0, process_part2("F600BC2D8F"));
    }
    #[test]
    fn test_part2_9C005AC2F8F0() {
        assert_eq!(0, process_part2("9C005AC2F8F0"));
    }
    #[test]
    fn test_part2_9C0141080250320F1802104A08() {
        assert_eq!(
            1,
            process_part2("9C0141080250320F1802104A08")
        );
    }
}
