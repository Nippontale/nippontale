use bevy::prelude::*;

mod battle;
mod world;

mod dialogue;
mod graphics;

/// game setup
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}