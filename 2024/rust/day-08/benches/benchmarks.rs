use day_08::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn parser_part1() {
    part1::parse(divan::black_box(part1::Span::new(
        include_str!("../input1.txt",),
    )))
    .unwrap();
}

#[divan::bench]
fn parser_part1_nolocate() {
    fn parse(input: &str) -> nom::IResult<&str, Vec<char>> {
        nom::multi::many1(nom::sequence::preceded(
            nom::bytes::complete::take_till(|c: char| {
                nom::AsChar::is_alphanum(c)
            }),
            nom::character::complete::satisfy(|c| {
                nom::AsChar::is_alphanum(c)
            }),
        ))(input)
    }
    parse(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn parser_part1_nolocate_tuple() {
    fn alphanum_pos(
        input: part1::Span,
    ) -> nom::IResult<part1::Span, (glam::IVec2, char)>
    {
        let (input, pos) = nom_locate::position(input)?;
        let x = pos.get_column() as i32 - 1;
        let y = pos.location_line() as i32 - 1;
        let (input, c) =
            nom::character::complete::satisfy(|c| {
                nom::AsChar::is_alphanum(c)
            })(input)?;
        Ok((input, (glam::IVec2::new(x, y), c)))
    }
    fn parse(
        input: part1::Span,
    ) -> nom::IResult<part1::Span, Vec<(glam::IVec2, char)>>
    {
        nom::multi::many1(nom::sequence::preceded(
            nom::bytes::complete::take_till(|c: char| {
                nom::AsChar::is_alphanum(c)
            }),
            alphanum_pos,
        ))(input)
    }
    parse(divan::black_box(part1::Span::new(
        include_str!("../input1.txt",),
    )))
    .unwrap();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part2_iterators() {
    part2_iterators::process(divan::black_box(
        include_str!("../input2.txt",),
    ))
    .unwrap();
}
