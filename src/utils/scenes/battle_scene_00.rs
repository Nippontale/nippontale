use bevy::prelude::*;
use crate::prelude::*;

pub fn spawn_battle_scene_00(
    mut commands: Commands, 
    mut scene_updater: ResMut<SceneUpdater>, 
    mut deletor: ResMut<Deletor>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut q: Query<(&mut Visibility), With<PlayerControlled>>,
    mut screen_cover: Query<Entity, With<Cover>>,
    mut battle: ResMut<Battle>,
    screen: Res<WindowDescriptor>,
    mut asset_handles: ResMut<AssetHandles>,
) {
    if scene_updater.num != 256 { return }

    if scene_updater.b && !deletor.b {
        scene_updater.b = false;  
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
            };
            let opacity: f32 = scene_updater.current/scene_updater.length;
            spawn_screen_cover(&mut commands, &screen, opacity, black_screen_asset.clone());
        } else if scene_updater.current == scene_updater.length {
            scene_updater.transitioned = true;
            scene_updater.current -= 1.;
            deletor.b = true;
            scene_updater.b = true;
            for (mut v) in q.iter_mut() {
                v.is_visible = false;
            };
            
            battle.state = 2;
            battle.choice = 5;
            battle.change = true;
        }
    }

    if asset_handles.scene_saved != 256 {
        asset_handles.scene_saved = 256;
        let bg_assets = [
            "black-cover.png",
            "0-battle.png",
            "1-choice-fight.png",
            "2-choice-act.png",
            "3-choice-item.png",
            "4-choice-mercy.png",
            "5-battle-in-progress.png",
            "fight-bar.png",
        ];
        for path in bg_assets {
            asset_handles.handles.push(asset_server.load(path));
        }
    }
}