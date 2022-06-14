use bevy::prelude::*;
use crate::prelude::*;

pub struct Battle {
    // State {
    //  0: not battle
    //  1: battle
    //  2: choice
    // }
    pub state: i8,
    pub choice: i8,
    pub change: bool,
    pub cool: u32,
}

impl Default for Battle {
    fn default() -> Self {
        Battle { state: 0, choice: 0, change: false, cool: 15 }
    }
}

pub fn handle_battle() {
    
}

fn draw_fight_bar() {

}