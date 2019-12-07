use std::env;
use std::fs;
use std::collections::BTreeMap;
// use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // let tree: BTreeMap<String, String> = BTreeMap::new();

    let lines: Vec<Vec<String>> = contents.split("\n")
                  .map(|line| line.split(")")
                              .map(|s| s.to_string()).collect())
                  .collect();

    let tree:BTreeMap<String, String> = lines.iter().fold(BTreeMap::new(), |mut tree2: BTreeMap<String, String>, orbit_array: &Vec<String>| {
        tree2.insert(orbit_array[1].clone(), orbit_array[0].clone());
        tree2
    });

    let result = tree.values().fold(0, |acc, obj| {
        // println!("obj {}", obj);
        let mut total = 1;
        let mut cur_obj = obj;
        loop {
            match tree.get(cur_obj) {
                Some(o) => {
                    // println!("match");
                    total = total + 1;
                    cur_obj = o;
                },
                None => {
                    // println!("break");
                    break
                }
            }
        };
        acc + total
    });

    println!("result {}", result)


    // for line in lines {
    //     tree.insert(line[1], line[0])
    // }

    // let tree = lines.iter().fold(BTreeMap::new(), |tree, orbitArray| tree.insert(orbitArray[0], orbitArray[1]));
                  
    // println
    // let arrow_points: Vec<Vec<(i64,i64)>> = lines.iter().map(|line| {})
}

