use std::panic::RefUnwindSafe;

use bevy::prelude::*;
use crate::prelude::*;

use crate::Logger;

use super::SyncHitboxSize;

#[derive(Component, Default)]
pub struct Blocking;

#[derive(Component, Default)]
pub struct HitboxSize {
    pub size: Size,
    pub ydelta: f32,
    pub xdelta: f32
}

#[derive(Component)]
pub struct Map;

#[derive(Bundle, Default)]
pub struct HitboxBundle {
    pub size: HitboxSize,
    pub sync: SyncHitboxSize,
    pub blocking: Blocking
}

impl HitboxBundle {
    pub fn rect(x: f32, y: f32, w: f32, h: f32) -> Self {
        HitboxBundle {
            size: HitboxSize { size: Size { width: w, height: h}, xdelta: 0.,  ydelta: 0. }, 
            sync: SyncHitboxSize { sync: false },
            ..Default::default() 
        }
    }
}

fn top(ent: cancollide<'_>) -> f32 {
    ent.1.translation.y + ent.0.size.height/2. + ent.0.ydelta
}

fn bottom(ent: cancollide<'_>) -> f32 {
    ent.1.translation.y - ent.0.size.height/2. + ent.0.ydelta
}

fn right(ent: cancollide<'_>) -> f32 {
    ent.1.translation.x + ent.0.size.width/2. + ent.0.xdelta
}

fn left(ent: cancollide<'_>) -> f32 {
    ent.1.translation.x - ent.0.size.width/2. + ent.0.xdelta
}

pub type cancollide<'a> = (&'a HitboxSize, &'a Transform);

fn withinx(a: (f32, f32), b: cancollide<'_>) -> bool {
    let x = a.0 <= right(b) && a.0 >= left(b);
    x
}

fn withiny(a: (f32, f32), b: cancollide<'_>) -> bool {
    let x = a.1 <= top(b) && a.1 >= bottom(b);
    x
}

pub fn within(a: (f32, f32), b: cancollide<'_>) -> bool {
    withinx(a, b) && withiny(a, b)
}


pub fn touching(a: cancollide<'_>, b: cancollide<'_>) -> bool {
    for x in [
        (right(a), top(a)), 
        (left(a), top(a)), 
        (right(a), bottom(a)), 
        (left(a), bottom(a))
    ] {
        if within(x, b) {
            return true
        }
    }
    false
}
