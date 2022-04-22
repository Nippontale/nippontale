use std::panic::RefUnwindSafe;

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Blocking;

#[derive(Component, Default)]
pub struct HitboxSize {
    pub size: Size
}
#[derive(Bundle, Default)]
pub struct Hitbox {
    pub transform: Transform,
    pub size: HitboxSize,
    pub blocking: Blocking
}

pub enum Edge {
    Bottom,
    Top,
    Right,
    Left
}

impl Edge {
    fn of(&self, ent: cancollide<'_>) -> f32 {
        match self {
            Bottom => ent.1.translation.y - ent.0.size.height/2.,
            Top => ent.1.translation.y + ent.0.size.height/2.,
            Right => ent.1.translation.x+ent.0.size.width/2.,
            Left => ent.1.translation.x-ent.0.size.width/2.
        }
    }
}

pub type cancollide<'a> = (&'a HitboxSize, &'a Transform);

pub fn touching(a: cancollide<'_>, b: cancollide<'_>) -> bool {
    // upper touching
    if (Edge::Top.of(a)) >= (Edge::Bottom.of(b)) 
    // bottom touching
    || (Edge::Bottom.of(a)) <= (Edge::Top.of(b))
    // left touching
    || (Edge::Left.of(a)) <= (Edge::Right.of(b))
    // right touching
    || (Edge::Right.of(a)) >= (Edge::Left.of(b)) {
        return true;
    }
    false
}
