use bevy::{
    color::palettes::tailwind::SKY_800, prelude::*,
};
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(SKY_800.into()))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}

fn parse(input: &str) -> IResult<&str, Vec<IVec2>> {
    separated_list1(
        line_ending,
        separated_pair(
            complete::i32,
            tag(","),
            complete::i32,
        )
        .map(|(x, y)| IVec2::new(x, y)),
    )
    .parse(input)
}

#[derive(Resource)]
struct Data(Vec<IVec2>);

fn startup(mut commands: Commands) {
    let input = include_str!("./input2.txt");
    let (_, vecs) = parse(input).unwrap();
    commands.insert_resource(Data(vecs));
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode:
                bevy::camera::ScalingMode::AutoMin {
                    min_width: 100000.,
                    min_height: 100000.,
                },
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(50000., 50000., 0.),
    ));
}
fn update(data: Res<Data>, mut gizmos: Gizmos) {
    for (a, b) in data.0.iter().tuple_windows() {
        gizmos.line_2d(
            a.as_vec2(),
            b.as_vec2(),
            Color::WHITE,
        );
    }
}
