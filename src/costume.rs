use crate::asset::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct Costume {
    #[serde(flatten)]
    pub shared: SharedAsset,

    bitmap_resolution: i32,
    rotation_center_x: i32,
    rotation_center_y: i32,
}

impl Costume {
    pub fn new(name: String) -> Self {
        Self {
            shared: SharedAsset::new(name),
            bitmap_resolution: 1,
            rotation_center_x: 0,
            rotation_center_y: 0,
        }
    }

    pub fn resolve(&mut self) {
        self.shared.resolve();
    }
}
