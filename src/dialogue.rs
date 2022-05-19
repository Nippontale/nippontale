use bevy::prelude::*;
use crate::prelude::*;
#[derive(Component, Default)]
pub struct Textbox {
    pub active: bool
} 

pub struct NewTextboxText {
    pub text: String,
    pub complete: String,
    pub i: usize,
    pub is_done: bool,
    pub font_size: f32,
    pub time: Timer
}

impl NewTextboxText {
    pub fn new(s: f32) -> Self {
        Self {
            text: String::new(), 
            complete: String::new(),
            i: 0,
            is_done: true,
            font_size: 20.,
            time: Timer::from_seconds(s, true)
        }
    }
    pub fn new_text<T>(&mut self, s: T, n: f32)
        where T: Into<String> {
        if self.complete == self.text {
            self.is_done = false;
            self.complete = s.into();
            self.i = 0;
            self.text = String::new();
            self.font_size = n;
        }
    }
}

pub fn txb_tick(mut ntt: ResMut<NewTextboxText>, time: Res<Time>) {
    ntt.time.tick(time.delta()); 
    if ntt.time.just_finished() {
        let z = ntt.complete.len();
        if z != 0 {
            ntt.i = std::cmp::min(ntt.i+1, z);
            ntt.text = ntt.complete[..ntt.i].into();
            if ntt.text == ntt.complete {
                ntt.is_done = true;
            }
        }
    }
}

pub fn sync_textbox_vis(mut q: Query<(&mut Textbox, &mut Visibility)>) {
    for (mut txb, mut vis) in q.iter_mut() {
        vis.is_visible = txb.active;
    }
}

pub fn sync_textbox_text(ntt: Res<NewTextboxText>, mut q: Query<(&mut Text), (With<Textbox>)>) {
    for (mut txt) in q.iter_mut() {
        txt.sections[0].value = ntt.text.clone();
        txt.sections[0].style.font_size = ntt.font_size;
    }
}