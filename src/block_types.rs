use crate::block::*;
use crate::id::*;
use serde::ser::SerializeMap;

#[opcode("event_whenflagclicked")]
#[derive(SerializeNext)]
pub struct WhenFlagClicked {
    pub id: ID,
    pub next: Option<Box<Block>>,
}

impl WhenFlagClicked {
    pub fn new(next: Option<Box<Block>>) -> Self {
        Self { id: new_id(), next }
    }

    pub fn push_next<'a>(&'a self, stack: &mut BlockIterStack<'a>) {
        if let Some(next) = &self.next {
            stack.push((next, Some(&self.id)));
        }
    }
}

impl SerializableBlock for WhenFlagClicked {}

#[opcode("event_whenthisspriteclicked")]
#[derive(SerializeNext)]
pub struct WhenThisSpriteClicked {
    pub id: ID,
    pub next: Option<Box<Block>>,
}

impl WhenThisSpriteClicked {
    pub fn new(next: Option<Box<Block>>) -> Self {
        Self { id: new_id(), next }
    }

    pub fn push_next<'a>(&'a self, stack: &mut BlockIterStack<'a>) {
        if let Some(next) = &self.next {
            stack.push((next, Some(&self.id)));
        }
    }
}

impl SerializableBlock for WhenThisSpriteClicked {}
