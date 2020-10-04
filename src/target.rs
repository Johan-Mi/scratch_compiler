use crate::block::*;
use crate::costume::*;
use crate::id::*;
use crate::sound::*;
use crate::sprite::*;
use crate::stage::*;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize)]
#[serde(untagged)]
pub enum Target {
    Stage(Stage),
    Sprite(Sprite),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedTarget {
    pub is_stage: bool,
    pub name: String,
    pub variables: HashMap<ID, (String, Value)>,
    pub lists: HashMap<ID, (String, Vec<Value>)>,
    pub broadcasts: HashMap<ID, String>,
    pub blocks: HashMap<ID, Block>,
    pub costumes: Vec<Costume>,
    pub sounds: Vec<Sound>,
    pub volume: i32,
    pub layer_order: i32,
}
