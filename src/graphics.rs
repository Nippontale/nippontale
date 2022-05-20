use bevy::prelude::*;
use crate::prelude::*;

#[derive(Component, Deref, DerefMut, Default)]
pub struct AnimationTimer(pub Timer);

pub fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    ), Without<PlayerControlled>>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

pub fn anime_moving_char(
    time: Res<Time>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &Moving,
        &Touching
    ), With<PlayerControlled>>,
) {
    for (mut timer, mut sprite, texture_atlas_handle , mut mv, tch) in query.iter_mut() {
        if !tch.in_scene {
            let v = match mv.direction {
                0 => ("0-walking.png", 3),
                1 => ("1-walking.png", 2),
                2 => ("2-walking.png", 3),
                _ => ("3-walking.png", 2)
            };
            let t = texture_atlases
                .set(texture_atlas_handle, TextureAtlas::from_grid(asset_server.load(v.0), Vec2::new(20., 30.), v.1, 1));
            sprite.index %= texture_atlases.get(t).unwrap().textures.len();
            if mv.t {
                timer.tick(time.delta());
                if timer.just_finished() {
                    let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                }
            } else {
                sprite.index = 0
            }        
        }
    }
}

#[derive(Bundle)]
pub struct AnimatedBundle {
    pub animation_timer: AnimationTimer
}

impl AnimatedBundle {
    pub fn from_seconds(duration: f32, repeating: bool) -> Self {
        AnimatedBundle { animation_timer:  AnimationTimer(Timer::from_seconds(duration, repeating)) }
    }
}
