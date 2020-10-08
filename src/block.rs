use crate::id::*;
use crate::primitive::*;
use serde::Serialize;
use serde_json::Value;
use std::boxed::Box;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Block {
    id: ID,

    pub opcode: String,
    next: Option<Box<Block>>,
    // parent: Option<ID>, // TODO
    inputs: HashMap<String, BlockInput>,
    fields: HashMap<String, (Value, Option<Box<Block>>)>,
    shadow: bool,
    pub top_level: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mutation: Option<Mutation>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            id: new_id(),
            opcode: String::new(),
            next: None,
            inputs: HashMap::new(),
            fields: HashMap::new(),
            shadow: false,
            top_level: false,
            x: None,
            y: None,
            mutation: None,
        }
    }
}

#[derive(Serialize)]
struct BlockInput {
    shadow_state: i32,
    input: PrimitiveOrID,
    shadow_value: Option<ID>,
}

#[derive(Serialize)]
enum PrimitiveOrID {
    ID(ID),
    Primitive(Primitive),
}

#[derive(Serialize)]
struct Mutation {
    tag_name: String,
    children: Vec<ID>,
    proccode: String,
    argumentids: String,
    warp: Option<bool>,
    hasnext: Option<bool>,
}
