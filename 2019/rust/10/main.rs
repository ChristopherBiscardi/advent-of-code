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
    // let mut all_points: Vec<(usize, usize)> = vec![];
    for (y, line) in lines.iter().enumerate() {
        for (x, point) in line.iter().enumerate() {
            if point == "#" {
                asteroid_locations.push((x, y));
            }
            // all_points.push((x, y));
        }
    }

    // println!("asteroid_locations {:?}", asteroid_locations);
    // quadrants
    // 0,1
    // 2,3
    let mut asteroid_maps: Vec<Vec<(f32, i32, f32)>> = vec![];
    for (x, y) in &asteroid_locations {
        // Vec<angle, distance>
        // println!("----");
        let mut angles: Vec<(f32, i32, f32)> = vec![];
        // println!("---");
        for (x2, y2) in &asteroid_locations {
            // println!("(x,y) {:?}", (x2, y2));
            if x2 == x && y2 == y {
                // do nothing, it's the same asteroid
                continue;
            }
            if x2 == x {
                // straight line in column
                let distance = *y as f32 - *y2 as f32;
                angles.push((
                    distance.signum() * 1000.0,
                    (*x2 as i32 - *x as i32).signum(),
                    distance,
                ));
                // println!("horizontal {}", distance.signum());
                continue;
            }
            if y2 == y {
                // straight line in row
                let distance = *x as f32 - *x2 as f32;
                angles.push((
                    distance.signum() * 2000.0,
                    (*x2 as i32 - *x as i32).signum(),
                    distance,
                ));
                // println!("vertical {}", distance.signum());
                continue;
            }
            // let (angle, distance, quadrant) = get_triangle()
            // get triangle
            let point_a = (x, y);
            // let point_b = (x2, y);
            let point_c = (x2, y2);
            let distance =
                ((*x2 as f32 - *x as f32).powf(2.0) + (*y2 as f32 - *y as f32).powf(2.0)).sqrt();
            // let adjacent = *x as f32 - *x2 as f32;
            // let opposite = *y as f32 - *y2 as f32;
            // println!("point to point {:?} {:?}", adjacent, opposite);
            // let hypo = (adjacent * adjacent + opposite * opposite).sqrt();
            // let mut angle = opposite / hypo;
            let slope = (*y2 as f32 - *y as f32) / (*x2 as f32 - *x as f32);
            // println!("slope0 {}", 2.0 / 0.0);
            // println!("slope {}", slope);
            let mut angle = slope;
            angles.push((angle, (*x2 as i32 - *x as i32).signum(), distance));
        }
        // println!("angles {:?}", angles.len());
        asteroid_maps.push(angles);
    }
    let counts: Vec<usize> = asteroid_maps
        .iter()
        .map(|ogMap| {
            let mut map = ogMap.clone();
            // println!("map {:?}", map);
            map.sort_by(|(angle, sign, distance), (angle1, sign1, distance1)| {
                angle.partial_cmp(angle1).unwrap()
            });
            // println!("srt {:?}", map);
            map.dedup_by(|(angle, sign, distance), (angle1, sign1, distance1)| {
                angle == angle1 && sign == sign1
            });
            // println!("ddp {:?}", map);
            return map.len();
        })
        .collect();
    // println!("lines {:?}", lines);
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
    println!("max {} asteroid {:?}", max, asteroid_locations[index])
}
