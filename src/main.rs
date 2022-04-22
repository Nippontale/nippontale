#![allow(clippy::all)]
#![allow(warnings)]

use bevy::prelude::*;

pub mod utils;
pub mod imp;
pub mod base;

/// main function
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(utils::setup)
        .run();
}