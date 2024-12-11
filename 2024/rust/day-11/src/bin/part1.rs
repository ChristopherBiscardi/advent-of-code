use day_11::part1::process;
use miette::{miette, Context};
use std::env;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();
    let num = env::args()
        .nth(1)
        .ok_or(miette!("blinks argument not parseable"))?;

    let file = include_str!("../../input1.txt");
    let result = process(
        file,
        num.parse::<u64>().expect("number to be parseable"),
    )
    .context("process part 1")?;
    println!("{}", result);
    Ok(())
}
