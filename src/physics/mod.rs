use bevy::prelude::*;
use crate::prelude::*;

pub mod collisions;

pub use collisions::HitboxSize;
pub use collisions::HitboxBundle;
use collisions::touching;

use crate::Logger;

#[derive(Component)]
pub struct Moving {
    pub idle_time: f32,
    pub maxspeed: f32,
    pub currentspeed: f32,
    pub acceleration: f32,
    pub t: bool,
    pub direction: u8
}

impl Default for Moving {
    fn default() -> Self {
        Moving { idle_time: 0., maxspeed: 10., currentspeed: 0., acceleration: 1., t: false, direction: 0}
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

#[derive(Component, Default)]
pub struct Touching {
    pub savepoint: bool,
    pub in_scene: bool
}

pub fn player_movement(keys: Res<Input<KeyCode>>, win: Res<WindowDescriptor>, 
    mut ply: Query<(&mut Transform, &PlayerControlled, &mut Moving, &TextureAtlasSprite, &HitboxSize, &mut Touching)>,
    mut logger: ResMut<Logger>, 
    mut ntt: ResMut<NewTextboxText>,
    mut other: Query<(&Transform, &HitboxSize, Option<&crate::events::Savepoint>), Without<PlayerControlled>>) {
    for (mut tr, pc, mut mv, sp, hbsize, mut tch) in ply.iter_mut() {
        if pc.controlled && !tch.in_scene {
            let mut ydelta = 0f32;
            let mut xdelta = 0f32;
            for v in [(KeyCode::S, -1.), (KeyCode::W, 1.)] { if keys.pressed(v.0) { ydelta += v.1 }}
            for v in [(KeyCode::A, -1.), (KeyCode::D, 1.)] { if keys.pressed(v.0) { xdelta += v.1 }}
            let prev = (tr.translation.x, tr.translation.y);
            tr.translation.y += (ydelta*mv.maxspeed);
            tr.translation.x += (xdelta*mv.maxspeed);
            if (xdelta != 0. || ydelta != 0.) {
                if xdelta > 0. { mv.direction = 1}
                else if xdelta < 0. { mv.direction = 3}
                else if ydelta > 0. { mv.direction = 0 }
                else if ydelta < 0. {mv.direction = 2}
                else  {mv.direction = 0};
                mv.t = true;
                tch.savepoint = false;
                for (otr, ohbsize, svpt) in other.iter_mut() {
                    if touching((hbsize, &tr), (ohbsize, otr)) || touching((ohbsize, otr), (hbsize, &tr)) {
                        logger.info("Collision!");
                        tr.translation.x = prev.0;
                        tr.translation.y = prev.1;
                        if let Some(is_svpt) = svpt {
                            tch.savepoint = true
                        }
                        mv.t = false;
                        break;
                    }
                }
            } else {
                mv.t = false;
            }
        }
    }
}

