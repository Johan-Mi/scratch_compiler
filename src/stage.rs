use crate::block::*;
use crate::costume::*;
use crate::id::*;
use crate::sound::*;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stage {
    is_stage: bool,
    name: String,
    variables: HashMap<ID, (String, Value)>,
    lists: HashMap<ID, (String, Vec<Value>)>,
    broadcasts: HashMap<ID, String>,
    pub blocks: Vec<Block>,
    pub costumes: Vec<Costume>,
    sounds: Vec<Sound>,
    volume: i32,
    layer_order: i32,
}

impl Stage {
    pub fn new() -> Self {
        Self {
            is_stage: true,
            name: "Stage".to_string(),
            variables: HashMap::new(),
            lists: HashMap::new(),
            broadcasts: HashMap::new(),
            blocks: Vec::new(),
            costumes: Vec::new(),
            sounds: Vec::new(),
            volume: 0,
            layer_order: 0,
        }
    }
}
