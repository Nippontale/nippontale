use bevy::prelude::*;
use crate::prelude::*;

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
        //spawn_savepoint(&mut commands, -50., 55., texture_atlas_handle.clone());
        //spawn_savepoint(&mut commands, -200., 250., texture_atlas_handle.clone());
    }
}