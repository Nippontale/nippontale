use bevy::{prelude::*, ecs::event::Events, window::WindowResized};

pub mod battle;
pub mod world;

mod dialogue;
mod graphics;

/// game setup
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(world::MainCharacter::from(asset_server.load("DEF.png")));
}
/// prevent window resize from breaking the game
pub fn window_size_update(resize: Res<Events<WindowResized>>, mut win: ResMut<WindowDescriptor>, mut trq: Query<(&mut Transform)>) {
    let mut reader = resize.get_reader();
    for e in reader.iter(&resize) {
        for (mut tr) in trq.iter_mut() {
            tr.translation.x = (tr.translation.x/win.width)*e.width;
            tr.translation.y = (tr.translation.y/win.height)*e.height;
        }
        win.width = e.width;
        win.height = e.height;
    }
}