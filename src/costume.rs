use crate::asset::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct Costume {
    #[serde(flatten)]
    shared: SharedAsset,

    bitmap_resolution: i32,
    rotation_center_x: i32,
    rotation_center_y: i32,
}
