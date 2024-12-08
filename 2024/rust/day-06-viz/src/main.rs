use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_ecs_tilemap::{
    map::{
        TilemapId, TilemapSize, TilemapTexture,
        TilemapTileSize, TilemapType,
    },
    prelude::get_tilemap_center_transform,
    tiles::{
        TileBundle, TileColor, TilePos, TileStorage,
        TileTextureIndex,
    },
    TilemapBundle, TilemapPlugin,
};
use day_06_viz::{
    loader::{AocDay6, AocDay6Loader},
    BaseMap, Direction, Guard,
};
use glam::IVec2;
use itertools::Itertools;
use miette::miette;
use std::{collections::HashSet, time::Duration};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_asset::<AocDay6>()
        .init_asset_loader::<AocDay6Loader>()
        .init_resource::<State>()
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (print_on_load,))
        .add_systems(Update, move_guard)
        .run();
}

// pub fn process(input: &str) -> miette::Result<String> {
//     let (_input, ((mut player_position, _), walls)) =
//         parse(Span::new(input))
//             .map_err(|e| miette!("parse failed {}", e))?;

//     let x_minmax = walls
//         .iter()
//         .map(|(position, _)| position.x)
//         .minmax()
//         .into_option()
//         .unwrap();

//     let y_minmax = walls
//         .iter()
//         .map(|(position, _)| position.y)
//         .minmax()
//         .into_option()
//         .unwrap();

//     let mut direction = Direction::North;

//     let mut visited_positions: HashSet<IVec2> =
//         HashSet::from([player_position]);

//     while (x_minmax.0..=x_minmax.1)
//         .contains(&player_position.x)
//         && (y_minmax.0..=y_minmax.1)
//             .contains(&player_position.y)
//     {
//         let next_position =
//             player_position + direction.to_ivec2();
//         if walls.get(&next_position).is_some() {
//             direction = direction.turn_right();
//         } else {
//             player_position = next_position;
//             visited_positions.insert(player_position);
//         }
//     }
//     // dbg!(&visited_positions);

//     Ok((visited_positions.len() - 1).to_string())
// }

#[derive(Resource, Default)]
struct State {
    handle: Handle<AocDay6>,
    other_handle: Handle<AocDay6>,
    printed: bool,
}

fn setup(
    mut state: ResMut<State>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
    // Recommended way to load an asset
    state.handle = asset_server.load("input.day6.txt");

    // File extensions are optional, but are recommended for project management and last-resort inference
    // state.other_handle =
    // asset_server.load("data/asset_no_extension");
}

fn print_on_load(
    mut state: ResMut<State>,
    custom_assets: Res<Assets<AocDay6>>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    let custom_asset = custom_assets.get(&state.handle);

    if state.printed || custom_asset.is_none() {
        return;
    }

    let asset = custom_asset.unwrap();
    info!(
        "start at {}",
        asset.guard_start_position
    );
    info!("wall count {}", asset.walls.len());

    // Once printed, we won't print again
    state.printed = true;

    // Build tilemap
    let x_minmax = asset
        .walls
        .iter()
        .map(|position| position.x)
        .minmax()
        .into_option()
        .unwrap();

    let y_minmax = asset
        .walls
        .iter()
        .map(|position| position.y)
        .minmax()
        .into_option()
        .unwrap();

    let texture_handle: Handle<Image> =
        asset_server.load("tilemap.png");

    let map_size = TilemapSize {
        x: (x_minmax.1 + 1) as u32,
        y: (y_minmax.1 + 1) as u32,
    };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::filling::fill_tilemap.
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let mut t = commands.spawn(TileBundle {
                // color: TileColor(Color::BLACK),
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index: if asset
                    .walls
                    .get(&IVec2::new(x as i32, y as i32))
                    .is_some()
                {
                    TileTextureIndex(24)
                } else {
                    TileTextureIndex(14)
                },
                ..Default::default()
            });

            let tile_entity = t.id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 8.0, y: 8.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands
        .entity(tilemap_entity)
        .insert(TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(
                texture_handle.clone(),
            ),
            tile_size,
            transform: get_tilemap_center_transform(
                &map_size, &grid_size, &map_type, 0.0,
            ),
            ..Default::default()
        })
        .insert(BaseMap);

    // layer 2

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::filling::fill_tilemap.
    // for x in 0..map_size.x {
    //     for y in 0..map_size.y {
    let x = asset.guard_start_position.x;
    let y = asset.guard_start_position.y;

    let tile_pos = TilePos {
        x: x as u32,
        y: y as u32,
    };
    let mut t = commands.spawn(TileBundle {
        // color: TileColor(Color::BLACK),
        position: tile_pos,
        tilemap_id: TilemapId(tilemap_entity),
        texture_index: TileTextureIndex(13),
        ..Default::default()
    });

    t.insert((Direction::North, Guard));

    let tile_entity = t.id();
    tile_storage.set(&tile_pos, tile_entity);
    //     }
    // }

    let tile_size = TilemapTileSize { x: 8.0, y: 8.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    let mut layer = get_tilemap_center_transform(
        &map_size, &grid_size, &map_type, 0.0,
    );
    layer.translation.z = 2.;
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: layer,
        ..Default::default()
    });
}

fn move_guard(
    mut query: Query<
        (&mut TilePos, &mut Direction),
        With<Guard>,
    >,
    mut state: ResMut<State>,
    custom_assets: Res<Assets<AocDay6>>,
    base_map: Single<&TileStorage, With<BaseMap>>,
    mut tiles: Query<&mut TileTextureIndex>,
) {
    if !state.printed {
        return;
    }
    let asset = custom_assets.get(&state.handle).unwrap();

    let x_minmax = asset
        .walls
        .iter()
        .map(|position| position.x)
        .minmax()
        .into_option()
        .unwrap();

    let y_minmax = asset
        .walls
        .iter()
        .map(|position| position.y)
        .minmax()
        .into_option()
        .unwrap();

    for (mut pos, mut dir) in &mut query {
        if (x_minmax.0..=x_minmax.1)
            .contains(&(pos.x as i32))
            && (y_minmax.0..=y_minmax.1)
                .contains(&(pos.y as i32))
        {
            let x = pos.x as i32 + dir.to_ivec2().x;
            let y = pos.y as i32 + dir.to_ivec2().y;
            let next_position = TilePos {
                x: x as u32,
                y: y as u32,
            };

            if asset
                .walls
                .get(&IVec2::new(x as i32, y as i32))
                .is_some()
            {
                *dir = dir.turn_right();
            } else {
                if let Some(entity) = base_map.get(&pos) {
                    let mut index =
                        tiles.get_mut(entity).unwrap();
                    index.0 = match *dir {
                        Direction::North => 5,
                        Direction::South => 7,
                        Direction::East => 1,
                        Direction::West => 1,
                    };
                };
                *pos = next_position;
                // visited_positions.insert(player_position);
            }
        }
    }
}
