use permutohedron;

fn main() {
    let raw_input: Vec<i64> = vec![
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 38, 55, 64, 89, 114, 195, 276, 357, 438, 99999, 3,
        9, 101, 3, 9, 9, 102, 3, 9, 9, 1001, 9, 5, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 1002, 9, 3, 9,
        101, 5, 9, 9, 4, 9, 99, 3, 9, 101, 3, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 4, 9, 101, 5, 9, 9,
        1002, 9, 5, 9, 101, 5, 9, 9, 102, 3, 9, 9, 4, 9, 99, 3, 9, 101, 3, 9, 9, 1002, 9, 4, 9,
        101, 5, 9, 9, 102, 5, 9, 9, 1001, 9, 5, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101,
        2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4,
        9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9,
        1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001,
        9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4,
        9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9,
        102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2,
        9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9,
        3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001,
        9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2,
        9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3,
        9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9,
        2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9,
        4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9,
        101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9,
        9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 99,
    ];

    let mut opts: Vec<i64> = (0..=4).collect();

    let mut permutations = Vec::new();
    permutohedron::heap_recursive(&mut opts, |permutation| {
        permutations.push(permutation.to_vec())
    });
    // debug a single perm
    // let permutations = vec![vec![4, 3, 2, 1, 0]];
    let mut max_output: i64 = 0;
    for mut perm in permutations {
        println!("perm: {:?}", perm);
        let mut input: Vec<i64> = raw_input.clone();
        perm.reverse();
        let mut phase: Option<i64> = perm.pop();
        let mut opcode_3_input: i64 = 0;
        let mut opcode_3_inputs: Vec<i64>;
        match phase {
            Some(n) => opcode_3_inputs = vec![opcode_3_input, n],
            None => panic!("at the disco"),
        }

        let mut opcode: i64 = input[0];
        let mut position = 0;
        let mut mode_list: Vec<u32> = vec![];
        let mut output: i64 = 0; // gets set on first run

        loop {
            if opcode == 99 {
                // println!("---");
                phase = perm.pop();
                match phase {
                    Some(the_phase) => {
                        opcode_3_input = output;
                        opcode_3_inputs = vec![opcode_3_input, the_phase];
                        position = 0;
                        mode_list = vec![];
                        opcode = input[0];
                        input = raw_input.clone()
                    }
                    None => {
                        // println!("final output {:?}", output);
                        if output > max_output {
                            max_output = output;
                        }
                        break;
                    }
                }
            };
            // println!("opcode {}", opcode);
            // println!("input {:?}", input);
            match opcode {
                1 | 2 => {
                    let mode1 = mode_list.pop();
                    let mode2 = mode_list.pop();
                    let mode3 = 0;
                    let position_to_replace = input[position + 3];
                    let value_a = get_value(&input, position + 1 as usize, mode1);
                    let value_b = get_value(&input, position + 2 as usize, mode2);
                    // println!("insert into {}", position_to_replace);
                    // println!(
                    //     "op {} a (at {} in mode {:?}) {} b (at {} in mode {:?}) {}",
                    //     opcode,
                    //     position + 1,
                    //     mode1,
                    //     value_a,
                    //     position + 2,
                    //     mode2,
                    //     value_b
                    // );
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
                    let position_to_replace = input[position + 1];
                    // println!("inserting {} into {}", opcode_3_input, position_to_replace as usize);
                    input[position_to_replace as usize] = opcode_3_inputs.pop().unwrap();
                    position = position + 2;
                    opcode = input[position];
                }
                4 => {
                    let value = input[input[position + 1] as usize];
                    println!("phase {:?} input {:?} output {:?}", phase, output, value);
                    position = position + 2;
                    opcode = input[position];
                    output = value;
                }
                5 => {
                    let value = get_value(&input, position + 1 as usize, mode_list.pop());
                    let value2 = get_value(&input, position + 2 as usize, mode_list.pop());
                    if value != 0 {
                        // println!("inserting {} at {}", value2, position);
                        input[position] = value2;
                        position = value2 as usize
                    } else {
                        // jump
                        position = position + 3
                    }
                    opcode = input[position];
                    mode_list = vec![];
                }
                6 => {
                    let value = get_value(&input, position + 1 as usize, mode_list.pop());
                    let value2 = get_value(&input, position + 2 as usize, mode_list.pop());
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
                }
                7 => {
                    let value1 = get_value(&input, position + 1 as usize, mode_list.pop());
                    let value2 = get_value(&input, position + 2 as usize, mode_list.pop());
                    let value3 = get_value(&input, position + 3 as usize, Some(1));
                    println!("input3 {}", value3);
                    input[value3 as usize] = if value1 < value2 { 1 } else { 0 };
                    // jump if value3 is not instruction index (aka position)
                    position = position + 4;

                    opcode = input[position];
                    mode_list = vec![];
                }
                8 => {
                    let value1 = get_value(&input, position + 1 as usize, mode_list.pop());
                    let value2 = get_value(&input, position + 2 as usize, mode_list.pop());
                    let value3 = get_value(&input, position + 3 as usize, Some(1));
                    input[value3 as usize] = if value1 == value2 { 1 } else { 0 };
                    // TODO: jump if value3 is not instruction index (aka position)
                    position = position + 4;
                    opcode = input[position];
                    mode_list = vec![];
                }
                x => {
                    // println!("processing {}", x);
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
    }
    println!("max_output: {}", max_output);
}

fn get_value(input: &Vec<i64>, position: usize, mode: Option<u32>) -> i64 {
    // println!(
    //     "mode {}",
    //     match mode {
    //         Some(x) => x.to_string(),
    //         None => "none".to_string(),
    //     }
    // );
    let position = input[position];
    let value = if mode == Some(1) {
        position
    } else {
        input[position as usize]
    };
    value
}
