use bevy::prelude::*;
use crate::prelude::*;

pub fn spawn_battle_scene_00(mut commands: Commands, 
    mut scene_updater: ResMut<SceneUpdater>, 
    deletor: Res<Deletor>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut q: Query<(&mut Visibility), With<PlayerControlled>>,
    mut screen_cover: Query<Entity, With<Cover>>,
    screen: Res<WindowDescriptor>,
) {
    if scene_updater.num != 256 { return }

    if scene_updater.transitioning {
        
        if scene_updater.current < scene_updater.length {
            for (mut c) in screen_cover.iter_mut() {
                commands.entity(c).despawn()
            }
            let black_screen_asset = asset_server.load("black_sreen.png");
            spawn_screen_cover(&mut commands, black_screen_asset.clone(), screen, 0.5)
        } else if scene_updater.current == scene_updater.length {
            ()
        }
    }

    else if scene_updater.b && !deletor.b {
        scene_updater.b = false;
        // for (mut v) in q.iter_mut() {
        //     v.is_visible = false;
        // }
        
        
    }
}