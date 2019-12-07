fn main() {
    let opcode_3_input = 5;
    let mut input: Vec<i64> = vec![
3,225,1,225,6,6,1100,1,238,225,104,0,101,71,150,224,101,-123,224,224,4,224,102,8,223,223,101,2,224,224,1,224,223,223,2,205,209,224,1001,224,-3403,224,4,224,1002,223,8,223,101,1,224,224,1,223,224,223,1101,55,24,224,1001,224,-79,224,4,224,1002,223,8,223,101,1,224,224,1,223,224,223,1,153,218,224,1001,224,-109,224,4,224,1002,223,8,223,101,5,224,224,1,224,223,223,1002,201,72,224,1001,224,-2088,224,4,224,102,8,223,223,101,3,224,224,1,223,224,223,1102,70,29,225,102,5,214,224,101,-250,224,224,4,224,1002,223,8,223,1001,224,3,224,1,223,224,223,1101,12,52,225,1101,60,71,225,1001,123,41,224,1001,224,-111,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1102,78,66,224,1001,224,-5148,224,4,224,1002,223,8,223,1001,224,2,224,1,223,224,223,1101,29,77,225,1102,41,67,225,1102,83,32,225,1101,93,50,225,1102,53,49,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1107,677,677,224,1002,223,2,223,1005,224,329,101,1,223,223,7,677,677,224,1002,223,2,223,1005,224,344,1001,223,1,223,7,226,677,224,102,2,223,223,1006,224,359,101,1,223,223,1108,226,226,224,1002,223,2,223,1005,224,374,1001,223,1,223,8,226,677,224,1002,223,2,223,1006,224,389,1001,223,1,223,1108,226,677,224,1002,223,2,223,1006,224,404,101,1,223,223,1107,677,226,224,102,2,223,223,1006,224,419,101,1,223,223,1007,677,677,224,1002,223,2,223,1005,224,434,101,1,223,223,7,677,226,224,102,2,223,223,1006,224,449,1001,223,1,223,1008,226,677,224,1002,223,2,223,1006,224,464,101,1,223,223,8,677,677,224,1002,223,2,223,1006,224,479,101,1,223,223,108,226,226,224,102,2,223,223,1005,224,494,101,1,223,223,1107,226,677,224,1002,223,2,223,1006,224,509,101,1,223,223,107,226,226,224,1002,223,2,223,1006,224,524,1001,223,1,223,107,677,677,224,1002,223,2,223,1005,224,539,101,1,223,223,1007,226,226,224,102,2,223,223,1006,224,554,101,1,223,223,108,677,677,224,102,2,223,223,1005,224,569,101,1,223,223,107,677,226,224,102,2,223,223,1005,224,584,101,1,223,223,1008,226,226,224,102,2,223,223,1006,224,599,101,1,223,223,1108,677,226,224,1002,223,2,223,1006,224,614,101,1,223,223,8,677,226,224,102,2,223,223,1005,224,629,1001,223,1,223,1008,677,677,224,102,2,223,223,1006,224,644,101,1,223,223,1007,226,677,224,102,2,223,223,1005,224,659,101,1,223,223,108,226,677,224,102,2,223,223,1006,224,674,101,1,223,223,4,223,99,226
// 3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9
];
    let mut opcode: i64 = input[0];
    let mut position = 0;
    let mut mode_list: Vec<u32> = vec![];

    loop {
        if opcode == 99 { break; };
        println!("opcode {}", opcode);
        match opcode {
            1 | 2 => {
                let mode1 = mode_list.pop();
                let mode2 = mode_list.pop();
                let mode3 = 0;
                let position_to_replace = input[position+3];
                let value_a = get_value(&input, position+1 as usize, mode1);
                let value_b = get_value(&input, position+2 as usize, mode2);

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
            },
            3 => {
                let position_to_replace = input[position+1];
                input[position_to_replace as usize] = opcode_3_input;
                if position_to_replace == position as i64 {
                    println!("asfkjsgfjsgkljsdlgfkjksdl")
                }
                position = position + 2;
                opcode = input[position];
            }
            4 => {
                let value = input[input[position+1]as usize];
                println!("op4: {}", value);
                position = position + 2;
                opcode = input[position];
            },
            5 => {
                let value = get_value(&input, position+1 as usize, mode_list.pop());
                let value2 = get_value(&input, position+2 as usize, mode_list.pop());
                if value != 0 {
                    println!("inserting {} at {}", value2, position);
                    input[position] = value2;
                    position = value2 as usize
                } else {
                    // jump
                    position = position + 3
                }
                opcode = input[position];
                mode_list = vec![];
            },
            6 => {
                let value = get_value(&input, position+1 as usize, mode_list.pop());
                let value2 = get_value(&input, position+2 as usize, mode_list.pop());
                println!("value: {} {}", value, value2);
                if value == 0 {
                    input[position] = value2;
                    position = value2 as usize;
                    println!("position {}", position)
                } else {
                    // jump
                    position = position + 3
                }
                // println!("inputPos {}", input[position]);
                opcode = input[position];
                mode_list = vec![];
            },
            7 => {
                let value1 = get_value(&input, position+1 as usize, mode_list.pop());
                let value2 = get_value(&input, position+2 as usize, mode_list.pop());
                let value3 = get_value(&input, position+3 as usize, Some(1));
                println!("input3 {}", value3);
                input[value3 as usize] = if value1 < value2 { 1 } else { 0 };
                // jump if value3 is not instruction index (aka position)
                position = position + 4;
                
                opcode = input[position];
                mode_list = vec![];
            },
            8 => {
                let value1 = get_value(&input, position+1 as usize, mode_list.pop());
                let value2 = get_value(&input, position+2 as usize, mode_list.pop());
                let value3 = get_value(&input, position+3 as usize, Some(1));
                input[value3 as usize] = if value1 == value2 { 1 } else { 0 };
                // TODO: jump if value3 is not instruction index (aka position)
                position = position + 4;
                opcode = input[position];
                mode_list = vec![];
            },
            x => {
                let  mut digits: Vec<u32> = x.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
                let one = digits.pop();
                let two = digits.pop();
                let _unused: Option<i64> = one.and_then(|t| match two {
                    Some(n) => {
                        opcode = (n * 10 + t) as i64;
                        None
                    },
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

fn get_value(input: &Vec<i64>, position: usize, mode: Option<u32>) -> i64 {
    println!("mode {}", match mode {
        Some(x) => x.to_string(),
        None => "none".to_string()
    });
    let position = input[position];
    let value = if mode == Some(1) { position } else { input[position as usize] };
    value
}