use std::collections::BTreeMap;
use std::env;
use std::fmt;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut tree: BTreeMap<String, FuelNode> = BTreeMap::new();
    for line in contents.split("\n").map(|s| String::from(s)) {
        // println!("line {:?}", line);
        let args = line.split(" => ").collect::<Vec<&str>>();
        // println!("args {:?}", args);
        let inputs = args[0].split(", ").collect::<Vec<&str>>();
        // println!("input {:?}", input);
        let output = args[1].split(" ").collect::<Vec<&str>>();
        // println!("output {:?}", output);
        let mut input_tree = vec![];
        inputs.iter().for_each(|s| {
            let input = s.split(" ").collect::<Vec<&str>>();
            input_tree.push((input[1].to_string(), input[0].parse::<i64>().unwrap()));
        });
        tree.insert(
            output[1].to_string(),
            FuelNode::new(output[0].parse::<i64>().unwrap(), input_tree),
        );
    }

    let mut counts_map: BTreeMap<String, i64> = BTreeMap::new();
    counts_map.insert(String::from("FUEL"), 1);
    let mut leftovers: BTreeMap<String, i64> = BTreeMap::new();
    loop {
        let ore_count = counts_map.get("ORE");
        match ore_count {
            Some(ore) => {
                if counts_map.len() == 1 && ore > &0 {
                    break;
                }
            }
            None => {}
        }
        println!("   >>> counts_map {:?}", counts_map);
        let all_fuel_node_outputs = counts_map
            .clone()
            .keys()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut all_fuel_node_needs: Vec<String> = vec![];

        for node_name in &all_fuel_node_outputs {
            // println!("pre-unwrap {:?}", node_name);
            if node_name == "ORE" {
                continue;
            }
            let fuel_node = tree.get(node_name).unwrap();
            for input in fuel_node.input.iter().map(|v| v.0.to_string()) {
                all_fuel_node_needs.push(input);
            }
        }
        for (name, needed) in counts_map.clone().iter() {
            println!("--->>> counts_map {:?}", counts_map);
            println!("--->>> leftovers {:?}", leftovers);
            if name == "ORE" {
                continue;
            }
            println!("fuel node {}", name);

            // if current output is involved in an equation that produces it as a needed value, then skip while we create the needed before processing the output equation
            // if all_fuel_node_needs.contains(&name) {
            //     continue;
            // }
            let node_has_more_equations_before_output: Vec<(&String, &FuelNode)> = tree
                .iter()
                .filter(|node| {
                    let involved_in = node
                        .1
                        .input
                        .iter()
                        .map(|(k, v)| k.to_string())
                        .collect::<Vec<String>>()
                        .contains(name);
                    involved_in && node.1.processed == false
                })
                .collect();
            if node_has_more_equations_before_output.len() > 0 {
                println!("node_has_more {}", name);
                continue;
            }
            let mut fuel_node = tree.get_mut(name).unwrap();

            let mut leftover_count_for_name: i64 = 0;
            for (input_name, input_value) in fuel_node.input.iter() {
                println!("input {}", input_name);
                match leftovers.get(input_name) {
                    Some(number) => {
                        println!("leftovers check {}", number);
                        if number >= input_value {
                            println!("using leftovers for {}", input_name);
                            *leftovers.entry(input_name.to_string()).or_insert(0) -= input_value;
                            counts_map.remove(name);
                            fuel_node.processed = true;
                            continue;
                        }
                    }
                    None => {}
                }
                let (input_count, leftover_count) =
                    leftover(*needed, fuel_node.output, *input_value);
                // TODO: should we use leftovers to top off amount_to_add
                leftover_count_for_name = leftover_count;
                fuel_node.processed = true;
                println!(
                    "post-leftover: {} {}, {} {}",
                    input_count, input_name, leftover_count, name
                );

                *counts_map.entry(input_name.to_string()).or_insert(0) += input_count;
            }
            *leftovers.entry(name.to_string()).or_insert(0) += leftover_count_for_name;

            println!("count");
            let count = counts_map.get(name);
            match count {
                Some(c) => {
                    println!("count: {}", c);
                    // if c == needed {
                    println!("removing {}", name);
                    counts_map.remove(name);
                    // }
                }
                None => {
                    println!("can't count {}", name);
                }
            }
        }
    }
    println!("tree {:?}", leftovers);
    println!("ORE Count {:?}", counts_map.get("ORE"));
    for (k, v) in leftovers {
        if v > 0 {
            println!("{} {} {:?}", k, v, v == tree.get(&k).unwrap().output);
        }
    }
}

fn leftover(needed: i64, output: i64, input: i64) -> (i64, i64) {
    // println!("leftover: {} {} {}", needed, output, input);
    let m = needed % output;
    // println!("m {}", m);
    let amount_to_add = match m {
        0 => 0,
        n => output - n,
    };
    // println!("amount_to_add {}", amount_to_add);
    let input_amount = input * ((needed + amount_to_add) / output);
    // println!("input_amount {}", input_amount);
    return (input_amount, amount_to_add);
}

struct FuelNode {
    processed: bool,
    output: i64,
    input: Vec<(String, i64)>,
}

impl FuelNode {
    fn new(output: i64, input: Vec<(String, i64)>) -> FuelNode {
        FuelNode {
            processed: false,
            output: output,
            input: input,
        }
    }
}
impl fmt::Debug for FuelNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("FuelNode")
            .field("processed", &self.processed)
            .field("output", &self.output)
            .field("input", &self.input)
            .finish()
    }
}

#[test]
fn test_leftovers() {
    // two ones
    assert_eq!(leftover(1, 1, 1), (1, 0));
    assert_eq!(leftover(1, 1, 2), (2, 0));
    assert_eq!(leftover(1, 1, 4), (4, 0));
    assert_eq!(leftover(4, 1, 1), (4, 0));
    assert_eq!(leftover(1, 1, 7), (7, 0));

    // one one
    assert_eq!(leftover(2, 1, 3), (6, 0));
    assert_eq!(leftover(3, 1, 5), (15, 0));
    assert_eq!(leftover(4, 1, 4), (16, 0));

    // other
    assert_eq!(leftover(7, 10, 10), (10, 3));
    assert_eq!(leftover(10, 2, 9), (45, 0));
    assert_eq!(leftover(23, 3, 8), (64, 1));
    assert_eq!(leftover(37, 5, 7), (56, 3));

    assert_eq!(leftover(9, 2, 165), (825, 1));
    assert_eq!(leftover(48, 5, 177), (1770, 2));
    // assert_eq!(leftover(9, 2, 165), (330, 1));
    // assert_eq!(leftover(9, 2, 165), (330, 1));
}
