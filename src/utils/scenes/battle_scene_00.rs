use bevy::prelude::*;
use crate::prelude::*;

pub fn spawn_battle_scene_00(mut commands: Commands, 
    mut scene_updater: ResMut<SceneUpdater>, 
    deletor: Res<Deletor>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut q: Query<(&mut Visibility), With<PlayerControlled>>,
    screen: Res<WindowDescriptor>,
) {
    if scene_updater.b && scene_updater.num == 256 && !deletor.b {
        scene_updater.b = false;
        // for (mut v) in q.iter_mut() {
        //     v.is_visible = false;
        // }
    }
}