use bevy::prelude::*;
use crate::prelude::*;

use bevy::prelude::*;
use crate::prelude::*;

pub fn spawn_scene_08(mut commands: Commands, 
    mut scene_updater: ResMut<SceneUpdater>, 
    deletor: Res<Deletor>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    screen: Res<WindowDescriptor>,
) {
    if scene_updater.b && scene_updater.num == 8 && !deletor.b {
        scene_updater.b = false;
        
        let texture_handle = asset_server.load("savesheet.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(22.5, 25.), 2, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        spawn_savepoint(&mut commands, 50., 55., texture_atlas_handle.clone());
    }
}