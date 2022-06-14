use bevy::prelude::*;
use crate::prelude::*;

pub fn spawn_scene_01(mut commands: Commands, 
    mut scene_updater: ResMut<SceneUpdater>, 
    deletor: Res<Deletor>,
    mut asset_server: ResMut<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    screen: Res<WindowDescriptor>,
    mut asset_handles: ResMut<AssetHandles>,

) {
    if scene_updater.num != 1 { return }

    if asset_handles.scene_saved != 1 {
        // asset_handles.handles = Vec::new();
        let bg_assets = [
            "wooden-plank.png",
            "brick-wall.png",
            "black-cover.png",
        ];
        for path in bg_assets {
            asset_handles.handles.push(asset_server.load(path))
        }
    }

    if scene_updater.b && !deletor.b {
        scene_updater.b = false;
        // obtain the spritesheet and create the texture atlas
        // this should be made into its own function
        let texture_handle = asset_server.load("savesheet.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(22.5, 25.), 2, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        spawn_savepoint(&mut commands, 50., -100., texture_atlas_handle.clone());

        let wood_plank_asset = asset_server.load("wooden-plank.png");

        const plank_width: f32 = 64.;
        const nb_rows: i32 = 2;
        const nb_columns: i32 = 10;

        spawn_loading_zone(&mut commands, -(screen.width/2.)-32., -plank_width, 100., plank_width, 0, false);
        spawn_loading_zone(&mut commands, -32., -plank_width, 100., plank_width, 256, true);

        for y in ((-1*(nb_rows/2))..(nb_rows/2)) {
            for x in ((-1*(nb_columns/2))..(nb_columns/2)) {
                spawn_pass_tile(&mut commands, (x as f32)*plank_width-plank_width, (y as f32)*plank_width-plank_width, 0., wood_plank_asset.clone());
            }
        }
    }
}

