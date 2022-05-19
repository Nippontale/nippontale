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
        let wood_plank_asset: Handle<Image> = asset_server.load("wooden-plank.png");
        let mut spawn_wood_plank = move |x, y| {
            spawn_pass_tile(&mut commands, x, y, 0., wood_plank_asset.clone());
        };
        let mut spawn_wood_planks = |v: &[(f32, f32)]| {
            for (x, y) in v {
                spawn_wood_plank(*x, *y);
            }
        };
        spawn_wood_planks(&[(-78., 55.), (-14., 55.), (50., 55.), (114., 55.), (178., 55.)]);
    }
}