use std::collections::BTreeMap;

fn test() {
    let mut robot = Robot::new();
    robot.run(1);
    robot.run(0);

    robot.run(0);
    robot.run(0);

    robot.run(1);
    robot.run(0);
    robot.run(1);
    robot.run(0);

    robot.run(0);
    robot.run(1);
    robot.run(1);
    robot.run(0);
    robot.run(1);
    robot.run(0);

    println!(
        "grid {:?} count {:?}",
        robot.grid.len(),
        robot.paint_tile_count
    );
    robot.print_grid();
    for key in robot.grid {
        println!("{:?}", key);
    }
}

fn main() {
    let mut input: Vec<i64> = vec![
        3,
        8,
        1005,
        8,
        310,
        1106,
        0,
        11,
        0,
        0,
        0,
        104,
        1,
        104,
        0,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        108,
        1,
        8,
        10,
        4,
        10,
        1002,
        8,
        1,
        28,
        1,
        105,
        11,
        10,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        1008,
        8,
        0,
        10,
        4,
        10,
        102,
        1,
        8,
        55,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        108,
        0,
        8,
        10,
        4,
        10,
        1001,
        8,
        0,
        76,
        3,
        8,
        1002,
        8,
        -1,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        108,
        0,
        8,
        10,
        4,
        10,
        102,
        1,
        8,
        98,
        1,
        1004,
        7,
        10,
        1006,
        0,
        60,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        108,
        0,
        8,
        10,
        4,
        10,
        1002,
        8,
        1,
        127,
        2,
        1102,
        4,
        10,
        1,
        1108,
        7,
        10,
        2,
        1102,
        4,
        10,
        2,
        101,
        18,
        10,
        3,
        8,
        1002,
        8,
        -1,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        1008,
        8,
        0,
        10,
        4,
        10,
        102,
        1,
        8,
        166,
        1006,
        0,
        28,
        3,
        8,
        1002,
        8,
        -1,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        108,
        1,
        8,
        10,
        4,
        10,
        101,
        0,
        8,
        190,
        1006,
        0,
        91,
        1,
        1108,
        5,
        10,
        3,
        8,
        1002,
        8,
        -1,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        1008,
        8,
        1,
        10,
        4,
        10,
        1002,
        8,
        1,
        220,
        1,
        1009,
        14,
        10,
        2,
        1103,
        19,
        10,
        2,
        1102,
        9,
        10,
        2,
        1007,
        4,
        10,
        3,
        8,
        1002,
        8,
        -1,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        1008,
        8,
        1,
        10,
        4,
        10,
        101,
        0,
        8,
        258,
        2,
        3,
        0,
        10,
        1006,
        0,
        4,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        108,
        1,
        8,
        10,
        4,
        10,
        1001,
        8,
        0,
        286,
        1006,
        0,
        82,
        101,
        1,
        9,
        9,
        1007,
        9,
        1057,
        10,
        1005,
        10,
        15,
        99,
        109,
        632,
        104,
        0,
        104,
        1,
        21102,
        1,
        838479487636,
        1,
        21102,
        327,
        1,
        0,
        1106,
        0,
        431,
        21102,
        1,
        932813579156,
        1,
        21102,
        1,
        338,
        0,
        1106,
        0,
        431,
        3,
        10,
        104,
        0,
        104,
        1,
        3,
        10,
        104,
        0,
        104,
        0,
        3,
        10,
        104,
        0,
        104,
        1,
        3,
        10,
        104,
        0,
        104,
        1,
        3,
        10,
        104,
        0,
        104,
        0,
        3,
        10,
        104,
        0,
        104,
        1,
        21101,
        0,
        179318033447,
        1,
        21101,
        385,
        0,
        0,
        1105,
        1,
        431,
        21101,
        248037678275,
        0,
        1,
        21101,
        0,
        396,
        0,
        1105,
        1,
        431,
        3,
        10,
        104,
        0,
        104,
        0,
        3,
        10,
        104,
        0,
        104,
        0,
        21101,
        0,
        709496558348,
        1,
        21102,
        419,
        1,
        0,
        1105,
        1,
        431,
        21101,
        825544561408,
        0,
        1,
        21101,
        0,
        430,
        0,
        1106,
        0,
        431,
        99,
        109,
        2,
        22101,
        0,
        -1,
        1,
        21101,
        40,
        0,
        2,
        21102,
        462,
        1,
        3,
        21101,
        0,
        452,
        0,
        1106,
        0,
        495,
        109,
        -2,
        2105,
        1,
        0,
        0,
        1,
        0,
        0,
        1,
        109,
        2,
        3,
        10,
        204,
        -1,
        1001,
        457,
        458,
        473,
        4,
        0,
        1001,
        457,
        1,
        457,
        108,
        4,
        457,
        10,
        1006,
        10,
        489,
        1101,
        0,
        0,
        457,
        109,
        -2,
        2106,
        0,
        0,
        0,
        109,
        4,
        2101,
        0,
        -1,
        494,
        1207,
        -3,
        0,
        10,
        1006,
        10,
        512,
        21101,
        0,
        0,
        -3,
        22101,
        0,
        -3,
        1,
        22101,
        0,
        -2,
        2,
        21101,
        1,
        0,
        3,
        21102,
        531,
        1,
        0,
        1105,
        1,
        536,
        109,
        -4,
        2105,
        1,
        0,
        109,
        5,
        1207,
        -3,
        1,
        10,
        1006,
        10,
        559,
        2207,
        -4,
        -2,
        10,
        1006,
        10,
        559,
        22101,
        0,
        -4,
        -4,
        1106,
        0,
        627,
        21202,
        -4,
        1,
        1,
        21201,
        -3,
        -1,
        2,
        21202,
        -2,
        2,
        3,
        21102,
        578,
        1,
        0,
        1105,
        1,
        536,
        22101,
        0,
        1,
        -4,
        21101,
        1,
        0,
        -1,
        2207,
        -4,
        -2,
        10,
        1006,
        10,
        597,
        21102,
        0,
        1,
        -1,
        22202,
        -2,
        -1,
        -2,
        2107,
        0,
        -3,
        10,
        1006,
        10,
        619,
        21201,
        -1,
        0,
        1,
        21102,
        1,
        619,
        0,
        105,
        1,
        494,
        21202,
        -2,
        -1,
        -2,
        22201,
        -4,
        -2,
        -4,
        109,
        -5,
        2106,
        0,
        0,
    ];
    for _i in 0..(input.len() * 10) {
        input.push(0)
    }
    let mut relative_base = 0;
    let mut opcode: i64 = input[0];
    let mut position = 0;
    let mut mode_list: Vec<u32> = vec![];

    // // 0 or not found is black, 1 is white
    // let mut grid: BTreeMap<(i64, i64), i64> = BTreeMap::new();
    // let mut cur_pos: (i64, i64) = (0, 0);
    // let mut output_count = 0;
    // // pointing up
    // let mut direction: i64 = 0;
    let mut robot = Robot::new();
    // let mut count = 0;
    loop {
        if opcode == 99 {
            println!("HALT");
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
                // count = count + 1;
                let opcode_3_input = robot.get_current_tile();
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
                robot.run(value);

                // end paint panel
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
                // println!("input3 {}", value3);
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
    println!(
        "grid {:?} count {:?}",
        robot.grid.len(),
        robot.paint_tile_count
    );
    robot.print_grid();
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

struct Robot {
    // 0 or not found is black, 1 is white
    grid: BTreeMap<(i64, i64), i64>,
    cur_pos: (i64, i64),
    output_count: i64,
    // pointing up
    direction: i64,
    paint_tile_count: i64,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            grid: BTreeMap::new(),
            cur_pos: (0, 0),
            output_count: 0,
            direction: 0,
            paint_tile_count: 0,
        }
    }
    fn get_current_tile(&self) -> i64 {
        let grid_cell = self.grid.get(&(self.cur_pos.0, self.cur_pos.1));
        let cell_color = match grid_cell {
            Some(n) => {
                // println!("op3 {}", n);
                *n
            }
            None => {
                // println!("op3 {}", 0);
                0
            }
        };
        cell_color
    }
    fn run(&mut self, value: i64) {
        // let value = input[input[position+1]as usize];
        // paint panel
        if self.output_count == 0 {
            self.paint_tile_count = self.paint_tile_count + 1;
            self.grid.insert(self.cur_pos, value);
            // print!("insert {:?} {:?}", self.cur_pos, value);
            self.output_count = 1;
        } else {
            // println!("turning {}", value);
            match value {
                0 => self.direction = self.direction - 90,
                1 => self.direction = self.direction + 90,
                _ => panic!("weird"),
            };
            // println!("direction {}", (self.direction % 360));
            match (self.direction % 360) {
                0 => {
                    self.cur_pos = (self.cur_pos.0, self.cur_pos.1 + 1);
                    // println!("face up");
                }
                90 => {
                    self.cur_pos = (self.cur_pos.0 + 1, self.cur_pos.1);
                    // println!("face right");
                }
                180 => {
                    self.cur_pos = (self.cur_pos.0, self.cur_pos.1 - 1);
                    // println!("face down");
                }
                270 => {
                    self.cur_pos = (self.cur_pos.0 - 1, self.cur_pos.1);
                    // println!("face left");
                }
                -90 => {
                    self.cur_pos = (self.cur_pos.0 - 1, self.cur_pos.1);
                    // println!("face left");
                }
                -180 => {
                    self.cur_pos = (self.cur_pos.0, self.cur_pos.1 - 1);
                    // println!("face down");
                }
                -270 => {
                    self.cur_pos = (self.cur_pos.0 + 1, self.cur_pos.1);
                    // println!("face right");
                }
                _ => panic!("weird 2"),
            }
            self.output_count = 0;
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
        println!("paint_tile_count {}", self.paint_tile_count);
        println!("grid size {}x{}", xmax - xmin, ymax - ymin);
        for y in 0..yaxis {
            for x in 0..xaxis {
                match self.grid.get(&(x, y)) {
                    Some(n) => match n {
                        0 => print!(" "),
                        1 => print!("#"),
                        _ => panic!("shouldnt"),
                    },
                    None => print!(" "),
                }
            }
            print!("\n");
        }
    }
}
