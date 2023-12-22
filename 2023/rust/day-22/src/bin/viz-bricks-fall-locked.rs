//! A shader and a material that uses it.
use std::f32::consts::PI;

use bevy::{
    core_pipeline::{
        bloom::BloomSettings, tonemapping::Tonemapping,
    },
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Anchor,
};
use bevy_basic_camera::{
    CameraController, CameraControllerPlugin,
};
use bevy_xpbd_3d::prelude::*;
use day_22::parse_full_brick::parse_bricks;
use rand::Rng;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("1e1e2e").unwrap(),
        ))
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<CustomMaterial>::default(),
            PhysicsPlugins::default(),
            CameraControllerPlugin,
            // PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, highlight_colliding_cubes)
        .run();
}

const INPUT: &str = include_str!("../../input1.txt");

const TEST_INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

const GRID_TEST: &str = "0,0,1~4,4,1
0,0,2~0,0,4
1,1,2~1,1,4
2,2,2~2,2,3
1,2,2~1,2,2
4,4,5~4,4,5
4,4,6~4,4,6
4,4,7~4,4,7
4,4,8~4,4,8
4,4,9~4,4,10
0,0,5~0,0,5
0,0,6~0,0,6
0,0,7~0,0,7
0,0,8~0,0,8
0,0,9~0,0,10";
/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut materials_std: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let (_, bricks) =
        parse_bricks(INPUT).expect("should parse");
    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            // illuminance: 1.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        // cascade_shadow_config: CascadeShadowConfigBuilder {
        //     first_cascade_far_bound: 4.0,
        //     maximum_distance: 10.0,
        //     ..default()
        // }
        // .into(),
        ..default()
    });
    commands.spawn((
        PbrBundle {
            mesh: meshes
                .add(shape::Plane::from_size(100.0).into()),

            material: materials_std.add(StandardMaterial {
                base_color: Color::hex("313244").unwrap(),
                perceptual_roughness: 1.0,
                ..default()
            }),
            transform: Transform::from_translation(
                Vec3::new(0., 0., 0.5),
            ),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(100.0, 0.002, 100.0),
    ));

    let mut rng = rand::thread_rng();

    for brick in bricks.iter() {
        let size = IVec3::new(
            brick.end.x - brick.start.x + 1,
            brick.end.z - brick.start.z + 1,
            brick.end.y - brick.start.y + 1,
        );
        info!(?size);
        let b = shape::Box {
            min_x: 0.,
            max_x: size.x as f32,
            min_y: 0.,
            max_y: size.y as f32,
            min_z: 0.,
            max_z: size.z as f32,
        };
        let hue: i32 = rng.gen_range(0..360);

        let color = Color::Lcha {
            lightness: 0.8,
            chroma: 1.0,
            hue: hue as f32,
            alpha: 1.0,
        };
        let collider_size = (brick.end.as_vec3()
            - brick.start.as_vec3())
            - 0.1;
        let translation = brick.start.as_vec3().xzy();
        let translation = Vec3::new(
            translation.x,
            translation.y,
            translation.z,
        );
        info!(?translation, ?b);
        commands
            .spawn((
                // Anchor::BottomLeft,
                MaterialMeshBundle {
                    mesh: meshes.add(Mesh::from(b)),
                    transform: Transform::from_translation(
                        translation,
                    ),
                    // material: materials.add(CustomMaterial {
                    //     color,
                    //     // color_texture: None,
                    //     alpha_mode: AlphaMode::Blend,
                    // }),
                    material: materials_std.add(
                        StandardMaterial {
                            base_color: color,
                            emissive: color,
                            ..default()
                        },
                    ),
                    ..default()
                },
                RigidBody::Dynamic,
                // Collider::
                LockedAxes::ROTATION_LOCKED
                    .lock_translation_x()
                    .lock_translation_z(),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Restitution::new(0.0),
                    Collider::cuboid(
                        size.x as f32 - 0.01,
                        size.y as f32 - 0.01,
                        size.z as f32 - 0.01,
                    ),
                    Transform::from_xyz(
                        size.x as f32 / 2.0,
                        size.y as f32 / 2.0,
                        size.z as f32 / 2.0,
                    ),
                ));
            });
    }

    // let max_y = bricks
    //     .iter()
    //     .flat_map(|brick| brick.cubes.iter())
    //     .max_by_key(|c| c.z)
    //     .unwrap();
    // let halfway = max_y.z / 2;
    // camera
    // dbg!(halfway);
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(20.0, 5.0, 20.0)
                .looking_at(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::Y,
                ),
            ..default()
        },
        BloomSettings::default(),
        CameraController::default(),
    ));
}

fn highlight_colliding_cubes(
    query: Query<(
        Entity,
        &CollidingEntities,
        &LinearVelocity,
    )>,
    mut standard_materials: Query<
        &mut Handle<StandardMaterial>,
    >,
    mut materials_std: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, _colliding_entities, v) in &query {
        // println!(
        //     "{:?} is colliding with the following entities: {:?}",
        //     entity,
        //     colliding_entities
        // );
        if let Ok(mat) = standard_materials.get(entity) {
            let m =
                materials_std.get_mut(mat.id()).unwrap();
            m.emissive.as_lcha().set_l(v.y.abs());
            m.base_color.as_lcha().set_l(v.y.abs());
        }
    }
}

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    // #[texture(1)]
    // #[sampler(2)]
    // color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
