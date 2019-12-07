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

    let mut tree:BTreeMap<String, String> = lines.iter().fold(BTreeMap::new(), |mut tree2: BTreeMap<String, String>, orbit_array: &Vec<String>| {
        tree2.insert(orbit_array[1].clone(), orbit_array[0].clone());
        tree2
    });

    let paths = get_paths(&tree);

    let mut common_points = vec![];
    for point in &paths[0] {
        if paths[1].contains(&point) {
            common_points.push(point);
        }
    }


    let result: Vec<(i64, &String)> = common_points.iter().map(|obj| {
        let mut cur_obj = *obj;
        let mut total = 1;

        loop {
           let val = tree.get(&cur_obj.to_string());
           match val {
                Some(o) => {
                    // println!("match");
                    cur_obj = &o;
                    total = total + 1;
                    ()
                },
                None => {
                    // println!("break");
                    break
                }
            }
        };
        (total, *obj)
    }
    ).collect();

    let mut max = 0;
    let mut max_point = "";
    for (a,b) in result {
        if a > max {
            max = a;
            max_point = b
        }
    }

    tree.remove(&max_point.to_string());
    let new_paths = get_paths(&tree);
    println!("distance: {}", new_paths[0].len() + new_paths[1].len() - 2 /*YOU and SAN are included in the nodes*/);
    
    // println!("result {}", result)
}

fn get_paths(tree: &BTreeMap<String,String>) -> Vec<Vec<String>> {
    return vec!["YOU".to_string(), "SAN".to_string()].iter().map(|obj| {
        let mut cur_obj = obj;
        let mut acc = vec![];
        loop {
            match tree.get(&cur_obj.to_string()) {
                Some(o) => {
                    cur_obj = o;
                    acc.push(o.to_string())
                },
                None => {
                    break
                }
            }
        };
        acc
    }).collect();
}