use bevy::prelude::*;
use bevy_ecs_ldtk::ldtk::LdtkJson;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::assets::{LdtkProjectLoaderSettings, LdtkProjectLoader, Value};

mod coin;
mod player;
mod respawn;

pub fn map_generation(mut map_json: LdtkJson) -> Result<LdtkJson, ()> {
    
    let level = map_json.levels.get_mut(0).unwrap();
    level.world_x += -100;

    println!("awsome map transformation");

    return Ok(map_json);
}


fn main() {
    let loader = LdtkProjectLoader{
        callback: Some(Box::new(|map_json, config| {
            let v = config.get("value").unwrap();
            if let Value::Int(seed) = v {
                println!("config passed down for generation {}", seed);
            }
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

    let ldtk_handle = asset_server.load_with_settings("collectathon.ldtk", |s: &mut LdtkProjectLoaderSettings| {
        s.data.insert("value".into(), Value::Int(24242));
    });

    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}
