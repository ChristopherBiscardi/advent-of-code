fn main() {
    let opcode_3_input = 2;
    let mut input: Vec<i64> = vec![
        1102, 34463338, 34463338, 63, 1007, 63, 34463338, 63, 1005, 63, 53, 1101, 0, 3, 1000, 109,
        988, 209, 12, 9, 1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63, 1005, 63, 65, 1008, 1000,
        2, 63, 1005, 63, 904, 1008, 1000, 0, 63, 1005, 63, 58, 4, 25, 104, 0, 99, 4, 0, 104, 0, 99,
        4, 17, 104, 0, 99, 0, 0, 1102, 23, 1, 1004, 1102, 1, 26, 1000, 1102, 897, 1, 1028, 1101,
        27, 0, 1012, 1102, 33, 1, 1001, 1102, 32, 1, 1007, 1101, 39, 0, 1005, 1101, 0, 29, 1018,
        1101, 0, 0, 1020, 1101, 1, 0, 1021, 1101, 0, 21, 1002, 1102, 1, 35, 1014, 1101, 0, 36,
        1009, 1102, 1, 38, 1006, 1102, 1, 251, 1024, 1102, 28, 1, 1017, 1102, 37, 1, 1008, 1102, 1,
        329, 1026, 1102, 25, 1, 1011, 1102, 31, 1, 1013, 1102, 892, 1, 1029, 1102, 242, 1, 1025,
        1102, 1, 881, 1022, 1102, 22, 1, 1003, 1102, 874, 1, 1023, 1101, 20, 0, 1016, 1101, 24, 0,
        1019, 1101, 0, 326, 1027, 1101, 0, 34, 1015, 1102, 1, 30, 1010, 109, -2, 2102, 1, 7, 63,
        1008, 63, 36, 63, 1005, 63, 205, 1001, 64, 1, 64, 1105, 1, 207, 4, 187, 1002, 64, 2, 64,
        109, 9, 21101, 40, 0, 6, 1008, 1013, 43, 63, 1005, 63, 227, 1105, 1, 233, 4, 213, 1001, 64,
        1, 64, 1002, 64, 2, 64, 109, 26, 2105, 1, -9, 4, 239, 1001, 64, 1, 64, 1106, 0, 251, 1002,
        64, 2, 64, 109, -15, 1205, 2, 263, 1105, 1, 269, 4, 257, 1001, 64, 1, 64, 1002, 64, 2, 64,
        109, -9, 2102, 1, 0, 63, 1008, 63, 36, 63, 1005, 63, 295, 4, 275, 1001, 64, 1, 64, 1106, 0,
        295, 1002, 64, 2, 64, 109, -14, 1207, 10, 38, 63, 1005, 63, 311, 1105, 1, 317, 4, 301,
        1001, 64, 1, 64, 1002, 64, 2, 64, 109, 28, 2106, 0, 4, 1106, 0, 335, 4, 323, 1001, 64, 1,
        64, 1002, 64, 2, 64, 109, -8, 1206, 6, 351, 1001, 64, 1, 64, 1106, 0, 353, 4, 341, 1002,
        64, 2, 64, 109, -1, 2107, 33, -7, 63, 1005, 63, 369, 1106, 0, 375, 4, 359, 1001, 64, 1, 64,
        1002, 64, 2, 64, 109, -9, 2108, 26, -1, 63, 1005, 63, 395, 1001, 64, 1, 64, 1106, 0, 397,
        4, 381, 1002, 64, 2, 64, 109, 3, 1201, -2, 0, 63, 1008, 63, 38, 63, 1005, 63, 419, 4, 403,
        1105, 1, 423, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -13, 2101, 0, 9, 63, 1008, 63, 23, 63,
        1005, 63, 445, 4, 429, 1105, 1, 449, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 11, 1208, 1,
        32, 63, 1005, 63, 471, 4, 455, 1001, 64, 1, 64, 1106, 0, 471, 1002, 64, 2, 64, 109, 17,
        21108, 41, 38, -4, 1005, 1019, 487, 1105, 1, 493, 4, 477, 1001, 64, 1, 64, 1002, 64, 2, 64,
        109, 6, 1206, -9, 511, 4, 499, 1001, 64, 1, 64, 1106, 0, 511, 1002, 64, 2, 64, 109, -23,
        21102, 42, 1, 8, 1008, 1014, 42, 63, 1005, 63, 533, 4, 517, 1106, 0, 537, 1001, 64, 1, 64,
        1002, 64, 2, 64, 109, -3, 2107, 36, 5, 63, 1005, 63, 555, 4, 543, 1106, 0, 559, 1001, 64,
        1, 64, 1002, 64, 2, 64, 109, -6, 1202, 5, 1, 63, 1008, 63, 21, 63, 1005, 63, 581, 4, 565,
        1106, 0, 585, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 1, 1208, 10, 40, 63, 1005, 63, 605,
        1001, 64, 1, 64, 1106, 0, 607, 4, 591, 1002, 64, 2, 64, 109, 7, 1201, 0, 0, 63, 1008, 63,
        42, 63, 1005, 63, 631, 1001, 64, 1, 64, 1106, 0, 633, 4, 613, 1002, 64, 2, 64, 109, 1,
        21107, 43, 42, 7, 1005, 1013, 649, 1105, 1, 655, 4, 639, 1001, 64, 1, 64, 1002, 64, 2, 64,
        109, 7, 21108, 44, 44, 3, 1005, 1016, 677, 4, 661, 1001, 64, 1, 64, 1106, 0, 677, 1002, 64,
        2, 64, 109, -7, 21102, 45, 1, 9, 1008, 1015, 44, 63, 1005, 63, 701, 1001, 64, 1, 64, 1106,
        0, 703, 4, 683, 1002, 64, 2, 64, 109, 13, 21101, 46, 0, -7, 1008, 1012, 46, 63, 1005, 63,
        729, 4, 709, 1001, 64, 1, 64, 1105, 1, 729, 1002, 64, 2, 64, 109, -13, 2101, 0, 3, 63,
        1008, 63, 33, 63, 1005, 63, 753, 1001, 64, 1, 64, 1106, 0, 755, 4, 735, 1002, 64, 2, 64,
        109, 14, 1205, 1, 773, 4, 761, 1001, 64, 1, 64, 1105, 1, 773, 1002, 64, 2, 64, 109, -23,
        1202, 10, 1, 63, 1008, 63, 30, 63, 1005, 63, 797, 1001, 64, 1, 64, 1105, 1, 799, 4, 779,
        1002, 64, 2, 64, 109, 13, 2108, 22, -7, 63, 1005, 63, 817, 4, 805, 1106, 0, 821, 1001, 64,
        1, 64, 1002, 64, 2, 64, 109, -11, 1207, 5, 24, 63, 1005, 63, 843, 4, 827, 1001, 64, 1, 64,
        1105, 1, 843, 1002, 64, 2, 64, 109, 11, 21107, 47, 48, 7, 1005, 1017, 861, 4, 849, 1106, 0,
        865, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 15, 2105, 1, -2, 1001, 64, 1, 64, 1106, 0, 883,
        4, 871, 1002, 64, 2, 64, 109, 10, 2106, 0, -7, 4, 889, 1106, 0, 901, 1001, 64, 1, 64, 4,
        64, 99, 21102, 1, 27, 1, 21102, 1, 915, 0, 1105, 1, 922, 21201, 1, 28510, 1, 204, 1, 99,
        109, 3, 1207, -2, 3, 63, 1005, 63, 964, 21201, -2, -1, 1, 21102, 1, 942, 0, 1106, 0, 922,
        22102, 1, 1, -1, 21201, -2, -3, 1, 21101, 957, 0, 0, 1106, 0, 922, 22201, 1, -1, -2, 1105,
        1, 968, 21202, -2, 1, -2, 109, -3, 2106, 0, 0,
    ];
    for i in 0..(input.len() * 10) {
        input.push(0)
    }
    let mut relative_base = 0;
    let mut opcode: i64 = input[0];
    let mut position = 0;
    let mut mode_list: Vec<u32> = vec![];

    loop {
        if opcode == 99 {
            break;
        };
        // println!("input {:?}", input);
        println!("opcode {}", opcode);
        match opcode {
            1 | 2 => {
                let mode1 = mode_list.pop();
                let mode2 = mode_list.pop();
                let mode3 = mode_list.pop();
                let value_a = get_value(&input, &relative_base, position + 1 as usize, mode1);
                let value_b = get_value(&input, &relative_base, position + 2 as usize, mode2);
                let position_to_replace =
                    get_position(&input, &relative_base, position + 3 as usize, mode3);

                let final_size;
                if opcode == 1 {
                    final_size = value_a + value_b;
                } else {
                    final_size = value_a * value_b;
                }

                input[position_to_replace as usize] = final_size;
                position = position + 4;
                opcode = input[position];
                mode_list = vec![];
            }
            3 => {
                let position_to_replace = get_position(
                    &input,
                    &relative_base,
                    position + 1 as usize,
                    mode_list.pop(),
                );
                // println!("position_to_replace {}", position_to_replace);
                input[position_to_replace as usize] = opcode_3_input;
                if position_to_replace == position as i64 {
                    println!("asfkjsgfjsgkljsdlgfkjksdl")
                }
                position = position + 2;
                opcode = input[position];
            }
            4 => {
                let value = get_value(
                    &input,
                    &relative_base,
                    position + 1 as usize,
                    mode_list.pop(),
                );
                // let value = input[input[position+1]as usize];
                println!("op4: {}", value);
                position = position + 2;
                opcode = input[position];
                mode_list = vec![];
            }
            5 => {
                let value = get_value(
                    &input,
                    &relative_base,
                    position + 1 as usize,
                    mode_list.pop(),
                );
                let value2 = get_value(
                    &input,
                    &relative_base,
                    position + 2 as usize,
                    mode_list.pop(),
                );
                if value != 0 {
                    // println!("inserting {} at {}", value2, position);
                    // input[position] = value2;
                    position = value2 as usize
                } else {
                    // jump
                    position = position + 3
                }
                opcode = input[position];
                mode_list = vec![];
            }
            6 => {
                let value = get_value(
                    &input,
                    &relative_base,
                    position + 1 as usize,
                    mode_list.pop(),
                );
                let value2 = get_value(
                    &input,
                    &relative_base,
                    position + 2 as usize,
                    mode_list.pop(),
                );
                // println!("value: {} {}", value, value2);
                if value == 0 {
                    // println!("input[position] {}", input[position]);
                    // input[position] = value2;
                    // println!("input[position] {}", input[position]);
                    position = value2 as usize;
                // opcode = input[position];
                // println!("set position to {}", position)
                // println!("position {}", position)
                } else {
                    // jump
                    position = position + 3
                }
                // println!("next pos {}, op {}", position, input[position]);
                opcode = input[position];
                mode_list = vec![];
            }
            7 => {
                let value1 = get_value(
                    &input,
                    &relative_base,
                    position + 1 as usize,
                    mode_list.pop(),
                );
                let value2 = get_value(
                    &input,
                    &relative_base,
                    position + 2 as usize,
                    mode_list.pop(),
                );
                let value3 = get_position(
                    &input,
                    &relative_base,
                    position + 3 as usize,
                    mode_list.pop(),
                );
                println!("input3 {}", value3);
                input[value3 as usize] = if value1 < value2 { 1 } else { 0 };
                // jump if value3 is not instruction index (aka position)
                position = position + 4;
                opcode = input[position];
                mode_list = vec![];
            }
            8 => {
                let value1 = get_value(
                    &input,
                    &relative_base,
                    position + 1 as usize,
                    mode_list.pop(),
                );
                let value2 = get_value(
                    &input,
                    &relative_base,
                    position + 2 as usize,
                    mode_list.pop(),
                );
                let value3 = get_position(
                    &input,
                    &relative_base,
                    position + 3 as usize,
                    mode_list.pop(),
                );
                input[value3 as usize] = if value1 == value2 { 1 } else { 0 };
                position = position + 4;
                opcode = input[position];
                mode_list = vec![];
            }
            9 => {
                let value = get_value(
                    &input,
                    &relative_base,
                    position + 1 as usize,
                    mode_list.pop(),
                );
                relative_base = relative_base + value;
                position = position + 2;
                opcode = input[position];
                mode_list = vec![];
            }
            x => {
                let mut digits: Vec<u32> = x
                    .to_string()
                    .chars()
                    .map(|d| d.to_digit(10).unwrap())
                    .collect();
                let one = digits.pop();
                let two = digits.pop();
                let _unused: Option<i64> = one.and_then(|t| match two {
                    Some(n) => {
                        opcode = (n * 10 + t) as i64;
                        None
                    }
                    None => {
                        opcode = t as i64;
                        None
                    }
                });
                mode_list = digits;
            }
        }
    }
    // println!("{}", input[0]);
}

fn get_value(input: &Vec<i64>, relative_base: &i64, position: usize, mode: Option<u32>) -> i64 {
    let parameter = input[position];

    let value = if mode == Some(1) {
        parameter
    } else if mode == Some(2) {
        input[(relative_base + parameter) as usize]
    } else {
        input[parameter as usize]
    };
    value
}

fn get_position(input: &Vec<i64>, relative_base: &i64, position: usize, mode: Option<u32>) -> i64 {
    let parameter = input[position];

    let value = if mode == Some(2) {
        // println!("relative_base, parameter {} {}", relative_base, parameter);
        relative_base + parameter
    } else if mode == Some(1) {
        panic!("Some 1")
    } else {
        parameter
    };
    value
}
