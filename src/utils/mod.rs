use bevy::{prelude::*, ecs::event::Events, window::WindowResized};

pub mod battle;
pub mod world;

mod dialogue;
mod graphics;

use world::collisions::Hitbox;

/// game setup
/// TODO: split into multiple setup functions
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, win: Res<WindowDescriptor>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(world::MainCharacter::from(asset_server.load("DEF.png")));
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("DEF.png"),
        transform: Transform::from_xyz(0., 0., 0.),
        sprite: Sprite { custom_size: Some(Vec2::new(100., 100.)), ..Default::default()},
        ..Default::default()
    });
    // right edge
    commands.spawn_bundle(Hitbox::rect(0., 0., 80., 80.));
}
/// prevent window resize from breaking the game
/// TODO change sizes relatively too
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