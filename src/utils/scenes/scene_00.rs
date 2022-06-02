use bevy::prelude::*;
use crate::prelude::*;

pub fn spawn_scene_00(mut commands: Commands, 
    mut scene_updater: ResMut<SceneUpdater>, 
    deletor: Res<Deletor>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    screen: Res<WindowDescriptor>,
) {
    if scene_updater.b && scene_updater.num == 0 && !deletor.b {
        scene_updater.b = false;
        // obtain the spritesheet and create the texture atlas
        // this should be made into its own function
        let texture_handle = asset_server.load("savesheet.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(22.5, 25.), 2, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let wood_plank_asset = asset_server.load("wooden-plank.png");
        let brick_wall_asset = asset_server.load("brick-wall.png");

        const wall_width: f32 = 64.;
        let mut nb_rows: i32 = 2;
        let mut nb_columns: i32 = 12;
        let mut z = 0.;
        
        spawn_loading_zone(&mut commands, screen.width/2., -wall_width, 100., wall_width, 1, false);

        
        nb_rows = 1;
        z = 2.;

        for y in 0..nb_rows{
            for x in 0..nb_columns {
                spawn_wall_tile(&mut commands, x as f32*wall_width-(wall_width*4.), y as f32*wall_width, z, brick_wall_asset.clone());;
            }
        }

        for y in 0..nb_rows{
            for x in 0..nb_columns {
                spawn_wall_tile(&mut commands, x as f32*wall_width-(wall_width*4.), y as f32*wall_width-(wall_width*2.5), z, brick_wall_asset.clone());;
            }
        }
        
        nb_rows = 2;
        z = 0.;

        for y in -1*(nb_rows/2)..(nb_rows/2) {
            for x in -1*(nb_columns/2)..(nb_columns/2) {
                spawn_pass_tile(&mut commands, x as f32*wall_width+(wall_width*2.), y as f32*wall_width-(wall_width), z, wood_plank_asset.clone());
            }
        }
    }
}