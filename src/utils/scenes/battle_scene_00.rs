use bevy::prelude::*;
use crate::prelude::*;
use bevy::render::color::*;

pub fn spawn_battle_scene_00(mut commands: Commands, 
    mut scene_updater: ResMut<SceneUpdater>, 
    deletor: Res<Deletor>,
    mut bg_color: ResMut<ClearColor>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut q: Query<(&mut Visibility), With<PlayerControlled>>,
    screen: Res<WindowDescriptor>,
) {
    if scene_updater.b && scene_updater.num == 256 && !deletor.b {
        scene_updater.b = false;
        println!("{:?}", bg_color);
        bg_color = (Color::hsla(252., 0.33, 0.18, 0.0));
        // for (mut v) in q.iter_mut() {
        //     v.is_visible = false;
        // }
    }
}