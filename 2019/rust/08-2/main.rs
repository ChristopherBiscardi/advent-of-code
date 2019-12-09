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

    let mut final_layer: Vec<u32> = vec![];

    for i in 0..layer_length {
        let pixel = 2;
        for n in 0..layer_count {
            if layers[n][i] != 2 {
                final_layer.push(layers[n][i]);
                break;
            }
        }
    }

    for (i, n) in final_layer.iter().enumerate() {
        if *n == 0 {
            print!("{}", String::from(" "));
        } else {
            print!("{}", n.to_string());
        }
        // println!("rem {}", i + 1 % width);
        if (i + 1) % width == 0 {
            print!("\n");
        }
    }
    //    println!("output {:?}", final_layer);
}
