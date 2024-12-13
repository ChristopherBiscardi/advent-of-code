use bevy::prelude::*;

pub mod loader;
pub mod parser;

#[derive(Component)]
pub struct BaseMap;

#[derive(Component)]
pub struct Guard;

#[derive(Component)]
pub struct Wall;

#[derive(Debug, Component, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
    pub fn to_ivec2(&self) -> IVec2 {
        match self {
            Direction::North => IVec2::Y,
            Direction::South => IVec2::NEG_Y,
            Direction::East => IVec2::X,
            Direction::West => IVec2::NEG_X,
        }
    }
}
