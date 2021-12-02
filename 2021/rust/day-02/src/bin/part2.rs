#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};
use std::fs;

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
}
enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let file = fs::read_to_string("./input.txt").unwrap();
    let final_position = file.lines().fold(
        Submarine::default(),
        |mut acc, item| {
            let items =
                item.split(' ').collect::<Vec<&str>>();
            let magnitude =
                items[1].parse::<i32>().unwrap();
            match items[0] {
                "forward" => {
                    acc.x += magnitude;
                    acc.y += acc.aim * magnitude;
                }
                "up" => {
                    acc.aim -= magnitude;
                }
                "down" => {
                    acc.aim += magnitude;
                }
                i => panic!("unhandled input {}", i),
            };
            acc
        },
    );
    dbg!(final_position.finalize());
}
