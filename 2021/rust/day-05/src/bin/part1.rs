use day_05::{puzzle_input, Point};
#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};
use std::collections::BTreeMap;
use std::fs;

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();

    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process(&file));
}

fn process(input: &str) -> usize {
    let (_, lines) = puzzle_input(input).unwrap();
    let mut points: BTreeMap<Point, u8> = BTreeMap::new();
    for (a, b) in lines {
        let dx = b.x - a.x;
        let dy = b.y - a.y;

        if dx == 0 {
            let pt = {
                match dy.signum() {
                    1 => a,
                    -1 => b,
                    _ => {
                        panic!("nope");
                    }
                }
            };
            for new_y in pt.y..=(pt.y + dy.abs()) {
                // println!("inserting {} {}", &pt.x, new_y);
                // println!("inserting ({},{})", &pt.x, new_y);

                points
                    .entry(Point { x: pt.x, y: new_y })
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        } else if dy == 0 {
            let pt = {
                match dx.signum() {
                    1 => a,
                    -1 => b,
                    _ => {
                        panic!("nope");
                    }
                }
            };
            for new_x in pt.x..=(pt.x + dx.abs()) {
                // println!("inserting ({},{})", new_x, &pt.y);

                points
                    .entry(Point { x: new_x, y: pt.y })
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        } else {
            continue;
        }
    }
    let count = points
        .iter()
        .filter(|(_point, &count)| count >= 2)
        .count();
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &'static str =
        include_str!("./test-input.txt");

    #[test]
    fn test_demo_data() {
        assert_eq!(5, process(input));
    }
}
