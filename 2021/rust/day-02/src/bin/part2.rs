#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};
use std::fs;

use day_02::{parse_direction, Direction};

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

#[derive(Default, Debug)]
struct Submarine {
    x: i32,
    y: i32,
    aim: i32,
}

impl Submarine {
    fn finalize(&self) -> i32 {
        self.x * self.y
    }
    fn swim(&mut self, dir: Direction) {
        match dir {
            Direction::Forward(magnitude) => {
                self.x += magnitude;
                self.y += self.aim * magnitude;
            }
            Direction::Up(magnitude) => {
                self.aim -= magnitude;
            }
            Direction::Down(magnitude) => {
                self.aim += magnitude;
            }
        };
    }
}

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let file = fs::read_to_string("./input.txt").unwrap();
    let final_sub = file.lines().fold(
        Submarine::default(),
        |mut sub, line| {
            let (_, dir) = parse_direction(line).unwrap();
            sub.swim(dir);
            sub
        },
    );
    dbg!(final_sub.finalize());
}
