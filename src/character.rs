use bevy::prelude::*;
use crate::prelude::*;

#[derive(Component, Default)]
pub struct PlayerControlled {
    pub controlled: bool
}

#[derive(Bundle, Default)]
pub struct MainCharacter {
    pub player_controlled: PlayerControlled,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub size: HitboxSize,
    pub sprite: TextureAtlasSprite,
    pub texture_atlas: Handle<TextureAtlas>,
    pub moving: Moving,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
    pub sync_hitbox_size: SyncHitboxSize,
    pub touching: Touching,
    pub animation_timer: AnimationTimer
}

impl From<Handle<TextureAtlas>> for MainCharacter {
    fn from(texture_atlas: Handle<TextureAtlas>) -> Self {
        MainCharacter {
            player_controlled:  
            PlayerControlled { controlled: true }, 
            transform: Transform::from_xyz(200., 100., 5.), 
            texture_atlas, 
            sprite: TextureAtlasSprite { custom_size: Some(Vec2::new(75., 90.)), ..Default::default()},
            sync_hitbox_size: SyncHitboxSize { sync: false },
            size: HitboxSize { size: Size { width: 25., height: 25.}, xdelta: 0., ydelta: 0. },
            animation_timer: crate::graphics::AnimationTimer(Timer::from_seconds(0.19, true)),
            ..Default::default()
        }
    }
}