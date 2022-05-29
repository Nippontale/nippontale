use bevy::prelude::*;
use crate::prelude::*;

pub mod scene_00;
pub mod battle_scene_00;

pub struct SceneUpdater {
    pub num: u32,
    pub b: bool
}

impl Default for SceneUpdater {
    fn default() -> Self {
        SceneUpdater { num:  0, b: true }
    }
}

pub fn spawn_savepoint(mut commands: &mut Commands, x: f32, y: f32, tat: Handle<TextureAtlas>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: tat,
            transform: Transform::from_xyz(x, y, 0.),
            sprite: TextureAtlasSprite { custom_size: Some(Vec2::new(64., 64.)), ..Default::default()},
            ..default()
        })
        // deleted as part of the map
        .insert(Map {})
        // HitboxBundle to take care of player - entity collisions
        // this will auto sync with the texture atlas sprite's size
        // so we simply use default.
        .insert(HitboxSize { size: Size { width: 52., height: 52.} })
        // save point event marker, marks this entity 
        // as a save point so it can be used as so
        // by the player.
        .insert(events::Savepoint {})
        // animated bundle to animate the spritesheet 
        // changes sprite every (duration)s 
        // and repeats if (repeating) is set to true
        .insert_bundle(graphics::AnimatedBundle::from_seconds(0.3, true));
}

pub fn spawn_loading_zone(mut commands: &mut Commands, x: f32, y: f32, width: f32, height: f32, scene_to: u32) {
    commands
        // spawn the loading zone
        .spawn()
        // inserted a map component so it's destroyed when changing scenes
        .insert(Map {})
        // inserted a transform component for it's position
        .insert(Transform::from_xyz((x.abs()+width)*(x/x.abs()), y, 0.))

        .insert(HitboxSize { size: Size {width, height} })

        .insert(events::LoadingZone { scene_to });
}

pub fn spawn_pass_tile(mut commands: &mut Commands, x: f32, y: f32, z: f32, tat: Handle<Image>) {
    commands
        .spawn_bundle(SpriteBundle {
        texture: tat,
        sprite: Sprite {
            custom_size: Some(Vec2::new(64., 64.)),
            ..Default::default()
        },
        transform: Transform::from_xyz(x, y, z),
        ..Default::default()
        })
        // deleted as part of the map
        .insert(Map {});
}
