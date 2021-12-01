use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let seafloor: Vec<u16> = file
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u16>>();

    let mut it = seafloor.iter().peekable();

    let mut i = 0;
    for _ in 0..it.len() {
        let one = it.next();
        let two = it.peek();
        match (one, two) {
            (Some(i_one), Some(&i_two)) => {
                if i_two > i_one {
                    i += 1;
                }
            }
            _ => {}
        }
    }
    dbg!(i);
}
