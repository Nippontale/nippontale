#![allow(clippy::all)]
#![allow(warnings)]

use bevy::prelude::*;

pub mod game;

/// main function
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1400.0,
            height: 800.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(game::setup)
        .add_system(game::physics::player_movement)
        .add_system(game::window_size_update)
        .add_system(game::sync_hitbox_with_sprite)
        .run();
}