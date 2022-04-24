#![allow(clippy::all)]
#![allow(warnings)]
#![feature(slice_take)]

use bevy::prelude::*;
use bevy::text::Text2dBounds;
use bevy::{prelude::*, ecs::event::Events, window::WindowResized};

pub mod physics;
pub mod utils;
pub mod events;
pub mod prelude;
pub mod dialogue;

use prelude::*;


use physics::SyncHitboxSize;
pub use utils::logging::{Logger, logging_system};

mod graphics;

use physics::collisions::HitboxBundle;


/// game setup
/// TODO: split into multiple setup functions
/// mostly for testing purposes rn
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, win: Res<WindowDescriptor>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,) {
    // obtain the spritesheet and create the texture atlas
    // this should be made into its own function
    let texture_handle= asset_server.load("savesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(22.5, 25.), 2, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(-50., -50., 0.),
            sprite: TextureAtlasSprite { custom_size: Some(Vec2::new(96., 96.)), ..Default::default()},
            ..default()
        })
        // HitboxBundle to take care of player - entity collisions
        // this will auto sync with the texture atlas sprite's size
        // so we simply use default.
        .insert(HitboxSize { size: Size { width: 86., height: 86.} })
        // save point event marker, marks this entity 
        // as a save point so it can be used as so
        // by the player.
        .insert(events::Savepoint {})
        // animated bundle to animate the spritesheet 
        // changes sprite every (duration)s 
        // and repeats if (repeating) is set to true
        .insert_bundle(graphics::AnimatedBundle::from_seconds(0.3, true));
    commands.spawn_bundle(physics::MainCharacter::from(asset_server.load("character.png")));
    // Text bundle for the text box
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            "DEFAULT VALUE",
            TextStyle {
                font: asset_server.load("fonts/detbubble.ttf"),
                font_size: 40.,
                color: Color::WHITE
            },
            TextAlignment { horizontal: HorizontalAlign::Center, ..Default::default() }
        ),
        transform: Transform::from_xyz(0.,-win.height/2.+150., 7.),
        ..Default::default()
    }).insert(Textbox::default());
    // Textbox sprite
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("textbox.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(win.width/1.5, 120.)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0., -win.height/2.+120., 6.),
        ..Default::default()
    }).insert(Textbox::default());
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

pub fn sync_hitbox_with_sprite(mut q: Query<(&mut physics::HitboxSize, &Sprite, &SyncHitboxSize)>) {
    for (mut hbsize, spr, sync) in q.iter_mut() {
        if sync.sync {
            if let Some(custom_size) = spr.custom_size {
                hbsize.size.width = custom_size[0];
                hbsize.size.height = custom_size[1];
            } 
        }   
    }
}

pub fn sync_hitbox_with_atlassprite(mut q: Query<(&mut physics::HitboxSize, &TextureAtlasSprite, &SyncHitboxSize)>) {
    for (mut hbsize, spr, sync) in q.iter_mut() {
        if sync.sync {
            if let Some(custom_size) = spr.custom_size {
                hbsize.size.width = custom_size[0];
                hbsize.size.height = custom_size[1];
            } 
        }   
    }
}

/// main function
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1400.0,
            height: 800.0,
            ..Default::default()
        })
        .insert_resource(
            Logger::default()
        )
        .insert_resource(NewTextboxText::new(
            0.1
        ))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(physics::player_movement)
        .add_system(logging_system)
        .add_system(window_size_update)
        .add_system(sync_hitbox_with_sprite)
        .add_system(sync_textbox_text)
        .add_system(sync_hitbox_with_atlassprite)
        .add_system(sync_textbox_vis)
        .add_system(txb_tick)
        .add_system(graphics::animate_sprite)
        .add_system(events::player_use_input)
        .run();
}