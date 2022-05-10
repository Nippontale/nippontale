pub mod scene_00;

pub struct SceneUpdater {
    pub num: u32,
    pub b: bool
}

impl Default for SceneUpdater {
    fn default() -> Self {
        SceneUpdater { num:  0, b: true }
    }
}