use bevy::prelude::*;
use bevy_ecs_ldtk::assets::{LdtkProjectLoader, LdtkProjectLoaderSettings};
use bevy_ecs_ldtk::ldtk::LdtkJson;
use bevy_ecs_ldtk::prelude::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod coin;
mod player;
mod respawn;

static SEED: i32 = 12345;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyConfigStruct {
    pub seed: i32,
}

pub fn map_generation(mut map_json: LdtkJson) -> Result<LdtkJson, ()> {
    let level = map_json.levels.get_mut(0).unwrap();
    level.world_x += -100;

    println!("awsome map transformation");

    return Ok(map_json);
}

fn main() {
    let loader = LdtkProjectLoader {
        callback: Some(Box::new(|map_json, config| {
            let config: MyConfigStruct = serde_json::from_value(serde_json::Value::Object(config))
                .expect("Failed to convert value to struct");

            println!("{:#?}", config);

            map_generation(map_json).unwrap()
        })),
    };

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::iid("34f51d20-8990-11ee-b0d1-cfeb0e9e30f6"))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            ..default()
        })
        .add_systems(Startup, setup)
        .add_plugins((
            coin::CoinPlugin,
            player::PlayerPlugin,
            respawn::RespawnPlugin,
        ))
        .register_asset_loader(loader)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    commands.spawn(camera);

    let ldtk_handle = asset_server.load_with_settings(
        "collectathon.ldtk",
        |s: &mut LdtkProjectLoaderSettings| {
            let config = MyConfigStruct { seed: SEED };

            s.data = serde_json::to_value(&config)
                .expect("Failed to convert struct to value")
                .as_object()
                .expect("Failed to convert value to object")
                .clone();
        },
    );

    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}
