use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let width = 25;
    let height = 6;
    let layer_length = width * height;

    if args.len() == 1 {
        panic!("Need to add a file argument");
    }

    let filename = &args[1];
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let input: Vec<u32> = contents.chars().map(|d| d.to_digit(10).unwrap()).collect();
    let layer_count = input.len() / (width * height);
    let mut layers: Vec<Vec<u32>> = vec![];
    for i in 0..layer_count {
        let mut layer: Vec<u32> = vec![];
        for n in 0..layer_length {
            layer.push(input[layer_length * i + n]);
        }

        layers.push(layer);
    }

    let mut zero_counts = layers.iter().map(|layer| {
        let mut l = layer.clone();
        l.retain(|n| *n == 0);
        return l.len();
    });
    let fewest_zeros = zero_counts.clone().min().unwrap();
    let layer_index = zero_counts.clone().position(|v| v == fewest_zeros).unwrap();

    let layer_to_process = &layers[layer_index as usize];
    let mut ones = layer_to_process.clone();
    ones.retain(|n| *n == 1);
    let mut twos = layer_to_process.clone();
    twos.retain(|n| *n == 2);

    println!("output {}", ones.len() * twos.len());
}
