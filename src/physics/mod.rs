use bevy::prelude::*;

pub mod collisions;

pub use collisions::HitboxSize;
pub use collisions::HitboxBundle;
use collisions::touching;

use crate::Logger;

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


#[derive(Component)]
pub struct SyncHitboxSize {
    pub sync: bool
}

impl Default for SyncHitboxSize {
    fn default() -> Self {
        return SyncHitboxSize { sync: true }
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
    pub visibility: Visibility,
    pub sync_hitbox_size: SyncHitboxSize,
    pub touching: Touching
}

#[derive(Component)]
pub struct Touching {
    savepoint: bool
}

impl From<Handle<Image>> for MainCharacter {
    fn from(texture: Handle<Image>) -> Self {
        MainCharacter {
            player_controlled:  
            PlayerControlled { controlled: true }, 
            transform: Transform::from_xyz(200., 100., 0.), 
            texture, 
            sprite: Sprite { custom_size: Some(Vec2::new(128., 128.)), ..Default::default()},
            sync_hitbox_size: SyncHitboxSize { sync: false },
            size: collisions::HitboxSize { size: Size { width: 64., height: 64.}},
            ..Default::default()
        }
    }
}

/// System for player controlled entities
/// TODO:
/// 1. Implement acceleration
/// 2. Implement better collisions
pub fn player_movement(keys: Res<Input<KeyCode>>, win: Res<WindowDescriptor>, 
    mut ply: Query<(&mut Transform, &PlayerControlled, &Moving, &Sprite, &HitboxSize, &mut Touching)>,
    mut logger: ResMut<Logger>, 
    mut other: Query<(&Transform, &HitboxSize, Option<&crate::events::Savepoint>), Without<PlayerControlled>>) {
    for (mut tr, pc, mv, sp, hbsize, mut tch) in ply.iter_mut() {
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
            tr.translation.x += (xdelta*mv.maxspeed);
            if (xdelta != 0. || ydelta != 0.) {
                tch.savepoint = false;
                for (otr, ohbsize, svpt) in other.iter_mut() {
                    if touching((hbsize, &tr), (ohbsize, otr)) || touching((ohbsize, otr), (hbsize, &tr)) {
                        logger.info("Collision!");
                        tr.translation.x = prev.0;
                        tr.translation.y = prev.1;
                        if let Some(is_svpt) = svpt {
                            logger.warn("Touching save point!");
                            tch.savepoint = true
                        }
                        break;
                    }
                }
            }  
        }
    }
}

