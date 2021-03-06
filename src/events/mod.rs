use bevy::prelude::*;
use crate::prelude::*;

#[derive(Component)]
pub struct OnTouch {
    pub scene: Option<LoadingZone>, 
    pub savepoint: Option<Savepoint>
}

pub struct Savepoint;

pub struct LoadingZone {
    pub scene_to: u32,
    // transitions to another scene or nah
    pub transition: bool,
}

impl OnTouch { 
    pub fn scene(scene_to: u32, transition: bool) -> Self {
        OnTouch {
            scene: Some(LoadingZone { scene_to, transition }),
            savepoint: None
        }
    }
    pub fn svpt() -> Self {
        OnTouch {
            scene: None,
            savepoint: Some(Savepoint {})
        }
    }
}

pub fn player_use_input(
    keys: Res<Input<KeyCode>>, 
    mut deletor: ResMut<Deletor>,
    mut scene_updater: ResMut<SceneUpdater>,
    mut q: Query<(&PlayerControlled, &mut Touching, &mut Visibility), Without<Textbox>>, 
    mut q2: Query<(&mut Textbox, &mut Visibility, Option<&mut Text>)>,
    mut logger: ResMut<Logger>, mut ntt: ResMut<NewTextboxText>
) {
    for (ply, mut tch, mut v) in q.iter_mut() {
        // This character is being controlled
        if ply.controlled && 
            // This character is touching a save point
            tch.savepoint && 
            // The player just pressed "E"
            keys.just_pressed(KeyCode::E) && 
            // Not currently in dialogue
            !tch.in_scene {
            // makes it so you are not considered to be touching the
            // the save point anymore to prevent freaky things from happening
            tch.savepoint = false;
            tch.in_scene = true;
            ntt.new_text("You are filled with pride and honor!", 20.);
            ntt.text = String::from("");
            for (mut txb, mut vis, mut txt) in q2.iter_mut() {
                txb.active = true;
                vis.is_visible = txb.active;
                if let Some(mut tx) = txt {
                    tx.sections[0].value = String::from("");
                } 
            }
        } else if keys.just_pressed(KeyCode::E) && ntt.complete == ntt.text && tch.in_scene {
            for (mut txb, mut vis, txt) in q2.iter_mut() {
                txb.active = false;
                vis.is_visible = txb.active;
            }
            ntt.is_done = false;
            tch.in_scene = false;
        } else if keys.just_pressed(KeyCode::E) {
            ntt.text = ntt.complete.clone();
            ntt.i = ntt.complete.len();
            ntt.is_done = true;
        }
        // DEBUG COMMANDS
        if keys.just_pressed(KeyCode::F) {
            deletor.b = true
        }
        if keys.just_pressed(KeyCode::G) {
            scene_updater.b = true
        }
        if keys.just_pressed(KeyCode::R) {
            v.is_visible = true;
        }
        if keys.just_pressed(KeyCode::T) {
            v.is_visible = false;
        }
    }
}
