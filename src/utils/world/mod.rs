use bevy::prelude::*;

#[derive(Component, Default)]
pub struct PlayerControlled {
    pub controlled: bool
}

#[derive(Component)]
pub struct Moving {
    pub idle_time: f32,
    pub maxspeed: f32,
    pub currentspeed: f32,
    pub acceleration: f32
}

impl Default for Moving {
    fn default() -> Self {
        Moving { idle_time: 0., maxspeed: 10., currentspeed: 0., acceleration: 1. }
    }
}

#[derive(Bundle, Default)]
pub struct MainCharacter {
    pub player_controlled: PlayerControlled,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub sprite: Sprite,
    pub moving: Moving,
    pub texture: Handle<Image>,
    pub visibility: Visibility
}

impl From<Handle<Image>> for MainCharacter {
    fn from(texture: Handle<Image>) -> Self {
        MainCharacter {
            player_controlled:  
            PlayerControlled { controlled: true }, 
            transform: Transform::from_xyz(100., 0., 0.), 
            texture, 
            sprite: Sprite { custom_size: Some(Vec2::new(128., 128.)), ..Default::default()},
            ..Default::default()
        }
    }
}

/// System for player controlled entities
/// TODO:
/// 1. Implement acceleration
/// 2. Implement better collisions
pub fn player_movement(keys: Res<Input<KeyCode>>, win: Res<WindowDescriptor>, mut query: Query<(&mut Transform, &PlayerControlled, &Moving, &Sprite)>) {
    for (mut tr, pc, mv, sp) in query.iter_mut() {
        if pc.controlled {
            let mut ydelta = 0f32;
            let mut xdelta = 0f32;
            let yhalfsize = sp.custom_size.unwrap().to_array()[1]/2.;
            let xhalfsize = sp.custom_size.unwrap().to_array()[0]/2.;
            if keys.pressed(KeyCode::S) && tr.translation.y > -(win.height/2.-yhalfsize) { // todo remove unwrap
                ydelta -= 1.;
            } 
            if keys.pressed(KeyCode::W) && tr.translation.y < (win.height/2.-yhalfsize) {
                ydelta += 1.;
            }
            if keys.pressed(KeyCode::A) && tr.translation.x > -(win.width/2.-xhalfsize) {
                xdelta -= 1.;
            }
            if keys.pressed(KeyCode::D) && tr.translation.x < (win.width/2.-xhalfsize) {
                xdelta += 1.;
            }
            tr.translation.y += (ydelta*mv.maxspeed);
            tr.translation.x += (xdelta*mv.maxspeed); // replace both mv.maxspeed with mv.speed and impl acceleration
        }
    }
}

