pub struct State {
    pub save: u16
    // add more save data as needed
}

impl State {
    pub fn new() -> Self {
        State { save: 0 }
    }
}