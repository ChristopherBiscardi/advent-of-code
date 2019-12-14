use std::collections::BTreeMap;

fn main() {
    let mut xs = vec![-8, -11, 8, -2];
    let mut ys = vec![-18, -14, -3, -16];
    let mut zs = vec![6, 4, -10, 1];
    let x = loop_check(&mut xs);
    let y = loop_check(&mut ys);
    let z = loop_check(&mut zs);

    println!("x {} y {} z {}", x, y, z);
    let mut ns = vec![x, y, z];
    ns.sort();
    let a = ns[0];
    let b = ns[1];
    let c = ns[2];
    let mut current = c;
    let mut i = 1;
    loop {
        // println!("current {}", current);
        if current % a == 0 && current % b == 0 {
            break;
        } else {
            i = i + 1;
            current = c * i;
        }
    }

    println!("x {} {} {} {}", x, y, z, current);
}

fn loop_check(xs: &mut Vec<i64>) -> i64 {
    // println!("xs {:?}", xs);
    println!("---");
    let xs_check = xs.clone();
    let mut velocity = vec![0, 0, 0, 0];
    let mut i = 1;
    loop {
        // get all pairs of moons
        let mut moons_combos: Vec<(i64, i64)> = vec![];
        let mut moon_keys: Vec<i64> = vec![0, 1, 2, 3];
        while moon_keys.len() > 1 {
            let moon = moon_keys.pop().unwrap();
            for other_moon in &moon_keys {
                moons_combos.push((moon, *other_moon));
            }
        }
        // apply gravity
        for (moon_index_a, moon_index_b) in moons_combos {
            let moon_a = xs[moon_index_a as usize];
            let moon_b = xs[moon_index_b as usize];
            let (velocity_change_a, velocity_change_b) = apply_gravity(moon_a, moon_b);

            velocity[moon_index_a as usize] = velocity[moon_index_a as usize] + velocity_change_a;
            velocity[moon_index_b as usize] = velocity[moon_index_b as usize] + velocity_change_b;
        }
        // apply velocity
        for i in vec![0, 1, 2, 3] {
            xs[i] = xs[i as usize] + velocity[i as usize];
            // println!("moon {} {:?} {:?}", i, moon.position, moon.velocity);
        }
        // if i % 10 == 0 {
        //     println!("xs {:?}", xs);
        // }
        let cycled = xs_check
            .iter()
            .zip(xs.iter())
            .fold(true, |acc, (a, b)| acc && a == b);
        if cycled {
            println!("xs {:?}", xs);

            break;
        }
        i = i + 1;
    }
    return i + 1;
}

fn apply_gravity(a: i64, b: i64) -> (i64, i64) {
    if a == b {
        return (0, 0);
    }
    if a > b {
        return (-1, 1);
    }
    return (1, -1);
}
