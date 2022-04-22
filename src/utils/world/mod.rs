use bevy::prelude::*;

pub mod collisions;

use collisions::{HitboxSize, touching};

#[derive(Component, Default)]
pub struct PlayerControlled {
    pub controlled: bool
}

#[derive(Component)]
pub struct Moving {
    pub idle_time: f32,
    pub maxspeed: f32,
    pub currentspeed: f32,
    pub acceleration: f32
}

impl Default for Moving {
    fn default() -> Self {
        Moving { idle_time: 0., maxspeed: 10., currentspeed: 0., acceleration: 1. }
    }
}

#[derive(Bundle, Default)]
pub struct MainCharacter {
    pub player_controlled: PlayerControlled,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub size: collisions::HitboxSize,
    pub sprite: Sprite,
    pub moving: Moving,
    pub texture: Handle<Image>,
    pub visibility: Visibility
}

impl From<Handle<Image>> for MainCharacter {
    fn from(texture: Handle<Image>) -> Self {
        MainCharacter {
            player_controlled:  
            PlayerControlled { controlled: true }, 
            transform: Transform::from_xyz(100., 0., 0.), 
            texture, 
            size: HitboxSize { size: Size { width: 128., height: 128. } }, // size for collision calculation 
            //TODO: system to update size based on sprite size 
            sprite: Sprite { custom_size: Some(Vec2::new(128., 128.)), ..Default::default()},
            ..Default::default()
        }
    }
}

/// System for player controlled entities
/// TODO:
/// 1. Implement acceleration
/// 2. Implement better collisions
pub fn player_movement(keys: Res<Input<KeyCode>>, win: Res<WindowDescriptor>, 
    mut ply: Query<(&mut Transform, &PlayerControlled, &Moving, &Sprite, &HitboxSize)>, 
    mut other: Query<(&Transform, &HitboxSize)>) {
    for (mut tr, pc, mv, sp, hbsize) in ply.iter_mut() {
        if pc.controlled {
            let mut ydelta = 0f32;
            let mut xdelta = 0f32;
            if keys.pressed(KeyCode::S) {
                ydelta -= 1.;
            } 
            if keys.pressed(KeyCode::W) {
                ydelta += 1.;
            }
            if keys.pressed(KeyCode::A) {
                xdelta -= 1.;
            }
            if keys.pressed(KeyCode::D) {
                xdelta += 1.;
            }
            let prev = (tr.translation.x, tr.translation.y);
            tr.translation.y += (ydelta*mv.maxspeed);
            tr.translation.x += (xdelta*mv.maxspeed); // replace both mv.maxspeed with mv.speed and impl acceleration
            for (otr, ohbsize) in other.iter_mut() {
                if touching((hbsize, &tr), (ohbsize, otr)) {
                    tr.translation.x = prev.0;
                    tr.translation.y = prev.1;
                    break;
                }
            }
        }
    }
}

