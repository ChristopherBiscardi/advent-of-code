use day_13::part1::detect_fold;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input1.txt");
    for mirrors in file.split("\n\n") {
        let result = detect_fold(mirrors);
        println!("{:?}", result);
    }
    Ok(())
}
