#![allow(clippy::all)]
#![allow(warnings)]

use bevy::prelude::*;

pub mod utils;
pub mod imp;
pub mod base;

/// main function
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1400.0,
            height: 800.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(utils::setup)
        .add_system(utils::world::player_movement)
        .add_system(utils::window_size_update)
        .run();
}