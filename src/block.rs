use crate::id::*;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Block {
    opcode: String,
    next: Option<ID>,
    parent: Option<ID>,
    inputs: HashMap<String, (i32, PrimitiveOrID, Option<ID>)>,
    fields: HashMap<String, (Value, Option<ID>)>,
    shadow: bool,
    top_level: bool,
    x: Option<i32>,
    y: Option<i32>,
    mutation: Option<Mutation>,
}

#[derive(Serialize)]
enum PrimitiveOrID {
    ID(ID),
    Primitive(/*TODO*/),
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
