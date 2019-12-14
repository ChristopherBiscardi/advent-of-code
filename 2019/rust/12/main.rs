use std::collections::BTreeMap;
// Io, Europa, Ganymede, and Callisto
fn main() {
    let mut moons: BTreeMap<String, Moon> = BTreeMap::new();
    moons.insert(String::from("io"), Moon::new((-8, -18, 6)));
    moons.insert(String::from("europa"), Moon::new((-11, -14, 4)));
    moons.insert(String::from("ganymede"), Moon::new((8, -3, -10)));
    moons.insert(String::from("callisto"), Moon::new((-2, -16, 1)));

    for i in 0..1000 {
        // get all pairs of moons
        let mut moons_combos: Vec<(String, String)> = vec![];
        let mut moon_keys: Vec<String> = moons.keys().cloned().collect();
        while moon_keys.len() > 1 {
            let moon = moon_keys.pop().unwrap();
            for other_moon in &moon_keys {
                moons_combos.push((moon.clone(), other_moon.to_string()));
            }
        }
        // println!("{:?}", moons_combos);
        // apply gravity
        for (moon_name_a, moon_name_b) in moons_combos {
            let moon_a: &Moon = moons.get(&moon_name_a).unwrap();
            let moon_b: &Moon = moons.get(&moon_name_b).unwrap();
            let (xa, xb) = apply_gravity(moon_a.position.0, moon_b.position.0);
            let (ya, yb) = apply_gravity(moon_a.position.1, moon_b.position.1);
            let (za, zb) = apply_gravity(moon_a.position.2, moon_b.position.2);

            let names = vec![moon_name_a, moon_name_b];
            for (i, name) in names.iter().enumerate() {
                let moon: &mut Moon = moons.get_mut(name).unwrap();
                if i == 0 {
                    moon.velocity = (
                        moon.velocity.0 + xa,
                        moon.velocity.1 + ya,
                        moon.velocity.2 + za,
                    )
                } else {
                    moon.velocity = (
                        moon.velocity.0 + xb,
                        moon.velocity.1 + yb,
                        moon.velocity.2 + zb,
                    );
                }
            }
        }
        // apply velocity
        for (i, moon) in moons.values_mut().enumerate() {
            moon.apply_velocity();
            // println!("moon {} {:?} {:?}", i, moon.position, moon.velocity);
        }
        // println!("---");
    }
    let energy = moons.values().fold(0, |eng, moon| {
        let (p1, p2, p3) = moon.position;
        let (v1, v2, v3) = moon.velocity;
        let total_eng = (p1.abs() + p2.abs() + p3.abs()) * (v1.abs() + v2.abs() + v3.abs());
        eng + total_eng
    });
    // println!("{} {}", pot_eng, vel_eng);
    println!("total energy {}", energy);
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

struct Moon {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
}
impl Moon {
    fn new(pos: (i64, i64, i64)) -> Moon {
        Moon {
            position: pos,
            velocity: (0, 0, 0),
        }
    }
    fn apply_velocity(&mut self) {
        let (va, vb, vc) = self.velocity;
        let (pa, pb, pc) = self.position;
        self.position = (va + pa, vb + pb, vc + pc);
    }
}
