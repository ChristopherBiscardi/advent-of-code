//! A shader and a material that uses it.

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use day_18::{
    camera::{CameraController, CameraControllerPlugin},
    points,
};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(
            MaterialPlugin::<CustomMaterial>::default(),
        )
        .add_plugin(CameraControllerPlugin)
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let input = include_str!("../../input.txt");
    let (_, points) = points(input).unwrap();
    for IVec3 { x, y, z } in points {
        // cube
        commands.spawn(MaterialMeshBundle {
            mesh: meshes
                .add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: Transform::from_xyz(
                x as f32, y as f32, z as f32,
            ),
            material: materials.add(CustomMaterial {
                color: Color::BLUE,
                alpha_mode: AlphaMode::Blend,
            }),
            ..default()
        });
    }

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(
                10.0, 10.0, -20.0,
            )
            .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController {
            orbit_mode: true,
            orbit_focus: Vec3::new(10.0, 10.0, 0.0),
            ..default()
        },
    ));
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/voxel.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    alpha_mode: AlphaMode,
}
