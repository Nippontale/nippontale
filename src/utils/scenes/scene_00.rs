use bevy::prelude::*;
use crate::prelude::*;

pub fn spawn_savepoint(mut commands: &mut Commands, x: f32, y: f32, tat: Handle<TextureAtlas>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: tat,
            transform: Transform::from_xyz(x, y, 0.),
            sprite: TextureAtlasSprite { custom_size: Some(Vec2::new(96., 96.)), ..Default::default()},
            ..default()
        })
        // deleted as part of the map
        .insert(Map {})
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
}
pub fn spawn_scene_00(mut commands: Commands, 
    mut scene_updater: ResMut<SceneUpdater>, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    if scene_updater.b && scene_updater.num == 0 {
        scene_updater.b = false;
        // obtain the spritesheet and create the texture atlas
        // this should be made into its own function
        let texture_handle= asset_server.load("savesheet.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(22.5, 25.), 2, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        spawn_savepoint(&mut commands, 50., 55., texture_atlas_handle.clone());
        spawn_savepoint(&mut commands, -50., 55., texture_atlas_handle.clone());
        spawn_savepoint(&mut commands, -200., 250., texture_atlas_handle.clone());
    }
}