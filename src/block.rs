use crate::id::*;
use crate::primitive::*;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Block {
    opcode: String,
    next: Option<ID>,
    parent: Option<ID>,
    inputs: HashMap<String, BlockInput>,
    fields: HashMap<String, (Value, Option<ID>)>,
    shadow: bool,
    top_level: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mutation: Option<Mutation>,
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
