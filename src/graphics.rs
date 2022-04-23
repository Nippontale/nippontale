use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
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
