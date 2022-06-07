use bevy::prelude::*;
use crate::prelude::*;

pub fn spawn_battle_scene_00(mut commands: Commands, 
    mut scene_updater: ResMut<SceneUpdater>, 
    mut deletor: ResMut<Deletor>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut q: Query<(&mut Visibility), With<PlayerControlled>>,
    mut screen_cover: Query<Entity, With<Cover>>,
    mut battle: ResMut<Battle>,
    screen: Res<WindowDescriptor>,
) {
    if scene_updater.num != 256 { return }
    
    if scene_updater.b && !deletor.b {
        scene_updater.b = false;
        // for (mut v) in q.iter_mut() {
        //     v.is_visible = false;
        // };
        battle.state = 2;
    }   

    if scene_updater.transitioning {
        if scene_updater.current < scene_updater.length {
            for (mut c) in screen_cover.iter_mut() {
                commands.entity(c).despawn()
            }
            let black_screen_asset = asset_server.load("black-cover.png");
            if !scene_updater.transitioned {
                scene_updater.current += 1.;
            } else if scene_updater.current > 0. {
                scene_updater.current -= 1.;
            } else {
                scene_updater.transitioned = false;
                scene_updater.transitioning = false;
                return;
            }
            let opacity: f32 = scene_updater.current/scene_updater.length;
            spawn_screen_cover(&mut commands, &screen, opacity, black_screen_asset.clone());
        } else if scene_updater.current == scene_updater.length {
            scene_updater.transitioned = true;
            scene_updater.current -= 1.;
            deletor.b = true;
            scene_updater.b = true;
            battle.state = 1;
            battle.choice = 0;
            battle.change = true;
        }
    }   
}