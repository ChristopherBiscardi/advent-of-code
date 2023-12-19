#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Round {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Game<'a> {
    pub id: &'a str,
    pub rounds: Vec<Round>,
}
