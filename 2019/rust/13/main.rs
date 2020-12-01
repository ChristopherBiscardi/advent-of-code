use std::collections::BTreeMap;

fn main() {
    let opcode_3_input = 2;
    let mut input: Vec<i64> = vec![
        1, 380, 379, 385, 1008, 2239, 817542, 381, 1005, 381, 12, 99, 109, 2240, 1102, 0, 1, 383,
        1101, 0, 0, 382, 21001, 382, 0, 1, 21001, 383, 0, 2, 21101, 0, 37, 0, 1106, 0, 578, 4, 382,
        4, 383, 204, 1, 1001, 382, 1, 382, 1007, 382, 40, 381, 1005, 381, 22, 1001, 383, 1, 383,
        1007, 383, 20, 381, 1005, 381, 18, 1006, 385, 69, 99, 104, -1, 104, 0, 4, 386, 3, 384,
        1007, 384, 0, 381, 1005, 381, 94, 107, 0, 384, 381, 1005, 381, 108, 1106, 0, 161, 107, 1,
        392, 381, 1006, 381, 161, 1101, -1, 0, 384, 1105, 1, 119, 1007, 392, 38, 381, 1006, 381,
        161, 1101, 1, 0, 384, 21002, 392, 1, 1, 21102, 1, 18, 2, 21101, 0, 0, 3, 21102, 138, 1, 0,
        1105, 1, 549, 1, 392, 384, 392, 21001, 392, 0, 1, 21102, 1, 18, 2, 21102, 3, 1, 3, 21101,
        0, 161, 0, 1106, 0, 549, 1102, 0, 1, 384, 20001, 388, 390, 1, 20101, 0, 389, 2, 21101, 180,
        0, 0, 1106, 0, 578, 1206, 1, 213, 1208, 1, 2, 381, 1006, 381, 205, 20001, 388, 390, 1,
        21002, 389, 1, 2, 21101, 205, 0, 0, 1106, 0, 393, 1002, 390, -1, 390, 1101, 0, 1, 384,
        21001, 388, 0, 1, 20001, 389, 391, 2, 21101, 228, 0, 0, 1105, 1, 578, 1206, 1, 261, 1208,
        1, 2, 381, 1006, 381, 253, 21001, 388, 0, 1, 20001, 389, 391, 2, 21101, 253, 0, 0, 1105, 1,
        393, 1002, 391, -1, 391, 1101, 0, 1, 384, 1005, 384, 161, 20001, 388, 390, 1, 20001, 389,
        391, 2, 21102, 279, 1, 0, 1106, 0, 578, 1206, 1, 316, 1208, 1, 2, 381, 1006, 381, 304,
        20001, 388, 390, 1, 20001, 389, 391, 2, 21102, 1, 304, 0, 1105, 1, 393, 1002, 390, -1, 390,
        1002, 391, -1, 391, 1102, 1, 1, 384, 1005, 384, 161, 21002, 388, 1, 1, 21002, 389, 1, 2,
        21102, 0, 1, 3, 21102, 338, 1, 0, 1106, 0, 549, 1, 388, 390, 388, 1, 389, 391, 389, 20102,
        1, 388, 1, 20102, 1, 389, 2, 21102, 4, 1, 3, 21101, 365, 0, 0, 1105, 1, 549, 1007, 389, 19,
        381, 1005, 381, 75, 104, -1, 104, 0, 104, 0, 99, 0, 1, 0, 0, 0, 0, 0, 0, 173, 18, 15, 1, 1,
        20, 109, 3, 22102, 1, -2, 1, 22102, 1, -1, 2, 21102, 0, 1, 3, 21102, 414, 1, 0, 1106, 0,
        549, 21201, -2, 0, 1, 21202, -1, 1, 2, 21101, 429, 0, 0, 1105, 1, 601, 2102, 1, 1, 435, 1,
        386, 0, 386, 104, -1, 104, 0, 4, 386, 1001, 387, -1, 387, 1005, 387, 451, 99, 109, -3,
        2105, 1, 0, 109, 8, 22202, -7, -6, -3, 22201, -3, -5, -3, 21202, -4, 64, -2, 2207, -3, -2,
        381, 1005, 381, 492, 21202, -2, -1, -1, 22201, -3, -1, -3, 2207, -3, -2, 381, 1006, 381,
        481, 21202, -4, 8, -2, 2207, -3, -2, 381, 1005, 381, 518, 21202, -2, -1, -1, 22201, -3, -1,
        -3, 2207, -3, -2, 381, 1006, 381, 507, 2207, -3, -4, 381, 1005, 381, 540, 21202, -4, -1,
        -1, 22201, -3, -1, -3, 2207, -3, -4, 381, 1006, 381, 529, 21201, -3, 0, -7, 109, -8, 2105,
        1, 0, 109, 4, 1202, -2, 40, 566, 201, -3, 566, 566, 101, 639, 566, 566, 1201, -1, 0, 0,
        204, -3, 204, -2, 204, -1, 109, -4, 2105, 1, 0, 109, 3, 1202, -1, 40, 593, 201, -2, 593,
        593, 101, 639, 593, 593, 21001, 0, 0, -2, 109, -3, 2105, 1, 0, 109, 3, 22102, 20, -2, 1,
        22201, 1, -1, 1, 21101, 409, 0, 2, 21102, 555, 1, 3, 21101, 800, 0, 4, 21101, 0, 630, 0,
        1105, 1, 456, 21201, 1, 1439, -2, 109, -3, 2106, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 2, 0, 0, 2, 2, 2, 2, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0,
        0, 2, 2, 0, 2, 2, 2, 0, 0, 0, 2, 2, 0, 2, 2, 2, 0, 1, 1, 0, 2, 2, 0, 0, 0, 0, 2, 0, 2, 2,
        2, 0, 2, 0, 0, 2, 0, 2, 0, 0, 2, 2, 0, 0, 0, 0, 2, 2, 2, 0, 2, 0, 2, 0, 0, 0, 0, 1, 1, 0,
        0, 0, 0, 2, 2, 0, 0, 2, 2, 0, 2, 2, 0, 2, 2, 0, 0, 2, 0, 2, 0, 2, 2, 0, 0, 2, 2, 0, 0, 0,
        2, 2, 0, 0, 0, 2, 0, 1, 1, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0,
        0, 0, 2, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 2, 0, 2, 0, 1, 1, 0, 0, 2, 0, 0, 0, 2, 0, 0, 2, 2,
        2, 0, 0, 2, 2, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 2, 2, 0, 0, 2, 0, 0, 0, 0, 0, 0, 1, 1, 0,
        2, 2, 0, 2, 0, 0, 0, 0, 2, 2, 2, 2, 0, 2, 2, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 2, 2, 0,
        0, 0, 0, 0, 0, 2, 0, 1, 1, 0, 2, 2, 0, 2, 0, 2, 2, 2, 2, 2, 2, 0, 2, 0, 0, 0, 2, 2, 0, 2,
        0, 0, 0, 2, 0, 0, 0, 2, 0, 2, 0, 2, 2, 2, 0, 2, 0, 1, 1, 0, 0, 0, 2, 0, 2, 0, 0, 2, 0, 0,
        0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 2, 0, 2, 0, 0, 0, 1, 1, 0,
        2, 2, 0, 0, 0, 0, 0, 0, 2, 0, 2, 2, 0, 2, 2, 2, 0, 2, 0, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 1, 1, 0, 0, 2, 2, 0, 0, 2, 2, 0, 0, 2, 2, 2, 0, 2, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 1, 1, 0, 0, 0, 0, 2, 2, 0, 2, 0, 0, 2,
        0, 0, 2, 0, 2, 2, 0, 0, 0, 0, 0, 2, 0, 0, 2, 0, 2, 0, 2, 2, 0, 0, 0, 0, 0, 2, 0, 1, 1, 0,
        0, 2, 0, 0, 2, 0, 0, 0, 2, 2, 0, 0, 0, 2, 0, 2, 0, 2, 2, 2, 0, 0, 0, 0, 2, 2, 0, 0, 2, 2,
        0, 2, 2, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 1, 46, 64, 87, 88, 13, 29, 59, 29, 27, 55, 47, 45, 47, 73, 76, 52, 56,
        23, 40, 18, 90, 67, 59, 33, 11, 88, 73, 62, 73, 30, 64, 64, 6, 90, 35, 9, 69, 17, 75, 95,
        52, 30, 90, 47, 32, 82, 66, 6, 27, 90, 54, 25, 11, 12, 29, 1, 66, 34, 85, 26, 57, 50, 62,
        70, 1, 84, 30, 52, 52, 26, 58, 88, 3, 35, 9, 90, 8, 95, 21, 70, 7, 27, 88, 23, 19, 28, 28,
        17, 4, 24, 71, 35, 34, 38, 70, 59, 47, 22, 80, 63, 98, 20, 35, 37, 81, 61, 8, 33, 76, 98,
        7, 55, 90, 80, 51, 10, 47, 90, 33, 36, 10, 26, 45, 14, 56, 71, 56, 95, 60, 28, 32, 15, 84,
        5, 74, 8, 98, 15, 58, 30, 33, 42, 75, 96, 83, 62, 78, 45, 3, 48, 24, 9, 15, 7, 36, 37, 4,
        15, 37, 86, 60, 89, 30, 98, 7, 78, 91, 28, 82, 78, 80, 91, 90, 44, 77, 4, 88, 23, 51, 64,
        58, 70, 96, 33, 32, 13, 50, 47, 84, 7, 41, 1, 79, 85, 95, 30, 84, 53, 23, 78, 97, 4, 26,
        97, 51, 78, 46, 78, 55, 48, 86, 66, 72, 60, 2, 37, 81, 52, 9, 7, 87, 13, 60, 38, 45, 78,
        46, 9, 72, 42, 12, 15, 61, 68, 77, 80, 81, 37, 10, 29, 29, 6, 22, 63, 12, 38, 87, 57, 47,
        31, 21, 18, 33, 73, 69, 85, 95, 79, 58, 81, 89, 1, 97, 53, 64, 82, 35, 60, 48, 70, 28, 76,
        73, 3, 58, 22, 92, 97, 98, 68, 51, 15, 5, 51, 51, 41, 55, 53, 46, 9, 74, 40, 46, 14, 4, 22,
        63, 84, 52, 14, 77, 10, 6, 11, 31, 58, 91, 27, 71, 5, 43, 70, 54, 90, 12, 67, 50, 56, 5, 9,
        86, 60, 91, 74, 47, 89, 37, 63, 11, 69, 84, 12, 36, 3, 67, 75, 85, 34, 17, 63, 23, 18, 44,
        34, 42, 94, 52, 16, 17, 57, 89, 98, 54, 52, 2, 32, 46, 16, 9, 31, 92, 36, 10, 78, 31, 70,
        1, 84, 17, 61, 85, 36, 11, 23, 96, 12, 79, 46, 34, 14, 94, 82, 11, 12, 79, 9, 41, 95, 38,
        33, 41, 12, 63, 74, 67, 6, 71, 12, 6, 11, 58, 56, 79, 57, 32, 47, 12, 50, 12, 70, 33, 60,
        49, 30, 36, 86, 10, 61, 3, 51, 52, 25, 24, 71, 55, 21, 96, 7, 36, 20, 38, 28, 93, 87, 66,
        54, 5, 45, 98, 11, 55, 11, 14, 7, 56, 62, 51, 51, 36, 71, 62, 81, 79, 44, 79, 51, 3, 57,
        25, 21, 60, 1, 56, 3, 21, 67, 64, 86, 65, 55, 23, 14, 90, 7, 84, 25, 86, 63, 70, 32, 50,
        82, 47, 8, 18, 35, 43, 88, 2, 31, 7, 1, 23, 3, 44, 70, 93, 90, 59, 41, 58, 85, 84, 37, 16,
        65, 69, 61, 59, 29, 68, 87, 58, 8, 31, 52, 47, 92, 60, 18, 48, 82, 24, 92, 50, 27, 38, 56,
        11, 56, 70, 5, 17, 48, 11, 60, 13, 93, 33, 36, 47, 42, 65, 82, 48, 11, 68, 67, 18, 59, 64,
        91, 82, 83, 72, 60, 42, 47, 16, 62, 62, 16, 15, 1, 49, 12, 41, 62, 45, 47, 50, 43, 56, 18,
        5, 76, 28, 61, 56, 75, 18, 35, 97, 43, 34, 20, 17, 23, 40, 69, 58, 75, 46, 65, 13, 12, 58,
        10, 11, 60, 29, 41, 22, 95, 43, 44, 37, 85, 42, 53, 28, 81, 67, 18, 69, 83, 10, 77, 25, 51,
        9, 53, 1, 20, 75, 10, 93, 73, 87, 16, 15, 40, 54, 12, 45, 59, 59, 32, 29, 91, 7, 58, 79,
        57, 63, 5, 36, 83, 17, 95, 2, 96, 84, 33, 18, 77, 1, 48, 28, 52, 80, 74, 58, 4, 5, 75, 2,
        82, 46, 16, 45, 25, 42, 24, 4, 21, 78, 75, 32, 1, 12, 55, 6, 17, 30, 82, 9, 41, 63, 13, 27,
        41, 35, 84, 16, 82, 13, 75, 51, 70, 52, 33, 19, 94, 87, 75, 25, 78, 49, 11, 97, 67, 68, 88,
        19, 84, 79, 55, 95, 80, 15, 37, 97, 18, 91, 17, 43, 95, 22, 14, 4, 97, 83, 69, 87, 4, 72,
        48, 35, 9, 37, 76, 2, 85, 84, 27, 18, 97, 68, 67, 90, 84, 65, 87, 91, 74, 16, 98, 23, 31,
        30, 88, 92, 26, 67, 4, 46, 40, 7, 52, 72, 44, 68, 1, 97, 62, 96, 49, 33, 76, 94, 84, 79,
        45, 18, 47, 54, 77, 83, 52, 32, 86, 40, 61, 75, 64, 30, 23, 21, 76, 817542,
    ];
    for i in 0..(input.len() * 10) {
        input.push(0)
    }
    let mut relative_base = 0;
    let mut opcode: i64 = input[0];
    let mut position = 0;
    let mut mode_list: Vec<u32> = vec![];

    let mut screen = Screen::new();

    loop {
        if opcode == 99 {
            break;
        };
        // println!("input {:?}", input);
        // println!("opcode {}", opcode);
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
                // println!("op4: {}", value);
                screen.run(value);
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
    screen.print_grid();
    let blocks_left = screen
        .grid
        .values()
        .filter(|n| **n == 2 as i64)
        .collect::<Vec<&i64>>()
        .len();
    println!("Blocks Left: {}", blocks_left);
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

struct Screen {
    // 0 or not found is black, 1 is white
    grid: BTreeMap<(i64, i64), i64>,
    current_instruction: Vec<i64>,
}

impl Screen {
    fn new() -> Screen {
        let mut map = BTreeMap::new();
        // map.insert((0, 0), 1);
        Screen {
            grid: map,
            current_instruction: vec![],
        }
    }
    fn run(&mut self, value: i64) {
        self.current_instruction.push(value);
        if self.current_instruction.len() == 3 {
            self.grid.insert(
                (self.current_instruction[0], self.current_instruction[1]),
                self.current_instruction[2],
            );
            self.current_instruction = vec![];
        }
    }
    fn print_grid(&self) {
        let mut xmin: i64 = 0;
        let mut xmax: i64 = 0;
        let mut ymin: i64 = 0;
        let mut ymax: i64 = 0;
        for (x, y) in self.grid.keys() {
            if x < &xmin {
                xmin = *x;
            }
            if x > &xmax {
                xmax = *x;
            }
            if y < &ymin {
                ymin = *y;
            }
            if y > &ymax {
                ymax = *y;
            }
        }
        let xaxis = xmax - xmin;
        let yaxis = ymax - ymin;
        // println!("paint_tile_count {}", self.paint_tile_count);
        // println!("grid size {}x{}", xmax - xmin, ymax - ymin);
        // println!("grid {:?}", self.grid);
        for y in (0..=yaxis).rev() {
            // print!(":");
            for x in 0..=xaxis {
                match self.grid.get(&(xmin + x, ymin + y)) {
                    Some(n) => match n {
                        0 => print!(" "),
                        n => print!("{}", n),
                        //                        _ => panic!("shouldnt"),
                    },
                    None => print!(" "),
                }
            }
            print!("\n");
        }
    }
}
