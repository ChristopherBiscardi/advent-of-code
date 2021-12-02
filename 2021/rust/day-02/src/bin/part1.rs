#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};
use nom::{
    branch::alt, bytes::complete::tag,
    character::complete::i32, IResult,
};
use std::fs;

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

#[derive(Default, Debug)]
struct Submarine {
    x: i32,
    y: i32,
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
            let (_, dir) = parse_direction(item).unwrap();

            match dir {
                Direction::Forward(magnitude) => {
                    acc.x += magnitude;
                }
                Direction::Up(magnitude) => {
                    acc.y -= magnitude;
                }
                Direction::Down(magnitude) => {
                    acc.y += magnitude;
                }
            };
            acc
        },
    );
    dbg!(final_position.finalize());
}

fn parse_direction(
    input: &str,
) -> IResult<&str, Direction> {
    let (input, dir) =
        alt((tag("forward"), tag("up"), tag("down")))(
            input,
        )?;
    let (input, _) = tag(" ")(input)?;
    let (input, magnitude) = i32(input)?;

    let result = match dir {
        "forward" => Direction::Forward(magnitude),
        "up" => Direction::Up(magnitude),
        "down" => Direction::Down(magnitude),
        _ => {
            panic!("invalid")
        }
    };

    Ok((input, result))
}
