use chrono::offset::Local;
use bevy::prelude::*;

fn color<T>(s: T, i: u8) -> String
where T: Into<String> {
    format!("\x1b[38;5;{}m{}\x1b[0m", i, s.into())
}

mod COLORCODES {
    pub const RED: u8 = 196;
    pub const YELLOW: u8 = 220;
    pub const GREEN: u8 = 28;
    pub const BLUE: u8 = 33;
    pub const BLACK: u8 = 0;
}

pub struct Logger {
    level: u8, // 0 = everything, 3 = error only, 4 = nothing
    channel: Vec<(String, u8)>
}

impl Default for Logger {
    fn default() -> Self {
        Logger {
            level: 2,
            channel: Vec::new()
        }
    }
}

pub fn logging_system(mut r: ResMut<Logger>) {
    r.channel.reverse();
    let v = r.channel.pop();
    if let Some((st, lv)) = v {
        if r.level <= lv {
            eprintln!("{}", st);
        }
    }
    r.channel.reverse();
}

impl Logger {

    fn log(&mut self, msg: &'_ str, col: u8, maxlevel: u8) {
        self.channel.push((format!("{} -- {}\n", color(msg, col), Local::now()), maxlevel));
    }

    pub fn error(&mut self, msg: &'_ str) {
        self.log(msg, COLORCODES::RED, 3)
    }

    pub fn warn(&mut self, msg: &'_ str) {
        self.log(msg, COLORCODES::YELLOW, 2)
    }
    
    pub fn debug(&mut self, msg: &'_ str) {
        self.log(msg, COLORCODES::BLUE, 1)
    }
    
    pub fn info(&mut self, msg: &'_ str) {
        self.log(msg, COLORCODES::BLACK, 0)
    }
}
