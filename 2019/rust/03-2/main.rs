use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines: Vec<Vec<String>> = contents.split("\n")
                  .map(|line| line.split(",")
                              .map(|s| s.to_string()).collect())
                  .collect();
    // println
    let arrow_points: Vec<Vec<(i64,i64)>> = lines.iter().map(|line| {
        // println!("in lines");
        // let arrow_points:  = vec![];
        let mut positions: Vec<(i64,i64)> = vec![(0,0)];

        for instruction in line {
            // println!("in instruction");
            let direction = instruction.get(..1);
            let distance = instruction.get(1..);
            match (direction, distance) {
                (Some(dir),Some(dis)) => {
                    let distance_int = dis.parse::<i64>().unwrap();
                    let mut i = 1;
                        match dir {
                        "U" => {
                            loop {
                                if i > distance_int { break; }
                                let (x,y) = positions[positions.len()-1];
                                positions.push((x,y+1));
                                i = i+1
                            }
                        }
                        "D" => {
                            loop {
                                if i > distance_int { break; }
                                let (x,y) = positions[positions.len()-1];
                                positions.push((x,y-1));
                                i = i+1
                            }
                        }
                        "L" => {
                            loop {
                                if i > distance_int { break; }
                                let (x,y) = positions[positions.len()-1];
                                positions.push((x-1,y));
                                i = i+1
                            }
                        }
                        "R" => {
                            loop {
                                if i > distance_int { break; }
                                let (x,y) = positions[positions.len()-1];
                                positions.push((x+1,y));
                                i = i+1
                            }
                        }
                        _ => println!("Invalid direction, {}", dir)
                    }
                }
                _    => println!("Cannot divide by 0"),
            }
            // println!("{}-{}", direction, distance);
        }
        positions

    }).collect();

    let arrow_one = &arrow_points[0];
    let arrow_two = &arrow_points[1];
    let mut common_points: Vec<(i64, i64)> = vec![];
    let mut min_point: (i64, i64) = (0,0);
    let mut min_distance: i64 = 10000000;

    let mut n = 1;
    for point in arrow_one {
        println!("on point {} of {} - {},{}", n, arrow_one.len(), point.0, point.1);
        n = n+1;
        // println!("one: {} {}", point.0, point.1);
        if !(point.0 == 0 && point.1 == 0) {
            // for p in arrow_two {
            //     println!("two {} {}", p.0, p.1);
            // }

            if arrow_two.contains(point) {
                common_points.push((point.0,point.1));
                let combined = point.0.abs() + point.1.abs();
                // println!("combined {}", combined);
                let new_min = if combined < min_distance { combined } else { min_distance };
                if new_min != min_distance {
                    min_point = (point.0,point.1);
                    min_distance = new_min;
                }
            }
        }
    }
    println!("min_distance {}", min_distance);

    // find arrow length
    let mut common_point_distance: usize = 1000000;
    for point in common_points {
        let a1_index = arrow_one.iter().position(|x| *x == point);
        let a2_index = arrow_two.iter().position(|x| *x == point);
        match (a1_index, a2_index) {
            (Some(a), Some(b)) => {
                if (a + b) < common_point_distance {
                    common_point_distance = a + b;
                }
            }
            _ => println!("no match")
        }

    }
    println!("common_point_distance {}", common_point_distance)

}
