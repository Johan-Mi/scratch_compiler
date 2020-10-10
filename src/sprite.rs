use crate::block::*;
use crate::costume::*;
use crate::id::*;
use crate::sound::*;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sprite {
    is_stage: bool,
    name: String,
    variables: HashMap<ID, (String, Value)>,
    lists: HashMap<ID, (String, Vec<Value>)>,
    broadcasts: HashMap<ID, String>,
    #[serde(serialize_with = "serialize_block_vec")]
    blocks: Vec<Block>,
    pub costumes: Vec<Costume>,
    sounds: Vec<Sound>,
    volume: i32,
    layer_order: i32,

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
