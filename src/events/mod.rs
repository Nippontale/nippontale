use bevy::prelude::*;
use crate::prelude::*;

#[derive(Component)]
pub struct Savepoint;

pub fn player_use_input(keys: Res<Input<KeyCode>>, q: Query<(&PlayerControlled, &Touching)>, mut logger: ResMut<Logger>) {
    for (ply, tch) in q.iter() {
        if ply.controlled && tch.savepoint && keys.just_released(KeyCode::E) {
            logger.error("PRESSED TO SAVE!")
        }
    }
}