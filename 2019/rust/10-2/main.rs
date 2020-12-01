use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<Vec<String>> = contents
        .split("\n")
        .map(|line| {
            line.split("")
                .map(|s| s.to_string())
                .filter(|x| x != "")
                .collect()
        })
        .collect();

    let mut asteroid_locations: Vec<(usize, usize)> = vec![];
    for (y, line) in lines.iter().enumerate() {
        for (x, point) in line.iter().enumerate() {
            if point == "#" {
                asteroid_locations.push((x, y));
            }
        }
    }

    let mut asteroid_maps: Vec<Vec<(f32, i32, f32, usize, usize)>> = vec![];
    for (x, y) in &asteroid_locations {
        asteroid_maps.push(get_asteroid_angles_and_distances(
            asteroid_locations.clone(),
            (*x, *y),
        ));
    }
    let counts: Vec<usize> = asteroid_maps
        .iter()
        .map(|ogMap| {
            let mut map = ogMap.clone();
            map.sort_by(
                |(angle, sign, distance, _x, _y), (angle1, sign1, distance1, _x1, _y1)| {
                    angle.partial_cmp(angle1).unwrap()
                },
            );
            map.dedup_by(
                |(angle, sign, distance, _x, _y), (angle1, sign1, distance1, _x1, _y1)| {
                    angle == angle1 && sign == sign1
                },
            );
            return map.len();
        })
        .collect();
    let max = counts.iter().max().unwrap();
    let index = counts.iter().position(|x| x == max).unwrap();
    // println!("asteroids {:?}", asteroid_maps);
    // println!(
    //     "asteroids {:?} num_ast {:?} max {:?} asteroid {:?}",
    //     asteroid_locations,
    //     asteroid_locations.len(),
    //     max,
    //     asteroid_locations[index]
    // );
    println!("max {} asteroid {:?}", max, asteroid_locations[index]);

    let base_location = asteroid_locations[index];
    let mut asteroid_obliviation_count = 0;
    let mut asteroid_locations_copy = asteroid_locations.clone();
    let mut asteroid_200 = (0, 0);
    while asteroid_obliviation_count < 200 {
        let all_asteroids =
            get_asteroid_angles_and_distances(asteroid_locations_copy.clone(), base_location);
        let mut map = all_asteroids.clone();
        map.sort_by(
            |(angle, sign, distance, _x, _y), (angle1, sign1, _distance1, _x1, _y1)| {
                angle.partial_cmp(angle1).unwrap()
            },
        );
        map.dedup_by(
            |(angle, sign, distance, _x, _y), (angle1, sign1, distance1, _x1, _y1)| {
                angle == angle1 && sign == sign1
            },
        );
        println!("map {:?}", map);
        // four quadrants (minus vertical and horizontal)
        let mut tl: Vec<(f32, i32, f32, usize, usize)> = vec![];
        let mut tr: Vec<(f32, i32, f32, usize, usize)> = vec![];
        let mut br: Vec<(f32, i32, f32, usize, usize)> = vec![];
        let mut bl: Vec<(f32, i32, f32, usize, usize)> = vec![];
        let mut ordered_destroys: Vec<Vec<(f32, i32, f32, usize, usize)>> = vec![];

        for angle in map.clone() {
            // sign of slope, sign of x
            match (angle.0.signum(), angle.1) {
                (-1.0, -1) => tl.push(angle),
                (-1.0, 1) => br.push(angle),
                (1.0, -1) => bl.push(angle),
                (1.0, 1) => tr.push(angle),
                (_, _) => (),
            }
            let top = map.iter().cloned().find(|y| y.0 == 2000.0);
            let right = map.iter().cloned().find(|x| x.0 == 1000.0);
            let down = map.iter().cloned().find(|y| y.0 == -2000.0);
            let left = map.iter().cloned().find(|x| x.0 == -1000.0);
            match top {
                Some(v) => {
                    ordered_destroys.push(vec![v]);
                    ();
                }
                None => (),
            }
            ordered_destroys.push(tr.clone());
            match right {
                Some(v) => {
                    ordered_destroys.push(vec![v]);
                    ();
                }
                None => (),
            }
            ordered_destroys.push(br.clone());
            match down {
                Some(v) => {
                    ordered_destroys.push(vec![v]);
                    ();
                }
                None => (),
            }
            ordered_destroys.push(bl.clone());
            match left {
                Some(v) => {
                    ordered_destroys.push(vec![v]);
                    ();
                }
                None => (),
            }
            ordered_destroys.push(tl.clone());
        }
        let asteroid_pew_pew = ordered_destroys.concat();

        let new_copy: Vec<(usize, usize)> = asteroid_locations_copy
            .clone()
            .iter()
            .filter(|(x, y)| {
                for point in asteroid_pew_pew.clone() {
                    if *x == point.3 && *y == point.4 {
                        return false;
                    }
                }
                return true;
            })
            .map(|a| *a)
            .collect();
        // let (asteroid_pew_pew, right) = new_copy.split_at(map.len());
        // filter new_copy;
        // ORDER asteroid_pew_pew by circle
        asteroid_locations_copy = new_copy;
        if asteroid_pew_pew.len() + asteroid_obliviation_count > 200 {
            let (_slope, _x_sign, _distance, xa, ya) =
                asteroid_pew_pew[200 - asteroid_obliviation_count];
            asteroid_200 = (xa, ya);
            break;
        }
        asteroid_obliviation_count = asteroid_pew_pew.len();
    }
    println!("asteroid 200 {:?}", asteroid_200);
}

fn get_asteroid_angles_and_distances(
    asteroid_locations: Vec<(usize, usize)>,
    (x, y): (usize, usize),
) -> Vec<(f32, i32, f32, usize, usize)> {
    let mut angles: Vec<(f32, i32, f32, usize, usize)> = vec![];
    for (x2, y2) in asteroid_locations {
        if x2 == x && y2 == y {
            // do nothing, it's the same asteroid
            continue;
        }
        if x2 == x {
            // straight line in column
            let distance = y as f32 - y2 as f32;
            angles.push((
                distance.signum() * 1000.0,
                (x2 as i32 - x as i32).signum(),
                distance,
                x2,
                y2,
            ));
            continue;
        }
        if y2 == y {
            // straight line in row
            let distance = x as f32 - x2 as f32;
            angles.push((
                distance.signum() * 2000.0,
                (x2 as i32 - x as i32).signum(),
                distance,
                x2,
                y2,
            ));
            continue;
        }
        let point_a = (x, y);
        let point_c = (x2, y2);
        let distance = ((x2 as f32 - x as f32).powf(2.0) + (y2 as f32 - y as f32).powf(2.0)).sqrt();
        let slope = (y2 as f32 - y as f32) / (x2 as f32 - x as f32);
        let mut angle = slope;
        angles.push((angle, (x2 as i32 - x as i32).signum(), distance, x2, y2));
    }
    return angles;
}
