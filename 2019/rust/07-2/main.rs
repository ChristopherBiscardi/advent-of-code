use permutohedron;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

fn main() {
    let raw_input: Vec<i64> = vec![
        3,8,1001,8,10,8,105,1,0,0,21,38,55,64,89,114,195,276,357,438,99999,3,9,101,3,9,9,102,3,9,9,1001,9,5,9,4,9,99,3,9,101,2,9,9,1002,9,3,9,101,5,9,9,4,9,99,3,9,101,3,9,9,4,9,99,3,9,1002,9,4,9,101,5,9,9,1002,9,5,9,101,5,9,9,102,3,9,9,4,9,99,3,9,101,3,9,9,1002,9,4,9,101,5,9,9,102,5,9,9,1001,9,5,9,4,9,99,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,99
    ];

    let mut opts: Vec<i64> = (5..=9).collect();

    let mut permutations = Vec::new();
    permutohedron::heap_recursive(&mut opts, |permutation| {
        permutations.push(permutation.to_vec())
    });
    // debug a single perm
    // let permutations = vec![vec![9, 8, 7, 6, 5]];

    // let mut rx: Receiver<i64>;
    let (tx2a, rx): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    // rx = rx2;
    for mut perm in permutations {
        println!("perm: {:?}", perm);
        // reverse is a bug
        // perm.reverse();
        let (first_amp_tx, amp_rx) = mpsc::channel();
        let mut cur_amp_rx: Option<Receiver<i64>> = Some(amp_rx);

        let last = perm.len();
        for (i, p) in perm.iter().enumerate() {
            let phase = p.clone();
            if i == last - 1 {
                let tx1 = mpsc::Sender::clone(&first_amp_tx);
                let tx2_2 = mpsc::Sender::clone(&tx2a);
                let new_input = raw_input.clone();
                match cur_amp_rx.take() {
                    Some(rxx) => {
                        thread::Builder::new().name("E".to_string()).spawn(move || {
                            run_amplifier(new_input, rxx, tx1, phase, Some(tx2_2));
                        });
                        ();
                    }
                    None => panic!("eek"),
                }
            } else {
                let (tx2, rx2) = mpsc::channel();
                let new_input = raw_input.clone();
                match cur_amp_rx.take() {
                    Some(rxx) => {
                        thread::Builder::new()
                            .name("child1".to_string())
                            .spawn(move || {
                                run_amplifier(new_input, rxx, tx2, phase, None);
                            });
                        ();
                    }
                    None => panic!("more disco"),
                }

                cur_amp_rx = Some(rx2);
            }
        }

        let mut opcode_3_input: i64 = 0;
        first_amp_tx.send(opcode_3_input);

        // let mut phase: Option<i64> = perm.pop();
    }
    let mut max_output: i64 = 0;
    for received in &rx {
        if (received > max_output) {
            max_output = received;
            println!("---");
            println!("new max: {}", received);
            println!("---");
        }
        print!("rec")
    }
    println!("max_output: {}", max_output);
}

fn get_value(input: &Vec<i64>, position: usize, mode: Option<u32>) -> i64 {
    let position = input[position];
    let value = if mode == Some(1) {
        position
    } else {
        input[position as usize]
    };
    value
}

fn run_amplifier(
    mut input: Vec<i64>,
    rx: Receiver<i64>,
    tx: Sender<i64>,
    phase: i64,
    mut last_amp_tx: Option<Sender<i64>>,
) {
    let mut phase_used: bool = false;
    let mut tx2: Sender<i64>;

    let mut opcode: i64 = input[0];
    let mut position = 0;
    let mut mode_list: Vec<u32> = vec![];
    // gets set on first run
    let mut output: i64 = 0;
    loop {
        if opcode == 99 {
            match last_amp_tx.take() {
                Some(tx4) => {
                    tx4.send(output).expect("could not send");
                    ()
                }
                None => (),
            }

            // if is_last_amp {
            //     // println!("final output {:?}", output);
            // } else {
            //     // println!("finished phase {}", phase);
            // }
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

                // if phase the use phase
                if !phase_used {
                    // println!(
                    //     "phase {} inserting {} into {}",
                    //     phase, phase, position_to_replace as usize
                    // );
                    input[position_to_replace as usize] = phase;
                    phase_used = true;
                } else {
                    let value = rx.recv().unwrap();
                    // println!(
                    //     "phase {} inserting {} into {}",
                    //     phase, value, position_to_replace as usize
                    // );
                    input[position_to_replace as usize] = value;
                }
                position = position + 2;
                opcode = input[position];
            }
            4 => {
                let value = get_value(&input, position + 1 as usize, mode_list.pop());
                // let value = input[input[position + 1] as usize];
                // println!("phase {:?} input {:?} output {:?}", phase, output, value);
                position = position + 2;
                opcode = input[position];
                output = value;
                tx.send(output).unwrap();
                mode_list = vec![];
            }
            5 => {
                let value = get_value(&input, position + 1 as usize, mode_list.pop());
                let value2 = get_value(&input, position + 2 as usize, mode_list.pop());
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
                let value = get_value(&input, position + 1 as usize, mode_list.pop());
                let value2 = get_value(&input, position + 2 as usize, mode_list.pop());
                // println!("value: {} {}", value, value2);
                if value == 0 {
                    // input[position] = value2;
                    position = value2 as usize;
                // println!("position {}", position)
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
                // println!("input3 {}", value3);
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
