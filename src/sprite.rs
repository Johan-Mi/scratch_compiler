use crate::target::*;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sprite {
    #[serde(flatten)]
    pub shared: SharedTarget,

    visible: bool,
    x: i32,
    y: i32,
    size: f32,
    direction: f32,
    draggable: bool,
    rotation_style: RotationStyle,
}

#[derive(Serialize)]
enum RotationStyle {
    #[serde(rename = "all around")]
    AllAround,
    #[serde(rename = "left-right")]
    LeftRight,
    #[serde(rename = "don't rotate")]
    DontRotate,
}
