use crate::id::*;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Monitor {
    id: ID,
    mode: MonitorMode,
    opcode: String,
    params: HashMap<String, String>,
    sprite_name: Option<String>,
    value: Value,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    visible: bool,
    slider_min: Option<f32>,
    slider_max: Option<f32>,
    is_discrete: bool,
}

#[derive(Serialize)]
enum MonitorMode {
    Default,
    Large,
    Slider,
    List,
}
